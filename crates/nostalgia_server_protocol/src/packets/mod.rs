pub mod add_entity;
pub mod add_painting;
pub mod add_player;
pub mod adventure_settings;
pub mod animate;
pub mod chat;
pub mod container_ack;
pub mod container_close;
pub mod container_open;
pub mod container_set_data;
pub mod entity_event;
pub mod explode;
pub mod hurt_armor;
pub mod interact;
pub mod level_event;
pub mod login_request;
pub mod login_response;
pub mod message;
pub mod move_entity;
pub mod move_entity_pos_rot;
pub mod move_player;
pub mod place_block;
pub mod player_action;
pub mod player_armor_equipment;
pub mod player_equipment;
pub mod ready;
pub mod remove_block;
pub mod remove_entity;
pub mod remove_player;
pub mod request_chunk;
pub mod respawn;
pub mod rotate_head;
pub mod set_entity_motion;
pub mod set_health;
pub mod set_riding;
pub mod set_spawn_position;
pub mod set_time;
pub mod sign_update;
pub mod start_game;
pub mod take_item_entity;
pub mod tile_event;
pub mod update_block;
pub mod use_item;

pub use add_entity::*;
pub use add_painting::*;
pub use add_player::*;
pub use adventure_settings::*;
pub use animate::*;
pub use chat::*;
pub use container_ack::*;
pub use container_close::*;
pub use container_open::*;
pub use container_set_data::*;
pub use entity_event::*;
pub use explode::*;
pub use hurt_armor::*;
pub use interact::*;
pub use level_event::*;
pub use login_request::*;
pub use login_response::*;
pub use message::*;
pub use move_entity::*;
pub use move_entity_pos_rot::*;
pub use move_player::*;
pub use place_block::*;
pub use player_action::*;
pub use player_armor_equipment::*;
pub use player_equipment::*;
pub use ready::*;
pub use remove_block::*;
pub use remove_entity::*;
pub use remove_player::*;
pub use request_chunk::*;
pub use respawn::*;
pub use rotate_head::*;
pub use set_entity_motion::*;
pub use set_health::*;
pub use set_riding::*;
pub use set_spawn_position::*;
pub use set_time::*;
pub use sign_update::*;
pub use start_game::*;
pub use take_item_entity::*;
pub use tile_event::*;
pub use update_block::*;
pub use use_item::*;

use crate::reader;
use std::io::{Cursor, Result};

#[derive(Clone, Debug)]
pub enum Packet {
    LoginRequest(LoginRequest),
    LoginResponse(LoginResponse),
    Ready(Ready),
    Message(Message),
    SetTime(SetTime),
    StartGame(StartGame),
    AddPlayer(AddPlayer),
    RemovePlayer(RemovePlayer),
    AddEntity(AddEntity),
    RemoveEntity(RemoveEntity),
    TakeItemEntity(TakeItemEntity),
    MoveEntity(MoveEntity),
    MoveEntityPosRot(MoveEntityPosRot),
    RotateHead(RotateHead),
    MovePlayer(MovePlayer),
    PlaceBlock(PlaceBlock),
    RemoveBlock(RemoveBlock),
    UpdateBlock(UpdateBlock),
    AddPainting(AddPainting),
    Explode(Explode),
    LevelEvent(LevelEvent),
    TileEvent(TileEvent),
    EntityEvent(EntityEvent),
    RequestChunk(RequestChunk),
    PlayerEquipment(PlayerEquipment),
    PlayerArmorEquipment(PlayerArmorEquipment),
    Interact(Interact),
    UseItem(UseItem),
    PlayerAction(PlayerAction),
    HurtArmor(HurtArmor),
    SetEntityMotion(SetEntityMotion),
    SetRiding(SetRiding),
    SetHealth(SetHealth),
    SetSpawnPosition(SetSpawnPosition),
    Animate(Animate),
    Respawn(Respawn),
    ContainerOpen(ContainerOpen),
    ContainerClose(ContainerClose),
    ContainerSetData(ContainerSetData),
    ContainerAck(ContainerAck),
    Chat(Chat),
    SignUpdate(SignUpdate),
    AdventureSettings(AdventureSettings),
}

