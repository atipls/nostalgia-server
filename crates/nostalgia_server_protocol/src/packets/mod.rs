pub mod login_request;
pub mod login_response;
pub mod ready;
pub mod message;
pub mod set_time;
pub mod start_game;
pub mod add_player;
pub mod remove_player;
pub mod add_entity;
pub mod remove_entity;
pub mod take_item_entity;
pub mod move_entity;
pub mod move_entity_pos_rot;
pub mod rotate_head;
pub mod move_player;
pub mod place_block;
pub mod remove_block;
pub mod update_block;
pub mod add_painting;
pub mod explode;
pub mod level_event;
pub mod tile_event;
pub mod entity_event;
pub mod request_chunk;
pub mod player_equipment;
pub mod player_armor_equipment;
pub mod interact;
pub mod use_item;
pub mod player_action;
pub mod hurt_armor;
pub mod set_entity_motion;
pub mod set_riding;
pub mod set_health;
pub mod set_spawn_position;
pub mod animate;
pub mod respawn;
pub mod container_open;
pub mod container_close;
pub mod container_set_data;
pub mod container_ack;
pub mod chat;
pub mod sign_update;
pub mod adventure_settings;

pub use login_request::*;
pub use login_response::*;
pub use ready::*;
pub use message::*;
pub use set_time::*;
pub use start_game::*;
pub use add_player::*;
pub use remove_player::*;
pub use add_entity::*;
pub use remove_entity::*;
pub use take_item_entity::*;
pub use move_entity::*;
pub use move_entity_pos_rot::*;
pub use rotate_head::*;
pub use move_player::*;
pub use place_block::*;
pub use remove_block::*;
pub use update_block::*;
pub use add_painting::*;
pub use explode::*;
pub use level_event::*;
pub use tile_event::*;
pub use entity_event::*;
pub use request_chunk::*;
pub use player_equipment::*;
pub use player_armor_equipment::*;
pub use interact::*;
pub use use_item::*;
pub use player_action::*;
pub use hurt_armor::*;
pub use set_entity_motion::*;
pub use set_riding::*;
pub use set_health::*;
pub use set_spawn_position::*;
pub use animate::*;
pub use respawn::*;
pub use container_open::*;
pub use container_close::*;
pub use container_set_data::*;
pub use container_ack::*;
pub use chat::*;
pub use sign_update::*;
pub use adventure_settings::*;

use std::io::{Cursor, Result};
use crate::reader;

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
            0x82 => Ok(Some(Packet::LoginRequest(LoginRequest::parse(&mut cursor)?))),
            0x83 => Ok(Some(Packet::LoginResponse(LoginResponse::parse(&mut cursor)?))),
            0x84 => Ok(Some(Packet::Ready(Ready::parse(&mut cursor)?))),
            0x85 => Ok(Some(Packet::Message(Message::parse(&mut cursor)?))),
            0x86 => Ok(Some(Packet::SetTime(SetTime::parse(&mut cursor)?))),
            0x87 => Ok(Some(Packet::StartGame(StartGame::parse(&mut cursor)?))),
            0x89 => Ok(Some(Packet::AddPlayer(AddPlayer::parse(&mut cursor)?))),
            0x8A => Ok(Some(Packet::RemovePlayer(RemovePlayer::parse(&mut cursor)?))),
            0x8C => Ok(Some(Packet::AddEntity(AddEntity::parse(&mut cursor)?))),
            0x8D => Ok(Some(Packet::RemoveEntity(RemoveEntity::parse(&mut cursor)?))),
            0x8F => Ok(Some(Packet::TakeItemEntity(TakeItemEntity::parse(&mut cursor)?))),
            0x90 => Ok(Some(Packet::MoveEntity(MoveEntity::parse(&mut cursor)?))),
            0x93 => Ok(Some(Packet::MoveEntityPosRot(MoveEntityPosRot::parse(&mut cursor)?))),
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
            0x9E => Ok(Some(Packet::RequestChunk(RequestChunk::parse(&mut cursor)?))),
            0xA0 => Ok(Some(Packet::PlayerEquipment(PlayerEquipment::parse(&mut cursor)?))),
            0xA1 => Ok(Some(Packet::PlayerArmorEquipment(PlayerArmorEquipment::parse(&mut cursor)?))),
            0xA2 => Ok(Some(Packet::Interact(Interact::parse(&mut cursor)?))),
            0xA3 => Ok(Some(Packet::UseItem(UseItem::parse(&mut cursor)?))),
            0xA4 => Ok(Some(Packet::PlayerAction(PlayerAction::parse(&mut cursor)?))),
            0xA6 => Ok(Some(Packet::HurtArmor(HurtArmor::parse(&mut cursor)?))),
            0xA8 => Ok(Some(Packet::SetEntityMotion(SetEntityMotion::parse(&mut cursor)?))),
            0xA9 => Ok(Some(Packet::SetRiding(SetRiding::parse(&mut cursor)?))),
            0xAA => Ok(Some(Packet::SetHealth(SetHealth::parse(&mut cursor)?))),
            0xAB => Ok(Some(Packet::SetSpawnPosition(SetSpawnPosition::parse(&mut cursor)?))),
            0xAC => Ok(Some(Packet::Animate(Animate::parse(&mut cursor)?))),
            0xAD => Ok(Some(Packet::Respawn(Respawn::parse(&mut cursor)?))),
            0xB0 => Ok(Some(Packet::ContainerOpen(ContainerOpen::parse(&mut cursor)?))),
            0xB1 => Ok(Some(Packet::ContainerClose(ContainerClose::parse(&mut cursor)?))),
            0xB3 => Ok(Some(Packet::ContainerSetData(ContainerSetData::parse(&mut cursor)?))),
            0xB5 => Ok(Some(Packet::ContainerAck(ContainerAck::parse(&mut cursor)?))),
            0xB6 => Ok(Some(Packet::Chat(Chat::parse(&mut cursor)?))),
            0xB7 => Ok(Some(Packet::SignUpdate(SignUpdate::parse(&mut cursor)?))),
            0xB8 => Ok(Some(Packet::AdventureSettings(AdventureSettings::parse(&mut cursor)?))),
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
