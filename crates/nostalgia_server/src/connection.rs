use network::{peer::Peer, reliability::Reliability, NetworkError};
use protocol::{Packet, *};
use std::{io::Cursor, sync::Arc, time::Duration};
use tokio::sync::{mpsc::Sender, Mutex};
use types::Vector3;
use world::World;

const MINECRAFT_TICKRATE: u64 = 100;
const MINECRAFT_TICKRATE_MS: f64 = 1000.0 / MINECRAFT_TICKRATE as f64;

pub struct Connection {
    peer: Peer,
}

impl Connection {
    pub fn new(peer: Peer) -> Self {
        Self { peer }
    }

    pub async fn update(
        &mut self,
        global_packet_sender: Arc<Mutex<Sender<Packet>>>,
        world: Arc<Mutex<World>>,
    ) -> network::Result<()> {
        let timeout = Duration::from_millis(MINECRAFT_TICKRATE_MS as u64);
        let packet = match self.peer.receive(timeout).await {
            Ok(packet) => Ok(packet),
            Err(NetworkError::ReceiveTimeout) => return Ok(()),
            error => error,
        }?;

        let mut cursor = Cursor::new(packet);

        let Some(minecraft_packet) = Packet::parse(&mut cursor)? else {
                return Err(NetworkError::InvalidPacketHeader);
            };

        match minecraft_packet {
            Packet::LoginRequest(login_request) => {
                let world = world.lock().await;

                if login_request.protocol_major != login_request.protocol_major
                    || login_request.protocol_minor != 14
                {
                    self.send_packet(LoginResponse { status: 1 }).await?;
                    return Ok(());
                }

                self.send_packet(LoginResponse { status: 0 }).await?;
                self.send_packet(StartGame {
                    world_seed: world.seed as i32,
                    generator_version: 0,
                    gamemode: world.game_type,
                    entity_id: 1,
                    position: Vector3 {
                        x: world.spawn_position.0 as f32 + 0.5,
                        y: world.spawn_position.1 as f32 + 1.6,
                        z: world.spawn_position.2 as f32 + 0.5,
                    },
                })
                .await?;
            }
            Packet::Message(message) => {
                let global_packet_sender = global_packet_sender.lock().await;
                global_packet_sender
                    .send(message.clone().into())
                    .await
                    .map_err(|_| NetworkError::ConnectionClosed)?;
            }
            Packet::MovePlayer(move_player) => {
                let global_packet_sender = global_packet_sender.lock().await;
                global_packet_sender
                    .send(move_player.clone().into())
                    .await
                    .map_err(|_| NetworkError::ConnectionClosed)?;
            }
            _ => {
                println!("Unhandled packet: {:?}", minecraft_packet);
            }
        }

        Ok(())
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

    pub fn peer(&self) -> &Peer {
        &self.peer
    }
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.peer == other.peer
    }
}
