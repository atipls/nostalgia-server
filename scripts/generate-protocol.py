#!/usr/bin/env python3

import os, re
from dataclasses import dataclass

def to_constant_case(name):
  name = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
  return re.sub('([a-z0-9])([A-Z])', r'\1_\2', name).upper()

def to_identifier_case(name):
    name = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
    return re.sub('([a-z0-9])([A-Z])', r'\1_\2', name).lower()

@dataclass(frozen=True)
class Field:
    name: str
    type: str

@dataclass(frozen=True)
class Packet:
    name: str
    id: int
    fields: list[Field]

    def as_identifier(self):
        return to_identifier_case(self.name)

MINECRAFT_PACKETS = [
    Packet(name="LoginRequest", id=130, fields=[
        Field(name="username", type="String"),
        Field(name="protocol_major", type="i32"),
        Field(name="protocol_minor", type="i32"),
        Field(name="client_id", type="u32"),
        Field(name="realms_data", type="String"),
    ]),
    Packet(name="LoginResponse", id=131, fields=[
        Field(name="status", type="i32"),
    ]),
    Packet(name="Ready", id=132, fields=[
        Field(name="status", type="u8"),
    ]),
    Packet(name="Message", id=133, fields=[
        Field(name="username", type="String"),
        Field(name="message", type="String"),
    ]),
    Packet(name="SetTime", id=134, fields=[
        Field(name="time", type="i32"),
    ]),
    Packet(name="StartGame", id=135, fields=[
        Field(name="world_seed", type="i32"),
        Field(name="generator_version", type="i32"),
        Field(name="gamemode", type="i32"),
        Field(name="entity_id", type="i32"),
        Field(name="position", type="Vector3"),
    ]),
    # Packet(name="AddMob", id=136, fields=[
    #     Field(name="entity_id", type="i32"),
    #     Field(name="entity_type", type="i32"),
    #     Field(name="pos", type="Vector3"),
    #     Field(name="yaw", type="u8"),
    #     Field(name="pitch", type="u8"),
    #     Field(name="metadata", type="SyncedEntityData"),
    # ]),
    Packet(name="AddPlayer", id=137, fields=[
        Field(name="player_id", type="u64"),
        Field(name="username", type="String"),
        Field(name="entity_id", type="i32"),
        Field(name="pos", type="Vector3"),
        Field(name="yaw", type="u8"),
        Field(name="pitch", type="u8"),
        Field(name="item_id", type="u16"),
        Field(name="item_aux_value", type="u16"),
    ]),
    Packet(name="RemovePlayer", id=138, fields=[
        Field(name="entity_id", type="i32"),
        Field(name="player_id", type="u64"),
    ]),
    Packet(name="AddEntity", id=140, fields=[
        Field(name="entity_id", type="i32"),
        Field(name="entity_type", type="u8"),
        Field(name="pos", type="Vector3"),
        Field(name="moved", type="i32"),
        Field(name="velocity", type="Vector3"),
    ]),
    Packet(name="RemoveEntity", id=141, fields=[
        Field(name="entity_id", type="i32"),
    ]),
    # Packet(name="AddItemEntity", id=142, fields=[
    #     Field(name="entity_id", type="i32"),
    #     Field(name="item", type="ItemInstance"),
    #     Field(name="pos", type="Vector3"),
    #     Field(name="yaw", type="u8"),
    #     Field(name="pitch", type="u8"),
    #     Field(name="roll", type="u8"),
    # ]),
    Packet(name="TakeItemEntity", id=143, fields=[
        Field(name="target", type="i32"),
        Field(name="entity_id", type="i32"),
    ]),
    Packet(name="MoveEntity", id=144, fields=[
        Field(name="entity_id", type="i32"),
        Field(name="pos", type="Vector3"),
    ]),
    Packet(name="MoveEntityPosRot", id=147, fields=[
        Field(name="entity_id", type="i32"),
        Field(name="pos", type="Vector3"),
        Field(name="yaw", type="u8"),
        Field(name="pitch", type="u8"),
    ]),
    Packet(name="RotateHead", id=148, fields=[
        Field(name="entity_id", type="i32"),
        Field(name="yaw", type="u8"),
    ]),
    Packet(name="MovePlayer", id=149, fields=[
        Field(name="entity_id", type="i32"),
        Field(name="pos", type="Vector3"),
        Field(name="rot", type="Vector3"),
    ]),
    Packet(name="PlaceBlock", id=150, fields=[
        Field(name="entity_id", type="i32"),
        Field(name="x", type="i32"),
        Field(name="z", type="i32"),
        Field(name="y", type="u8"),
        Field(name="block", type="u8"),
        Field(name="meta", type="u8"),
        Field(name="face", type="u8"),
    ]),
    Packet(name="RemoveBlock", id=151, fields=[
        Field(name="entity_id", type="i32"),
        Field(name="x", type="i32"),
        Field(name="z", type="i32"),
        Field(name="y", type="u8"),
    ]),
    Packet(name="UpdateBlock", id=152, fields=[
        Field(name="entity_id", type="i32"),
        Field(name="x", type="i32"),
        Field(name="z", type="i32"),
        Field(name="y", type="u8"),
        Field(name="block", type="u8"),
        Field(name="meta", type="u8"),
    ]),
    Packet(name="AddPainting", id=153, fields=[
        Field(name="entity_id", type="i32"),
        Field(name="x", type="i32"),
        Field(name="y", type="i32"),
        Field(name="direction", type="i32"),
        Field(name="title", type="String"),
    ]),
    Packet(name="Explode", id=154, fields=[
        Field(name="pos", type="Vector3"),
        Field(name="radius", type="f32"),
        Field(name="count", type="i32"),
    ]),
    Packet(name="LevelEvent", id=155, fields=[
        Field(name="event_id", type="u16"),
        Field(name="x", type="u16"),
        Field(name="y", type="u16"),
        Field(name="z", type="u16"),
        Field(name="data", type="i32"),
    ]),
    Packet(name="TileEvent", id=156, fields=[
        Field(name="x", type="i32"),
        Field(name="y", type="i32"),
        Field(name="case1", type="i32"),
        Field(name="case2", type="i32"),
    ]),
    Packet(name="EntityEvent", id=157, fields=[
        Field(name="entity_id", type="i32"),
        Field(name="event_id", type="u8"),
    ]),
    Packet(name="RequestChunk", id=158, fields=[
        Field(name="x", type="i32"),
        Field(name="z", type="i32"),
    ]),
    # Packet(name="ChunkData", id=159, fields=[
    #     Field(name="x", type="i32"),
    #     Field(name="z", type="i32"),
    #     Field(name="is_new", type="u8"),
    #     Field(name="data", type="Chunk"),
    # ]),
    Packet(name="PlayerEquipment", id=160, fields=[
        Field(name="entity_id", type="i32"),
        Field(name="block", type="u16"),
        Field(name="meta", type="u16"),
        Field(name="slot", type="u8"),
    ]),
    Packet(name="PlayerArmorEquipment", id=161, fields=[
        Field(name="entity_id", type="i32"),
        Field(name="slot1", type="u16"),
        Field(name="slot2", type="u16"),
        Field(name="slot3", type="u16"),
        Field(name="slot4", type="u16"),
    ]),
    Packet(name="Interact", id=162, fields=[
        Field(name="action", type="u8"),
        Field(name="entity_id", type="i32"),
        Field(name="target_id", type="i32"),
    ]),
    Packet(name="UseItem", id=163, fields=[
        Field(name="x", type="i32"),
        Field(name="y", type="i32"),
        Field(name="face", type="i32"),
        Field(name="block", type="u16"),
        Field(name="meta", type="u8"),
        Field(name="id", type="i32"),
        Field(name="f_pos", type="Vector3"),
        Field(name="pos", type="Vector3"),
    ]),
    Packet(name="PlayerAction", id=164, fields=[
        Field(name="action", type="i32"),
        Field(name="x", type="i32"),
        Field(name="y", type="i32"),
        Field(name="face", type="i32"),
        Field(name="entity_id", type="i32"),
    ]),
    Packet(name="HurtArmor", id=166, fields=[
        Field(name="armor", type="u8"),
    ]),
    # Packet(name="SetEntityData", id=167, fields=[
    #     Field(name="entity_id", type="i32"),
    #     Field(name="metadata", type="SyncedEntityData"),
    # ]),
    Packet(name="SetEntityMotion", id=168, fields=[
        Field(name="unk0", type="u8"),
        Field(name="entity_id", type="i32"),
        Field(name="x", type="u16"),
        Field(name="y", type="u16"),
        Field(name="z", type="u16"),
    ]),
    Packet(name="SetRiding", id=169, fields=[
        Field(name="entity_id", type="i32"),
        Field(name="target_id", type="i32"),
    ]),
    Packet(name="SetHealth", id=170, fields=[
        Field(name="health", type="u8"),
    ]),
    Packet(name="SetSpawnPosition", id=171, fields=[
        Field(name="x", type="i32"),
        Field(name="z", type="i32"),
        Field(name="y", type="u8"),
    ]),
    Packet(name="Animate", id=172, fields=[
        Field(name="action", type="u8"),
        Field(name="entity_id", type="i32"),
    ]),
    Packet(name="Respawn", id=173, fields=[
        Field(name="entity_id", type="i32"),
        Field(name="pos", type="Vector3"),
    ]),
    # Packet(name="SendInventory", id=174, fields=[
    #     Field(name="entity_id", type="i32"),
    #     Field(name="window_id", type="u8"),
    #     Field(name="items", type="ItemInstanceList"),
    # ]),
    # Packet(name="DropItem", id=175, fields=[
    #     Field(name="entity_id", type="i32"),
    #     Field(name="unk0", type="u8"),
    #     Field(name="item", type="ItemInstance"),
    # ]),
    Packet(name="ContainerOpen", id=176, fields=[
        Field(name="window_id", type="u8"),
        Field(name="container_type", type="u8"),
        Field(name="slot", type="u8"),
        Field(name="title", type="String"),
    ]),
    Packet(name="ContainerClose", id=177, fields=[
        Field(name="window_id", type="u8"),
    ]),
    # Packet(name="ContainerSetSlot", id=178, fields=[
    #     Field(name="window_id", type="u8"),
    #     Field(name="slot", type="u16"),
    #     Field(name="item", type="ItemInstance"),
    # ]),
    Packet(name="ContainerSetData", id=179, fields=[
        Field(name="window_id", type="u8"),
        Field(name="property", type="u16"),
        Field(name="value", type="u16"),
    ]),
    # Packet(name="ContainerSetContent", id=180, fields=[
    #     Field(name="window_id", type="u8"),
    #     Field(name="items", type="ItemInstanceList"),
    # ]),
    Packet(name="ContainerAck", id=181, fields=[
        Field(name="window_id", type="u8"),
        Field(name="unknown_first", type="u16"),
        Field(name="unknown_second", type="u8"),
    ]),
    Packet(name="Chat", id=182, fields=[
        Field(name="message", type="String"),
    ]),
    Packet(name="SignUpdate", id=183, fields=[
        Field(name="x", type="u16"),
        Field(name="y", type="u8"),
        Field(name="z", type="u16"),
        Field(name="lines", type="String"),
    ]),
    Packet(name="AdventureSettings", id=184, fields=[
        Field(name="unknown_first", type="u8"),
        Field(name="unknown_second", type="u32"),
    ]),
]

