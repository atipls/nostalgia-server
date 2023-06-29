mod connection;

use connection::Connection;
use network::NetworkError;
use network::{listener::Listener, reliability::Reliability};
use protocol::Packet;
use std::io::Cursor;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::Sender;
use types::Vector3;

fn get_packet_bytes(packet: Packet) -> Vec<u8> {
    let mut cursor = Cursor::new(Vec::new());
    packet
        .serialize(&mut cursor)
        .expect("Failed to serialize the packet");
    cursor.get_ref().clone()
}

struct Application {
    listener: Listener,
    connections: Vec<Connection>,
    disconnection_sender: Sender<Connection>,
    disconnection_receiver: Arc<Mutex<Receiver<Connection>>>,
}

const MINECRAFT_TICKRATE: u64 = 100;
const MINECRAFT_TICKRATE_MS: f64 = 1000.0 / MINECRAFT_TICKRATE as f64;

#[tokio::main]
async fn main() {
    let address = "0.0.0.0:19132".parse().expect("Address is already in use");
    let mut listener = Listener::started(&address, "Nostalgia Server".to_string())
        .await
        .expect("Failed to start the server");

    loop {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let peer = listener.accept().await.expect("Failed to accept a peer");
        let mut connection = Connection::new(peer);

        tokio::spawn(async move {
            match connection.run_worker().await {
                Ok(_) => println!("Connection closed"),
                Err(NetworkError::ConnectionClosed) => println!("Connection closed (disconnected)"),
                Err(_) => println!("Connection closed (error)"),
            }
        });

        /*
                let connection = Connection::new(peer);
                tokio::spawn(async move {
                    loop {
                        match connection.run_worker().await {
                            Ok(_) => break,
                            Err(_) => disconnect_sender
                                .lock()
                                .await
                                .send(connection.address)
                                .expect("Failed to send a disconnection"),
                        }
                    }
                });
        */
    }
}
