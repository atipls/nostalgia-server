use network::{peer::Peer, reliability::Reliability, NetworkError};
use protocol::interop::{EntityData, SyncedEntityData};
use protocol::{Packet, *};
use std::num::NonZeroU32;
use std::{io::Cursor, sync::Arc, time::Duration};
use tokio::sync::{mpsc::Sender, Mutex};
use entity::entity_flags;
use types::Vector3;
use world::World;

const MINECRAFT_TICKRATE: u64 = 100;
const MINECRAFT_TICKRATE_MS: f64 = 1000.0 / MINECRAFT_TICKRATE as f64;

pub struct Connection {
    peer: Peer,
    world: Arc<Mutex<World>>,
    global_packet_sender: Arc<Mutex<Sender<(Option<NonZeroU32>, Packet)>>>,
    client_id: Option<NonZeroU32>,
    connected: bool,

    position: Vector3,
    entity_counter: i32,
}

impl Connection {
    pub fn new(
        peer: Peer,
        world: Arc<Mutex<World>>,
        global_packet_sender: Arc<Mutex<Sender<(Option<NonZeroU32>, Packet)>>>,
    ) -> Self {
        Self {
            peer,
            world,
            global_packet_sender,
            client_id: None,
            connected: true,
            position: Vector3::default(),
            entity_counter: 2,
        }
    }

    pub async fn update(&mut self) -> network::Result<()> {
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
            Packet::LoginRequest(login_request) => self.handle_login_request(login_request).await?,
            Packet::Message(message) => self.broadcast_packet(false, message.clone()).await?,
            Packet::MovePlayer(move_player) => {
                self.broadcast_packet(true, move_player.clone()).await?;
                self.position = move_player.pos;
                let mut base_chicken_metadata = SyncedEntityData::from(&[
                    (1, EntityData::Short(11265)),
                    (14, EntityData::Byte(0)),
                    (0, EntityData::Byte(entity_flags::ON_FIRE as i8)),
                ]);

                self.send_packet(AddMob {
                    entity_id: self.entity_counter,
                    entity_type: 10,
                    pos: move_player.pos,
                    yaw: 199,
                    pitch: 0,
                    metadata: base_chicken_metadata,
                })
                .await?;
                self.entity_counter += 1;
            }
            Packet::Animate(animate) => {
                self.send_packet(Explode {
                    pos: self.position,
                    radius: 5.0,
                    count: 16,
                }).await?;
            }
            Packet::UseItem(use_item) => {
                self.send_packet(Message {
                    username: "Server".to_string(),
                    message: format!("Use item: {:?}", use_item),
                }).await?;
                self.send_packet(UpdateBlock {
                    // entity_id: 1,
                    x: use_item.x - 1,
                    z: use_item.z,
                    y: use_item.y as u8,
                    block: 3,
                    meta: 0,
                }).await?;
                self.send_packet(RemoveBlock {
                    entity_id: 1,
                    x: use_item.x + 1,
                    z: use_item.z,
                    y: use_item.y as u8,
                }).await?;
            }
            _ => {
                println!("Unhandled packet: {:?}", minecraft_packet);
            }
        }

        Ok(())
    }

    async fn handle_login_request(&mut self, login_request: LoginRequest) -> network::Result<()> {
        let world = self.world.clone().lock_owned().await;

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

        self.client_id = Some(NonZeroU32::new(login_request.client_id).unwrap());

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

    pub async fn broadcast_packet(
        &mut self,
        others_only: bool,
        packet: impl Into<Packet>,
    ) -> network::Result<()> {
        let global_packet_sender = self.global_packet_sender.lock().await;
        let exclude_client_id = if others_only { self.client_id } else { None };

        global_packet_sender
            .send((exclude_client_id, packet.into()))
            .await
            .map_err(|_| NetworkError::ConnectionClosed)
    }

    pub fn peer(&self) -> &Peer {
        &self.peer
    }

    pub fn client_id(&self) -> Option<NonZeroU32> {
        self.client_id
    }

    pub fn connected(&self) -> bool {
        self.connected
    }

    pub fn disconnect(&mut self) {
        self.connected = false;
    }
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.peer == other.peer
    }
}