BASE_PATH = "crates/nostalgia_server_protocol/src/packets/"

with open(f"{BASE_PATH}mod.rs", "w") as file:
    for packet in MINECRAFT_PACKETS:
        file.write(f"pub mod {packet.as_identifier()};\n")
    file.write("\n")

    for packet in MINECRAFT_PACKETS:
        file.write(f"pub use {packet.as_identifier()}::*;\n")
    file.write("\n")

    file.write("use std::io::{Cursor, Result};\n")
    file.write("use crate::reader;\n\n")

    file.write("#[derive(Clone, Debug)]\n")
    file.write("pub enum Packet {\n")
    for packet in MINECRAFT_PACKETS:
        file.write(f"    {packet.name}({packet.name}),\n")
    file.write("}\n\n")

    file.write("impl Packet {\n")
    file.write("    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Option<Self>> {\n")
    file.write("        match reader::read_u8(&mut cursor)? {\n")
    for packet in MINECRAFT_PACKETS:
        file.write(f"            0x{packet.id:2X} => Ok(Some(Packet::{packet.name}({packet.name}::parse(&mut cursor)?))),\n")
    file.write("            _ => Ok(None),\n")
    file.write("        }\n")
    file.write("    }\n\n")
    file.write("    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {\n")
    file.write("        match self {\n")
    for packet in MINECRAFT_PACKETS:
        file.write(f"            Packet::{packet.name}(packet) => packet.serialize(&mut cursor),\n")
    file.write("        }\n")
    file.write("    }\n")
    file.write("}\n")

    file.write("\n")

    for packet in MINECRAFT_PACKETS:
        file.write(f"impl From<{packet.name}> for Packet {{\n")
        file.write(f"    fn from(packet: {packet.name}) -> Self {{\n")
        file.write(f"        Packet::{packet.name}(packet)\n")
        file.write(f"    }}\n")
        file.write(f"}}\n")
        file.write("\n")


