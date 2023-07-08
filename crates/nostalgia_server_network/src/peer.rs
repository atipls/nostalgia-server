use crate::current_timestamp_milliseconds;
use crate::protocol::{ConnectedPacket, UnconnectedPacket};
use crate::reliability::FrameVec;
use crate::{
    reliability::{Frame, RecvQueue, Reliability, SendQueue},
    {NetworkError, Result},
};
use std::io::Cursor;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use std::{net::SocketAddr, sync::Arc};
use tokio::{
    net::UdpSocket,
    sync::{
        mpsc::{self, Receiver, Sender},
        {Mutex, Notify, RwLock, Semaphore},
    },
    time::sleep,
};

const RECEIVE_TIMEOUT: u64 = 10000; // 10 seconds

#[derive(Debug)]
pub struct Peer {
    local_addr: SocketAddr,
    peer_addr: SocketAddr,

    user_data_receiver: Arc<Mutex<Receiver<Vec<u8>>>>,
    send_queue: Arc<RwLock<SendQueue>>,
    recv_queue: Arc<Mutex<RecvQueue>>,
    last_heartbeat_time: Arc<AtomicU64>,
    incomming_notifier: Arc<Notify>,
    close_notifier: Arc<Semaphore>,

    sender: Sender<(Vec<u8>, SocketAddr)>,
}

impl Peer {
    pub async fn new(
        address: &SocketAddr,
        socket: &Arc<UdpSocket>,
        data_receiver: Receiver<Vec<u8>>,
        reaper: Arc<Mutex<Sender<SocketAddr>>>,
        mtu: u16,
    ) -> Self {
        let (user_data_sender, user_data_receiver) = mpsc::channel(16);
        let (sender, receiver) = mpsc::channel(16);

        let peer = Self {
            local_addr: socket.local_addr().unwrap(),
            peer_addr: *address,
            user_data_receiver: Arc::new(Mutex::new(user_data_receiver)),
            send_queue: Arc::new(RwLock::new(SendQueue::new(mtu))),
            recv_queue: Arc::new(Mutex::new(RecvQueue::new())),
            last_heartbeat_time: Arc::new(AtomicU64::new(0)),
            incomming_notifier: Arc::new(Notify::new()),
            close_notifier: Arc::new(Semaphore::new(0)),
            sender,
        };

        peer.start_receiver(socket, data_receiver, user_data_sender);
        peer.start_tick(socket, Some(reaper));
        peer.start_sender(socket, receiver);

        peer
    }

    async fn handle(
        frame: &Frame,
        peer_address: &SocketAddr,
        local_address: &SocketAddr,
        sendq: &RwLock<SendQueue>,
        user_data_sender: &Sender<Vec<u8>>,
        incomming_notify: &Notify,
    ) -> Result<bool> {
        let mut cursor = Cursor::new(frame.data.clone());
        let Some(connected_packet) = ConnectedPacket::parse(&mut cursor)? else {
            match user_data_sender.send(frame.data.clone()).await {
                Ok(_) => return Ok(true),
                Err(_) => return Ok(false),
            };
        };

        print!("Got connected packet: {:?}\n", connected_packet);

        match connected_packet {
            ConnectedPacket::ConnectionRequest {
                client_guid,
                timestamp,
                use_encryption,
            } => {
                Self::send_packet(
                    sendq,
                    ConnectedPacket::ConnectionRequestAccepted {
                        client_address: *peer_address,
                        system_index: 0,
                        request_timestamp: timestamp,
                        accepted_timestamp: current_timestamp_milliseconds(),
                    },
                    Reliability::ReliableOrdered,
                )
                .await?;

                Ok(true)
            }
            ConnectedPacket::ConnectionRequestAccepted {
                client_address,
                system_index,
                request_timestamp,
                accepted_timestamp,
            } => {
                Self::send_packet(
                    sendq,
                    ConnectedPacket::NewIncomingConnection {
                        server_address: *local_address,
                        request_timestamp,
                        accepted_timestamp: current_timestamp_milliseconds(),
                    },
                    Reliability::ReliableOrdered,
                )
                .await?;

                incomming_notify.notify_one();
                Ok(true)
            }
            ConnectedPacket::NewIncomingConnection {
                server_address,
                request_timestamp,
                accepted_timestamp,
            } => Ok(true),
            ConnectedPacket::Ping { timestamp } => {
                Self::send_packet(
                    sendq,
                    ConnectedPacket::Pong {
                        timestamp,
                        server_time: current_timestamp_milliseconds(),
                    },
                    Reliability::Reliable,
                )
                .await?;

                Ok(true)
            }
            ConnectedPacket::Pong {
                timestamp,
                server_time,
            } => Ok(true),
            _ => Ok(false),
        }
    }