impl Packet {
    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Option<Self>> {
        match reader::read_u8(&mut cursor)? {
            0x82 => Ok(Some(Packet::LoginRequest(LoginRequest::parse(
                &mut cursor,
            )?))),
            0x83 => Ok(Some(Packet::LoginResponse(LoginResponse::parse(
                &mut cursor,
            )?))),
            0x84 => Ok(Some(Packet::Ready(Ready::parse(&mut cursor)?))),
            0x85 => Ok(Some(Packet::Message(Message::parse(&mut cursor)?))),
            0x86 => Ok(Some(Packet::SetTime(SetTime::parse(&mut cursor)?))),
            0x87 => Ok(Some(Packet::StartGame(StartGame::parse(&mut cursor)?))),
            0x89 => Ok(Some(Packet::AddPlayer(AddPlayer::parse(&mut cursor)?))),
            0x8A => Ok(Some(Packet::RemovePlayer(RemovePlayer::parse(
                &mut cursor,
            )?))),
            0x8C => Ok(Some(Packet::AddEntity(AddEntity::parse(&mut cursor)?))),
            0x8D => Ok(Some(Packet::RemoveEntity(RemoveEntity::parse(
                &mut cursor,
            )?))),
            0x8F => Ok(Some(Packet::TakeItemEntity(TakeItemEntity::parse(
                &mut cursor,
            )?))),
            0x90 => Ok(Some(Packet::MoveEntity(MoveEntity::parse(&mut cursor)?))),
            0x93 => Ok(Some(Packet::MoveEntityPosRot(MoveEntityPosRot::parse(
                &mut cursor,
            )?))),
            0x94 => Ok(Some(Packet::RotateHead(RotateHead::parse(&mut cursor)?))),
            0x95 => Ok(Some(Packet::MovePlayer(MovePlayer::parse(&mut cursor)?))),
            0x96 => Ok(Some(Packet::PlaceBlock(PlaceBlock::parse(&mut cursor)?))),
            0x97 => Ok(Some(Packet::RemoveBlock(RemoveBlock::parse(&mut cursor)?))),
            0x98 => Ok(Some(Packet::UpdateBlock(UpdateBlock::parse(&mut cursor)?))),
            0x99 => Ok(Some(Packet::AddPainting(AddPainting::parse(&mut cursor)?))),
            0x9A => Ok(Some(Packet::Explode(Explode::parse(&mut cursor)?))),
            0x9B => Ok(Some(Packet::LevelEvent(LevelEvent::parse(&mut cursor)?))),
            0x9C => Ok(Some(Packet::TileEvent(TileEvent::parse(&mut cursor)?))),
            0x9D => Ok(Some(Packet::EntityEvent(EntityEvent::parse(&mut cursor)?))),
            0x9E => Ok(Some(Packet::RequestChunk(RequestChunk::parse(
                &mut cursor,
            )?))),
            0xA0 => Ok(Some(Packet::PlayerEquipment(PlayerEquipment::parse(
                &mut cursor,
            )?))),
            0xA1 => Ok(Some(Packet::PlayerArmorEquipment(
                PlayerArmorEquipment::parse(&mut cursor)?,
            ))),
            0xA2 => Ok(Some(Packet::Interact(Interact::parse(&mut cursor)?))),
            0xA3 => Ok(Some(Packet::UseItem(UseItem::parse(&mut cursor)?))),
            0xA4 => Ok(Some(Packet::PlayerAction(PlayerAction::parse(
                &mut cursor,
            )?))),
            0xA6 => Ok(Some(Packet::HurtArmor(HurtArmor::parse(&mut cursor)?))),
            0xA8 => Ok(Some(Packet::SetEntityMotion(SetEntityMotion::parse(
                &mut cursor,
            )?))),
            0xA9 => Ok(Some(Packet::SetRiding(SetRiding::parse(&mut cursor)?))),
            0xAA => Ok(Some(Packet::SetHealth(SetHealth::parse(&mut cursor)?))),
            0xAB => Ok(Some(Packet::SetSpawnPosition(SetSpawnPosition::parse(
                &mut cursor,
            )?))),
            0xAC => Ok(Some(Packet::Animate(Animate::parse(&mut cursor)?))),
            0xAD => Ok(Some(Packet::Respawn(Respawn::parse(&mut cursor)?))),
            0xB0 => Ok(Some(Packet::ContainerOpen(ContainerOpen::parse(
                &mut cursor,
            )?))),
            0xB1 => Ok(Some(Packet::ContainerClose(ContainerClose::parse(
                &mut cursor,
            )?))),
            0xB3 => Ok(Some(Packet::ContainerSetData(ContainerSetData::parse(
                &mut cursor,
            )?))),
            0xB5 => Ok(Some(Packet::ContainerAck(ContainerAck::parse(
                &mut cursor,
            )?))),
            0xB6 => Ok(Some(Packet::Chat(Chat::parse(&mut cursor)?))),
            0xB7 => Ok(Some(Packet::SignUpdate(SignUpdate::parse(&mut cursor)?))),
            0xB8 => Ok(Some(Packet::AdventureSettings(AdventureSettings::parse(
                &mut cursor,
            )?))),
            _ => Ok(None),
        }
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        match self {
            Packet::LoginRequest(packet) => packet.serialize(&mut cursor),
            Packet::LoginResponse(packet) => packet.serialize(&mut cursor),
            Packet::Ready(packet) => packet.serialize(&mut cursor),
            Packet::Message(packet) => packet.serialize(&mut cursor),
            Packet::SetTime(packet) => packet.serialize(&mut cursor),
            Packet::StartGame(packet) => packet.serialize(&mut cursor),
            Packet::AddPlayer(packet) => packet.serialize(&mut cursor),
            Packet::RemovePlayer(packet) => packet.serialize(&mut cursor),
            Packet::AddEntity(packet) => packet.serialize(&mut cursor),
            Packet::RemoveEntity(packet) => packet.serialize(&mut cursor),
            Packet::TakeItemEntity(packet) => packet.serialize(&mut cursor),
            Packet::MoveEntity(packet) => packet.serialize(&mut cursor),
            Packet::MoveEntityPosRot(packet) => packet.serialize(&mut cursor),
            Packet::RotateHead(packet) => packet.serialize(&mut cursor),
            Packet::MovePlayer(packet) => packet.serialize(&mut cursor),
            Packet::PlaceBlock(packet) => packet.serialize(&mut cursor),
            Packet::RemoveBlock(packet) => packet.serialize(&mut cursor),
            Packet::UpdateBlock(packet) => packet.serialize(&mut cursor),
            Packet::AddPainting(packet) => packet.serialize(&mut cursor),
            Packet::Explode(packet) => packet.serialize(&mut cursor),
            Packet::LevelEvent(packet) => packet.serialize(&mut cursor),
            Packet::TileEvent(packet) => packet.serialize(&mut cursor),
            Packet::EntityEvent(packet) => packet.serialize(&mut cursor),
            Packet::RequestChunk(packet) => packet.serialize(&mut cursor),
            Packet::PlayerEquipment(packet) => packet.serialize(&mut cursor),
            Packet::PlayerArmorEquipment(packet) => packet.serialize(&mut cursor),
            Packet::Interact(packet) => packet.serialize(&mut cursor),
            Packet::UseItem(packet) => packet.serialize(&mut cursor),
            Packet::PlayerAction(packet) => packet.serialize(&mut cursor),
            Packet::HurtArmor(packet) => packet.serialize(&mut cursor),
            Packet::SetEntityMotion(packet) => packet.serialize(&mut cursor),
            Packet::SetRiding(packet) => packet.serialize(&mut cursor),
            Packet::SetHealth(packet) => packet.serialize(&mut cursor),
            Packet::SetSpawnPosition(packet) => packet.serialize(&mut cursor),
            Packet::Animate(packet) => packet.serialize(&mut cursor),
            Packet::Respawn(packet) => packet.serialize(&mut cursor),
            Packet::ContainerOpen(packet) => packet.serialize(&mut cursor),
            Packet::ContainerClose(packet) => packet.serialize(&mut cursor),
            Packet::ContainerSetData(packet) => packet.serialize(&mut cursor),
            Packet::ContainerAck(packet) => packet.serialize(&mut cursor),
            Packet::Chat(packet) => packet.serialize(&mut cursor),
            Packet::SignUpdate(packet) => packet.serialize(&mut cursor),
            Packet::AdventureSettings(packet) => packet.serialize(&mut cursor),
        }
    }
}

