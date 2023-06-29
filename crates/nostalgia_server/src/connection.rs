use std::io::Cursor;

use network::{peer::Peer, reliability::Reliability};
use protocol::{Packet, *};
use types::Vector3;

pub struct Connection {
    peer: Peer,
}

impl Connection {
    pub fn new(peer: Peer) -> Self {
        Self { peer }
    }

    pub async fn run_worker(&mut self) -> network::Result<()> {
        loop {
            let packet = self.peer.receive().await?;
            let mut cursor = Cursor::new(packet);

            let Some(minecraft_packet) = Packet::parse(&mut cursor)? else {
                continue;
            };

            match minecraft_packet {
                Packet::LoginRequest(_login_request) => {
                    self.send_packet(LoginResponse { status: 0 })
                        .await
                        .expect("Failed to send the login response");

                    self.send_packet(StartGame {
                        world_seed: 0,
                        generator_version: 0,
                        gamemode: 0,
                        entity_id: 0,
                        position: Vector3 {
                            x: 128.0,
                            y: 72.0,
                            z: 128.0,
                        },
                    })
                    .await
                    .expect("Failed to send the start game packet");
                }
                _ => {
                    println!("Unhandled packet: {:?}", minecraft_packet);
                }
            }
        }
    }

    pub async fn send_packet(&mut self, packet: impl Into<Packet>) -> network::Result<()> {
        let mut cursor = Cursor::new(Vec::new());
        let packet = packet.into();
        packet.serialize(&mut cursor)?;

        self.peer
            .send(cursor.get_ref(), Reliability::Reliable)
            .await?;

        Ok(())
    }
}
