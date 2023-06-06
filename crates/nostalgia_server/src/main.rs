mod connection;

use network::{listener::Listener, reliability::Reliability};
use protocol::Packet;
use std::io::Cursor;
use types::Vector3;

fn get_packet_bytes(packet: Packet) -> Vec<u8> {
    let mut cursor = Cursor::new(Vec::new());
    packet
        .serialize(&mut cursor)
        .expect("Failed to serialize the packet");
    cursor.get_ref().clone()
}

#[tokio::main]
async fn main() {
    let address = "0.0.0.0:19132".parse().expect("Address is already in use");
    let mut listener = Listener::started(&address, "Nostalgia Server".to_string())
        .await
        .expect("Failed to start the server");

    loop {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let peer = listener.accept().await.expect("Failed to accept a peer");
        loop {
            let packet = peer.recv().await.expect("Failed to receive a packet");
            let mut cursor = Cursor::new(packet);

            let Some(minecraft_packet) =
                Packet::parse(&mut cursor).expect("Failed to parse the packet") else {
                    continue;
                };

            match minecraft_packet {
                Packet::LoginRequest(_login_request) => {
                    let login_response =
                        Packet::LoginResponse(protocol::LoginResponse { status: 0 });

                    peer.send(
                        get_packet_bytes(login_response).as_slice(),
                        Reliability::Reliable,
                    )
                    .await
                    .expect("Failed to send a packet");

                    let start_game = Packet::StartGame(protocol::StartGame {
                        world_seed: 0,
                        generator_version: 0,
                        gamemode: 0,
                        entity_id: 0,
                        position: Vector3 {
                            x: 128.0,
                            y: 72.0,
                            z: 128.0,
                        },
                    });

                    peer.send(
                        get_packet_bytes(start_game).as_slice(),
                        Reliability::Reliable,
                    )
                    .await
                    .expect("Failed to send a packet");
                }
                _ => {
                    println!("Unhandled packet: {:?}", minecraft_packet);
                }
            }
        }
    }
}