impl From<LoginRequest> for Packet {
    fn from(packet: LoginRequest) -> Self {
        Packet::LoginRequest(packet)
    }
}

impl From<LoginResponse> for Packet {
    fn from(packet: LoginResponse) -> Self {
        Packet::LoginResponse(packet)
    }
}

impl From<Ready> for Packet {
    fn from(packet: Ready) -> Self {
        Packet::Ready(packet)
    }
}

impl From<Message> for Packet {
    fn from(packet: Message) -> Self {
        Packet::Message(packet)
    }
}

impl From<SetTime> for Packet {
    fn from(packet: SetTime) -> Self {
        Packet::SetTime(packet)
    }
}

impl From<StartGame> for Packet {
    fn from(packet: StartGame) -> Self {
        Packet::StartGame(packet)
    }
}

impl From<AddPlayer> for Packet {
    fn from(packet: AddPlayer) -> Self {
        Packet::AddPlayer(packet)
    }
}

impl From<RemovePlayer> for Packet {
    fn from(packet: RemovePlayer) -> Self {
        Packet::RemovePlayer(packet)
    }
}

impl From<AddEntity> for Packet {
    fn from(packet: AddEntity) -> Self {
        Packet::AddEntity(packet)
    }
}

impl From<RemoveEntity> for Packet {
    fn from(packet: RemoveEntity) -> Self {
        Packet::RemoveEntity(packet)
    }
}

impl From<TakeItemEntity> for Packet {
    fn from(packet: TakeItemEntity) -> Self {
        Packet::TakeItemEntity(packet)
    }
}

