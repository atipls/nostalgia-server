mod connection;

use connection::Connection;
use network::{listener::Listener, protocol::ConnectedPacket, reliability::FrameVec, NetworkError};
use protocol::Packet;
use std::{
    arch::{aarch64, is_aarch64_feature_detected},
    num::NonZeroU32,
    path::PathBuf,
    sync::Arc,
};
use tokio::sync::{
    mpsc::{channel, Receiver, Sender},
    Mutex, Semaphore,
};
use world::World;

struct Application {
    listener: Listener,
    world: Arc<Mutex<World>>,
    connections: Arc<Mutex<Vec<Arc<Mutex<Connection>>>>>,
    global_packet_sender: Arc<Mutex<Sender<(Option<NonZeroU32>, Packet)>>>,
    global_packet_receiver: Arc<Mutex<Receiver<(Option<NonZeroU32>, Packet)>>>,
    disconnection_notifier: Arc<Semaphore>,
}

impl Application {
    pub fn new(listener: Listener, world: World) -> Self {
        let (global_packet_sender, global_packet_receiver) = channel(64);

        Self {
            listener,
            world: Arc::new(Mutex::new(world)),
            connections: Arc::new(Mutex::new(Vec::new())),
            global_packet_sender: Arc::new(Mutex::new(global_packet_sender)),
            global_packet_receiver: Arc::new(Mutex::new(global_packet_receiver)),
            disconnection_notifier: Arc::new(Semaphore::new(0)),
        }
    }

    pub async fn run(&mut self) -> Result<(), NetworkError> {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            let peer = self.listener.accept().await?;

            let world = self.world.clone();
            let global_packet_sender = self.global_packet_sender.clone();
            let connection = Arc::new(Mutex::new(Connection::new(
                peer,
                world,
                global_packet_sender,
            )));
            let mut connections = self.connections.lock().await;
            connections.push(connection.clone());

            let disconnection_notifier = self.disconnection_notifier.clone();
            tokio::spawn(async move {
                let disconnection_notifier = disconnection_notifier.clone();
                loop {
                    let mut connection = connection.lock().await;
                    match connection.update().await {
                        Ok(_) => {}
                        Err(NetworkError::ConnectionClosed) => {
                            connection.disconnect().await.expect("Failed to disconnect");
                            println!("Connection closed (disconnected)");
                            break;
                        }
                        Err(error) => {
                            connection.disconnect().await.expect("Failed to disconnect");
                            println!("Connection closed ({:#?})", error);
                            break;
                        }
                    }
                }

                disconnection_notifier.add_permits(1);
            });

            let global_packet_receiver = self.global_packet_receiver.clone();
            let connections = self.connections.clone();
            let disconnection_notifier = self.disconnection_notifier.clone();
            tokio::spawn(async move {
                loop {
                    let disconnection_notifier = disconnection_notifier.clone();
                    let mut global_packet_receiver = global_packet_receiver.lock().await;
                    tokio::select! {
                        packet = global_packet_receiver.recv() => {
                            if let Some((exclude, packet)) = packet {
                                let connections = connections.lock().await;
                                for connection in connections.iter() {
                                    let mut connection = connection.lock().await;
                                    if let Some(exclude) = exclude {
                                        if connection.client_id() == Some(exclude) {
                                            continue;
                                        }
                                    }
                                    connection.send_packet(packet.clone()).await.expect("Failed to send a global packet");
                                }
                            }
                        }
                        _disconnection_permit = disconnection_notifier.acquire() => {
                            let mut connections = connections.lock().await;
                            connections.retain(|connection| {
                                let Ok(connection) = connection.try_lock() else {
                                    return true; // We can't lock the connection, so we assume it's still in use
                                };

                                connection.connected()
                            });
                        }
                    }
                }
            });
        }
    }
}

#[tokio::main]
async fn main() {
    console_subscriber::init();
    let address = "0.0.0.0:19132".parse().expect("Address is already in use");
    let listener = Listener::started(&address, "Nostalgia Server".to_string())
        .await
        .expect("Failed to start the server");

    let world_path = PathBuf::from("assets/MainWorld");
    let world = World::from_file(world_path).expect("Failed to load the world");

    let packet_bytes: [u8; 87] = [
        0x84, 0xd0, 0x04, 0x00, 0x40, 0x00, 0x70, 0x8c, 0x03, 0x00, 0x97, 0x00, 0x00, 0x00, 0x02,
        0x00, 0x00, 0x00, 0x81, 0x00, 0x00, 0x00, 0x81, 0x40, 0x40, 0x01, 0x80, 0x8d, 0x03, 0x00,
        0xa3, 0x00, 0x00, 0x40, 0xef, 0xff, 0xff, 0xb9, 0x93, 0xff, 0xff, 0xab, 0x1d, 0x00, 0x00,
        0x00, 0xff, 0x00, 0x05, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x81, 0xe6, 0x48,
        0x2b, 0x90, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x01, 0xc8, 0x3d, 0xd8, 0x43, 0x00,
        0x00, 0x00, 0x04, 0x00, 0x00, 0x30, 0xac, 0x01, 0x00, 0x00, 0x00, 0x02,
    ];

    dump_wireshark_packets(&packet_bytes);

    let mut application = Application::new(listener, world);
    match application.run().await {
        Ok(_) => println!("Server closed"),
        Err(error) => println!("Server closed ({:#?})", error),
    }
}

fn dump_wireshark_packets(packet_bytes: &[u8]) {
    let frame_vec = FrameVec::new(Vec::from(packet_bytes)).unwrap();
    for frame in frame_vec.frames {
        let mut cursor = std::io::Cursor::new(frame.data.clone());
        match ConnectedPacket::parse(&mut cursor).unwrap() {
            Some(connected_packet) => {
                println!("Parsed connected packet {:?}", connected_packet);
            }
            None => {
                let mut payload_cursor = std::io::Cursor::new(frame.data.clone());
                while let Ok(Some(packet)) = Packet::parse(&mut payload_cursor) {
                    println!("Parsed packet {:?}", packet);
                }
            }
        };
    }
}