    async fn sendto(s: &UdpSocket, buf: &[u8], target: &SocketAddr) -> tokio::io::Result<usize> {
        match s.send_to(buf, target).await {
            Ok(p) => Ok(p),
            Err(e) => Ok(0),
        }
    }

    /*
       pub async fn connect(addr: &SocketAddr, raknet_version: u8) -> Result<Self> {
           let guid: u64 = rand::random();

           let s = match UdpSocket::bind("0.0.0.0:0").await {
               Ok(p) => p,
               Err(_) => return Err(RaknetError::BindAdressError),
           };

           let packet = OpenConnectionRequest1 {
               magic: true,
               protocol_version: raknet_version,
               mtu_size: RAKNET_CLIENT_MTU,
           };

           let buf = write_packet_connection_open_request_1(&packet).unwrap();

           let mut remote_addr: SocketAddr;
           let mut reply1_size: usize;

           let mut reply1_buf = [0u8; 2048];

           loop {
               match s.send_to(&buf, addr).await {
                   Ok(p) => p,
                   Err(e) => {
                       tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                       continue;
                   }
               };
               let (size, src) = match match timeout(
                   std::time::Duration::from_secs(2),
                   s.recv_from(&mut reply1_buf),
               )
               .await
               {
                   Ok(p) => p,
                   Err(_) => {
                       continue;
                   }
               } {
                   Ok(p) => p,
                   Err(e) => {
                       continue;
                   }
               };

               remote_addr = src;
               reply1_size = size;

               if reply1_buf[0] != PacketID::OpenConnectionReply1.to_u8() {
                   if reply1_buf[0] == PacketID::IncompatibleProtocolVersion.to_u8() {
                       let _packet = match read_packet_incompatible_protocol_version(&buf[..size]) {
                           Ok(p) => p,
                           Err(_) => return Err(RaknetError::NotSupportVersion),
                       };

                       return Err(RaknetError::NotSupportVersion);
                   } else {
                       continue;
                   }
               }

               break;
           }

           let reply1 = match read_packet_connection_open_reply_1(&reply1_buf[..reply1_size]) {
               Ok(p) => p,
               Err(_) => return Err(RaknetError::PacketParseError),
           };

           let packet = OpenConnectionRequest2 {
               magic: true,
               address: remote_addr,
               mtu: reply1.mtu_size,
               guid,
           };

           let buf = write_packet_connection_open_request_2(&packet).unwrap();

           loop {
               match s.send_to(&buf, addr).await {
                   Ok(_) => {}
                   Err(e) => {
                       tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                       continue;
                   }
               };

               let mut buf = [0u8; 2048];
               let (size, _) =
                   match match timeout(std::time::Duration::from_secs(2), s.recv_from(&mut buf)).await
                   {
                       Ok(p) => p,
                       Err(_) => {
                           continue;
                       }
                   } {
                       Ok(p) => p,
                       Err(e) => {
                           continue;
                       }
                   };

               if buf[0] == PacketID::OpenConnectionReply1.to_u8() {
                   continue;
               }

               if buf[0] != PacketID::OpenConnectionReply2.to_u8() {
                   continue;
               }

               let _reply2 = match read_packet_connection_open_reply_2(&buf[..size]) {
                   Ok(p) => p,
                   Err(_) => return Err(RaknetError::PacketParseError),
               };

               break;
           }

           let sendq = Arc::new(RwLock::new(SendQueue::new(reply1.mtu_size)));

           let packet = ConnectionRequest {
               guid,
               time: cur_timestamp_millis(),
               use_encryption: 0x00,
           };

           let buf = write_packet_connection_request(&packet).unwrap();

           let mut sendq1 = sendq.write().await;
           sendq1.insert(Reliability::ReliableOrdered, &buf)?;
           std::mem::drop(sendq1);

           let (user_data_sender, user_data_receiver) = channel::<Vec<u8>>(100);
           let (sender, receiver) = channel::<Vec<u8>>(100);

           let s = Arc::new(s);

           let recv_s = s.clone();
           let connected = Arc::new(tokio::sync::Semaphore::new(0));
           let connected_s = connected.clone();
           let peer_addr = *addr;
           tokio::spawn(async move {
               let mut buf = [0u8; 2048];
               loop {
                   if connected_s.is_closed() {
                       break;
                   }
                   let (size, _) = match match timeout(
                       std::time::Duration::from_secs(10),
                       recv_s.recv_from(&mut buf),
                   )
                   .await
                   {
                       Ok(p) => p,
                       Err(_) => continue,
                   } {
                       Ok(p) => p,
                       Err(e) => {
                           connected_s.close();
                           break;
                       }
                   };

                   match sender.send(buf[..size].to_vec()).await {
                       Ok(_) => {}
                       Err(e) => {
                           connected_s.close();
                           break;
                       }
                   };
               }
           });

           let (sender_sender, sender_receiver) = channel::<(Vec<u8>, SocketAddr, bool, u8)>(10);

           let ret = RaknetSocket {
               peer_addr: *addr,
               local_addr: s.local_addr().unwrap(),
               user_data_receiver: Arc::new(Mutex::new(user_data_receiver)),
               recvq: Arc::new(Mutex::new(RecvQueue::new())),
               sendq,
               close_notifier: connected,
               last_heartbeat_time: Arc::new(AtomicI64::new(cur_timestamp_millis())),
               incomming_notifier: Arc::new(Notify::new()),
               sender: sender_sender,
               drop_notifier: Arc::new(Notify::new()),
               raknet_version,
           };

           ret.start_receiver(&s, receiver, user_data_sender);
           ret.start_tick(&s, None);
           ret.start_sender(&s, sender_receiver);
           ret.drop_watcher().await;

           ret.incomming_notifier.notified().await;

           Ok(ret)
       }
    */