for packet in MINECRAFT_PACKETS:
    has_vector3 = False
    for field in packet.fields:
        if field.type == "Vector3":
            has_vector3 = True
            break

    implementation = f"{BASE_PATH}{to_identifier_case(packet.name)}.rs"
    implementation_file = open(implementation, "w")

    implementation_file.write(f"use crate::{{reader, writer}};\n")
    implementation_file.write(f"use std::io::{{Cursor, Result}};\n")
    if has_vector3:
        implementation_file.write(f"use types::Vector3;\n")
    implementation_file.write("\n")

    implementation_file.write(f"#[derive(Clone, Debug)]\n")
    implementation_file.write(f"pub struct {packet.name} {{\n")
    for field in packet.fields:
        implementation_file.write(f"    pub {field.name}: {field.type},\n")
    implementation_file.write("}\n\n")

    implementation_file.write(f"impl {packet.name} {{\n")
    implementation_file.write(f"    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {{\n")
    implementation_file.write(f"        Ok(Self {{\n")
    for field in packet.fields:
        implementation_file.write(f"            {field.name}: reader::read_{field.type.lower()}(&mut cursor)?,\n")
    implementation_file.write(f"        }})\n")
    implementation_file.write(f"    }}\n\n")

    implementation_file.write(f"    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {{\n")
    implementation_file.write(f"        writer::write_u8(&mut cursor, 0x{packet.id:2X})?;\n")
    for field in packet.fields:
        reference_symbol = "&" if field.type in ["String", "Vector3"] else ""
        implementation_file.write(f"        writer::write_{field.type.lower()}(&mut cursor, {reference_symbol}self.{field.name})?;\n")
    implementation_file.write(f"        Ok(())\n")
    implementation_file.write(f"    }}\n")

    implementation_file.write(f"}}\n\n")