impl From<MoveEntity> for Packet {
    fn from(packet: MoveEntity) -> Self {
        Packet::MoveEntity(packet)
    }
}

impl From<MoveEntityPosRot> for Packet {
    fn from(packet: MoveEntityPosRot) -> Self {
        Packet::MoveEntityPosRot(packet)
    }
}

impl From<RotateHead> for Packet {
    fn from(packet: RotateHead) -> Self {
        Packet::RotateHead(packet)
    }
}

impl From<MovePlayer> for Packet {
    fn from(packet: MovePlayer) -> Self {
        Packet::MovePlayer(packet)
    }
}

impl From<PlaceBlock> for Packet {
    fn from(packet: PlaceBlock) -> Self {
        Packet::PlaceBlock(packet)
    }
}

impl From<RemoveBlock> for Packet {
    fn from(packet: RemoveBlock) -> Self {
        Packet::RemoveBlock(packet)
    }
}

impl From<UpdateBlock> for Packet {
    fn from(packet: UpdateBlock) -> Self {
        Packet::UpdateBlock(packet)
    }
}

impl From<AddPainting> for Packet {
    fn from(packet: AddPainting) -> Self {
        Packet::AddPainting(packet)
    }
}

impl From<Explode> for Packet {
    fn from(packet: Explode) -> Self {
        Packet::Explode(packet)
    }
}

impl From<LevelEvent> for Packet {
    fn from(packet: LevelEvent) -> Self {
        Packet::LevelEvent(packet)
    }
}

impl From<TileEvent> for Packet {
    fn from(packet: TileEvent) -> Self {
        Packet::TileEvent(packet)
    }
}

impl From<EntityEvent> for Packet {
    fn from(packet: EntityEvent) -> Self {
        Packet::EntityEvent(packet)
    }
}

impl From<RequestChunk> for Packet {
    fn from(packet: RequestChunk) -> Self {
        Packet::RequestChunk(packet)
    }
}

impl From<PlayerEquipment> for Packet {
    fn from(packet: PlayerEquipment) -> Self {
        Packet::PlayerEquipment(packet)
    }
}

impl From<PlayerArmorEquipment> for Packet {
    fn from(packet: PlayerArmorEquipment) -> Self {
        Packet::PlayerArmorEquipment(packet)
    }
}

impl From<Interact> for Packet {
    fn from(packet: Interact) -> Self {
        Packet::Interact(packet)
    }
}

impl From<UseItem> for Packet {
    fn from(packet: UseItem) -> Self {
        Packet::UseItem(packet)
    }
}

impl From<PlayerAction> for Packet {
    fn from(packet: PlayerAction) -> Self {
        Packet::PlayerAction(packet)
    }
}

impl From<HurtArmor> for Packet {
    fn from(packet: HurtArmor) -> Self {
        Packet::HurtArmor(packet)
    }
}

impl From<SetEntityMotion> for Packet {
    fn from(packet: SetEntityMotion) -> Self {
        Packet::SetEntityMotion(packet)
    }
}

impl From<SetRiding> for Packet {
    fn from(packet: SetRiding) -> Self {
        Packet::SetRiding(packet)
    }
}

impl From<SetHealth> for Packet {
    fn from(packet: SetHealth) -> Self {
        Packet::SetHealth(packet)
    }
}

impl From<SetSpawnPosition> for Packet {
    fn from(packet: SetSpawnPosition) -> Self {
        Packet::SetSpawnPosition(packet)
    }
}

impl From<Animate> for Packet {
    fn from(packet: Animate) -> Self {
        Packet::Animate(packet)
    }
}

impl From<Respawn> for Packet {
    fn from(packet: Respawn) -> Self {
        Packet::Respawn(packet)
    }
}

impl From<ContainerOpen> for Packet {
    fn from(packet: ContainerOpen) -> Self {
        Packet::ContainerOpen(packet)
    }
}

impl From<ContainerClose> for Packet {
    fn from(packet: ContainerClose) -> Self {
        Packet::ContainerClose(packet)
    }
}

impl From<ContainerSetData> for Packet {
    fn from(packet: ContainerSetData) -> Self {
        Packet::ContainerSetData(packet)
    }
}

impl From<ContainerAck> for Packet {
    fn from(packet: ContainerAck) -> Self {
        Packet::ContainerAck(packet)
    }
}

impl From<Chat> for Packet {
    fn from(packet: Chat) -> Self {
        Packet::Chat(packet)
    }
}

impl From<SignUpdate> for Packet {
    fn from(packet: SignUpdate) -> Self {
        Packet::SignUpdate(packet)
    }
}

impl From<AdventureSettings> for Packet {
    fn from(packet: AdventureSettings) -> Self {
        Packet::AdventureSettings(packet)
    }
}