    fn start_receiver(
        &self,
        s: &Arc<UdpSocket>,
        mut receiver: Receiver<Vec<u8>>,
        user_data_sender: Sender<Vec<u8>>,
    ) {
        let connected = self.close_notifier.clone();
        let peer_addr = self.peer_addr;
        let local_addr = self.local_addr;
        let sendq = self.send_queue.clone();
        let recvq = self.recv_queue.clone();
        let last_heartbeat_time = self.last_heartbeat_time.clone();
        let incomming_notify = self.incomming_notifier.clone();
        let s = s.clone();
        tokio::spawn(async move {
            loop {
                if connected.is_closed() {
                    let mut recvq = recvq.lock().await;
                    for f in recvq.flush(&peer_addr) {
                        Peer::handle(
                            &f,
                            &peer_addr,
                            &local_addr,
                            &sendq,
                            &user_data_sender,
                            &incomming_notify,
                        )
                        .await
                        .unwrap();
                    }
                    break;
                }

                let buffer = match receiver.recv().await {
                    Some(buf) => buf,
                    None => {
                        connected.close();
                        break;
                    }
                };

                last_heartbeat_time.store(current_timestamp_milliseconds(), Ordering::Relaxed);

                let mut cursor = Cursor::new(buffer.clone());
                match ConnectedPacket::parse(&mut cursor).unwrap() {
                    Some(ConnectedPacket::Acknowledge { length, records }) => {
                        let mut sendq = sendq.write().await;
                        for i in 0..length {
                            if records[i as usize].0 == records[i as usize].1 {
                                sendq.ack(records[i as usize].0, current_timestamp_milliseconds());
                            } else {
                                for i in records[i as usize].0..records[i as usize].1 + 1 {
                                    sendq.ack(i, current_timestamp_milliseconds());
                                }
                            }
                        }
                        continue;
                    }
                    Some(ConnectedPacket::NegativeAcknowledge { length, records }) => {
                        let mut sendq = sendq.write().await;
                        for i in 0..length {
                            if records[i as usize].0 == records[i as usize].1 {
                                sendq.nack(records[i as usize].0, current_timestamp_milliseconds());
                            } else {
                                for i in records[i as usize].0..records[i as usize].1 + 1 {
                                    sendq.nack(i, current_timestamp_milliseconds());
                                }
                            }
                        }
                        continue;
                    }
                    Some(ConnectedPacket::DisconnectionNotification) => {
                        connected.close();
                        break;
                    }
                    Some(_) | None if ConnectedPacket::is_frame_packet(buffer[0]) => {
                        let frames = FrameVec::new(buffer.clone()).unwrap();

                        let mut recvq = recvq.lock().await;
                        let mut is_break = false;
                        for frame in frames.frames {
                            recvq.insert(frame).unwrap();

                            for f in recvq.flush(&peer_addr) {
                                if !Peer::handle(
                                    &f,
                                    &peer_addr,
                                    &local_addr,
                                    &sendq,
                                    &user_data_sender,
                                    &incomming_notify,
                                )
                                .await
                                .unwrap()
                                {
                                    connected.close();
                                    is_break = true;
                                };
                            }

                            if is_break {
                                break;
                            }
                        }

                        //flush ack
                        let acks = recvq.get_ack();
                        if !acks.is_empty() {
                            let ack_packet = ConnectedPacket::Acknowledge {
                                length: acks.len() as u16,
                                records: acks,
                            };

                            let mut cursor = Cursor::new(Vec::new());
                            ack_packet.serialize(&mut cursor).unwrap();

                            Peer::sendto(&s, cursor.get_ref(), &peer_addr)
                                .await
                                .unwrap();
                        }
                    }
                    Some(_) | None => {
                        println!("unknown packet");
                    }
                }
            }
        });
    }

