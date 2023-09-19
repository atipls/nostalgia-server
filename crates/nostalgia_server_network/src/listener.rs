use crate::{peer::Peer, protocol::*, NetworkError, Result};
use std::{collections::HashMap, io::Cursor, net::SocketAddr, sync::Arc};
use tokio::{
    net::UdpSocket,
    sync::{
        mpsc::{self, Receiver, Sender},
        Mutex, Notify, Semaphore,
    },
};

type BoundSession = (i64, Sender<Vec<u8>>);
type SessionTable = HashMap<SocketAddr, BoundSession>;

pub struct Listener {
    socket: Arc<UdpSocket>,

    new_connection_receiver: Receiver<Peer>,
    new_connection_sender: Sender<Peer>,
    is_listening: bool,

    sessions: Arc<Mutex<SessionTable>>,

    should_close_notifier: Arc<Semaphore>,
    done_closing_notifier: Arc<Notify>,
    kill_notifier: Arc<Notify>,

    guid: u64,
    name: String,
}

impl Listener {
    pub async fn new(address: &SocketAddr, name: String) -> Result<Self> {
        let socket = UdpSocket::bind(address).await?;
        let (sender, receiver) = mpsc::channel(16);

        Ok(Self {
            socket: Arc::new(socket),
            new_connection_sender: sender,
            new_connection_receiver: receiver,
            is_listening: false,
            sessions: Arc::new(Mutex::new(HashMap::new())),
            should_close_notifier: Arc::new(Semaphore::new(0)),
            done_closing_notifier: Arc::new(Notify::new()),
            kill_notifier: Arc::new(Notify::new()),
            guid: rand::random(),
            name: format!("MCCPP;Demo;{}", name),
        })
    }

    pub async fn started(address: &SocketAddr, name: String) -> Result<Self> {
        match Self::new(address, name).await {
            Ok(mut listener) => {
                listener.start().await;
                Ok(listener)
            }
            Err(error) => Err(error),
        }
    }

    async fn start_session_reaper(&mut self, mut disconnect_receiver: Receiver<SocketAddr>) {
        let socket = self.socket.clone();
        let sessions = self.sessions.clone();

        let should_close_notifier = self.should_close_notifier.clone();
        let done_closing_notifier = self.done_closing_notifier.clone();

        tokio::spawn(async move {
            loop {
                let address = tokio::select! {
                    address = disconnect_receiver.recv() => match address {
                        Some(address) => address,
                        None => break,
                    },
                    _ = should_close_notifier.acquire() => break,
                };

                let mut sessions = sessions.lock().await;
                if sessions.contains_key(&address) {
                    _ = socket.send_to(&[0x15, 0x00], address).await;
                    sessions.remove(&address);
                }
            }

            let mut sessions = sessions.lock().await;
            for (_, session) in sessions.iter() {
                let (_, session_sender) = session;
                _ = session_sender.send(vec![0x15, 0x00]).await;
            }

            while !sessions.is_empty() {
                let address = match disconnect_receiver.recv().await {
                    Some(address) => address,
                    None => break,
                };

                if sessions.contains_key(&address) {
                    _ = socket.send_to(&[0x15, 0x00], address).await;
                    sessions.remove(&address);
                }
            }

            done_closing_notifier.notify_waiters();
        });
    }

    async fn start_worker(&mut self, disconnect_sender: Arc<Mutex<Sender<SocketAddr>>>) {
        let socket = self.socket.clone();
        let sessions = self.sessions.clone();

        let guid = self.guid;
        let name = self.name.clone();

        let new_connection_sender = self.new_connection_sender.clone();

        let should_close_notifier = self.should_close_notifier.clone();

        tokio::spawn(async move {
            let mut buffer = [0u8; 2048];

            loop {
                let (address, length) = tokio::select! {
                    received = socket.recv_from(&mut buffer) => match received {
                        Ok((length, address)) => (address, length),
                        Err(_) => break,
                    },
                    _ = should_close_notifier.acquire() => break,
                };

                {
                    let mut sessions = sessions.lock().await;
                    if let Some((last_seen_time, sender)) = sessions.get_mut(&address) {
                        match sender.send(buffer[..length].to_vec()).await {
                            Ok(_) => *last_seen_time = 0,
                            Err(_) => _ = sessions.remove(&address),
                        };

                        continue;
                    }
                }

                let buffer = buffer[..length].to_vec();
                let mut cursor = Cursor::new(buffer);

                let Ok(packet) = UnconnectedPacket::parse(&mut cursor) else {
                    continue;
                };
                let Some(packet) = packet else { continue };

                println!("Got packet: {:?}", packet);

                async fn send_unconnected(
                    socket: &UdpSocket,
                    address: &SocketAddr,
                    packet: UnconnectedPacket,
                ) -> Result<usize> {
                    let mut cursor = Cursor::new(Vec::new());
                    packet.serialize(&mut cursor)?;

                    Ok(socket.send_to(&cursor.get_ref(), address).await?)
                }

                match packet {
                    UnconnectedPacket::Ping { timestamp: _ } => {
                        match send_unconnected(
                            &socket,
                            &address,
                            UnconnectedPacket::Pong {
                                timestamp: 0,
                                server_guid: guid,
                                motd: name.clone(),
                            },
                        )
                        .await
                        {
                            Ok(_) => (),
                            Err(_) => (),
                        }
                    }
                    UnconnectedPacket::ConnectionRequest {
                        protocol_version: _,
                    } => {
                        match send_unconnected(
                            &socket,
                            &address,
                            UnconnectedPacket::ConnectionReply {
                                server_guid: guid,
                                mtu_size: 1492,
                                use_encryption: false,
                            },
                        )
                        .await
                        {
                            Ok(_) => (),
                            Err(_) => (),
                        }
                    }
                    UnconnectedPacket::ConnectionEstablish {
                        server_address: _,
                        client_guid: _,
                        mtu_size: _,
                    } => {
                        if let Ok(_) = send_unconnected(
                            &socket,
                            &address,
                            UnconnectedPacket::ConnectionEstablished {
                                address: address.clone(),
                                server_guid: guid,
                                mtu_size: 1492,
                                use_encryption: false,
                            },
                        )
                        .await
                        {
                            let (data_sender, data_receiver) = mpsc::channel(16);
                            let peer = Peer::new(
                                &address,
                                &socket,
                                data_receiver,
                                disconnect_sender.clone(),
                                1492,
                            )
                            .await;

                            let mut sessions = sessions.lock().await;
                            sessions.insert(address.clone(), (0, data_sender.clone()));

                            _ = new_connection_sender.send(peer).await;
                        }
                    }
                    _ => (),
                }
            }
        });
    }

    pub async fn start(&mut self) {
        let (disconnect_sender, disconnect_receiver) = mpsc::channel(16);
        let disconnect_sender = Arc::new(Mutex::new(disconnect_sender));

        self.start_session_reaper(disconnect_receiver).await;
        self.start_worker(disconnect_sender).await;
    }

    pub async fn accept(&mut self) -> Result<Peer> {
        tokio::select! {
            peer = self.new_connection_receiver.recv() => match peer {
                Some(peer) => Ok(peer),
                None => Err(NetworkError::NotListening),
            },
            _ = self.should_close_notifier.acquire() => Err(NetworkError::NotListening),
        }
    }
}