    fn start_sender(&self, socket: &Arc<UdpSocket>, mut receiver: Receiver<(Vec<u8>, SocketAddr)>) {
        let close_notifier = self.close_notifier.clone();
        let socket = socket.clone();
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    a = receiver.recv() => {
                        match a {
                            Some(p) => {
                                match Peer::sendto(&socket, &p.0, &p.1).await {
                                    Ok(_) => {},
                                    Err(_) => break,
                                }
                            },
                            None => break,
                        };
                    },
                    _ = close_notifier.acquire() => break,
                }
            }
        });
    }

    fn start_tick(&self, socket: &Arc<UdpSocket>, reaper: Option<Arc<Mutex<Sender<SocketAddr>>>>) {
        let connected = self.close_notifier.clone();
        let s = socket.clone();
        let peer_addr = self.peer_addr;
        let sendq = self.send_queue.clone();
        let recvq = self.recv_queue.clone();
        let mut last_monitor_tick = current_timestamp_milliseconds();
        let last_heartbeat_time = self.last_heartbeat_time.clone();
        tokio::spawn(async move {
            loop {
                sleep(std::time::Duration::from_millis(
                    SendQueue::DEFAULT_TIMEOUT_MILLS as u64,
                ))
                .await;

                // flush nack
                let mut recvq = recvq.lock().await;
                let nacks = recvq.get_nack();
                if !nacks.is_empty() {
                    let nack_packet = ConnectedPacket::NegativeAcknowledge {
                        length: nacks.len() as u16,
                        records: nacks,
                    };

                    let mut cursor = Cursor::new(Vec::new());
                    nack_packet.serialize(&mut cursor).unwrap();

                    Peer::sendto(&s, cursor.get_ref(), &peer_addr)
                        .await
                        .unwrap();
                }

                //flush sendq
                let mut sendq = sendq.write().await;
                for f in sendq.flush(current_timestamp_milliseconds(), &peer_addr) {
                    let mut cursor = Cursor::new(Vec::new());
                    f.serialize(&mut cursor).unwrap();
                    Peer::sendto(&s, cursor.get_ref(), &peer_addr)
                        .await
                        .unwrap();
                }

                if current_timestamp_milliseconds() - last_heartbeat_time.load(Ordering::Relaxed)
                    > RECEIVE_TIMEOUT
                {
                    connected.close();
                    break;
                }

                if connected.is_closed() {
                    for _ in 0..10 {
                        Peer::sendto(&s, &[0x15, 0x00], &peer_addr).await.unwrap();
                    }
                    break;
                }
            }

            match reaper {
                Some(p) => {
                    match p.lock().await.send(peer_addr).await {
                        Ok(_) => {}
                        Err(e) => {}
                    };
                }
                None => {}
            }
        });
    }

    pub async fn close(&self) -> Result<()> {
        if !self.close_notifier.is_closed() {
            let mut send_queue = self.send_queue.write().await;
            send_queue.insert(Reliability::Reliable, &[0x15, 0x00])?;
            self.close_notifier.close();
        }
        Ok(())
    }

    /*
    pub async fn ping(addr: &SocketAddr) -> Result<(i64, String)> {
        let s = match UdpSocket::bind("0.0.0.0:0").await {
            Ok(p) => p,
            Err(_) => return Err(NetworkError::BindError),
        };

        loop {
            let packet = UnconnectedPacket::Ping {
                timestamp: current_timestamp_milliseconds(),
            };

            let buf = write_packet_ping(&packet)?;

            match s.send_to(buf.as_slice(), addr).await {
                Ok(_) => {}
                Err(e) => return Err(NetworkError::SocketError),
            };

            let mut buf = [0u8; 1024];

            match match tokio::time::timeout(
                std::time::Duration::from_secs(5),
                s.recv_from(&mut buf),
            )
            .await
            {
                Ok(p) => p,
                Err(_) => {
                    continue;
                }
            } {
                Ok(p) => p,
                Err(_) => return Err(NetworkError::SocketError),
            };

            if let Ok(p) = read_packet_pong(&buf) {
                return Ok((p.time - packet.time, p.motd));
            };

            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }
        }*/

    pub async fn send(&self, buf: &[u8], r: Reliability) -> Result<()> {
        if buf.is_empty() {
            return Err(NetworkError::InvalidPacketHeader);
        }

        if self.close_notifier.is_closed() {
            return Err(NetworkError::ConnectionClosed);
        }

        let mut sendq = self.send_queue.write().await;
        sendq.insert(r, buf)?;
        let sender = self.sender.clone();
        for f in sendq.flush(current_timestamp_milliseconds(), &self.peer_addr) {
            let mut cursor = Cursor::new(Vec::new());
            f.serialize(&mut cursor).unwrap();
            sender
                .send((cursor.get_ref().clone(), self.peer_addr))
                .await
                .unwrap();
        }
        Ok(())
    }

    pub async fn send_packet(
        send_queue: &RwLock<SendQueue>,
        packet: ConnectedPacket,
        reliability: Reliability,
    ) -> Result<()> {
        let mut send_queue = send_queue.write().await;
        let mut cursor = Cursor::new(Vec::new());

        packet.serialize(&mut cursor)?;

        send_queue.insert(reliability, cursor.get_ref())?;

        Ok(())
    }

    pub async fn flush(&self) -> Result<()> {
        loop {
            if self.close_notifier.is_closed() {
                return Err(NetworkError::ConnectionClosed);
            }
            let sendq = self.send_queue.read().await;
            if sendq.is_empty() {
                return Ok(());
            }
            tokio::time::sleep(std::time::Duration::from_millis(5)).await;
        }
    }

    pub async fn receive(&self, timeout: Duration) -> Result<Vec<u8>> {
        let mut user_data_receiver = self.user_data_receiver.lock().await;
        tokio::select! {
            user_data = user_data_receiver.recv() => {
                match user_data {
                    Some(p) => Ok(p),
                    None => Err(if self.close_notifier.is_closed() {
                        NetworkError::ConnectionClosed
                    } else {
                        NetworkError::SocketError
                    }),
                }
            }
            _ = tokio::time::sleep(timeout) => {
                Err(NetworkError::ReceiveTimeout)
            }
        }
    }
}

impl PartialEq for Peer {
    fn eq(&self, other: &Self) -> bool {
        self.peer_addr == other.peer_addr
    }
}
