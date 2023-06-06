use crate::{reader, writer};
use std::io::{Cursor, Result};
use types::Vector3;

#[derive(Clone, Debug)]
pub struct StartGame {
    pub world_seed: i32,
    pub generator_version: i32,
    pub gamemode: i32,
    pub entity_id: i32,
    pub position: Vector3,
}

impl StartGame {
    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(Self {
            world_seed: reader::read_i32(&mut cursor)?,
            generator_version: reader::read_i32(&mut cursor)?,
            gamemode: reader::read_i32(&mut cursor)?,
            entity_id: reader::read_i32(&mut cursor)?,
            position: reader::read_vector3(&mut cursor)?,
        })
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        writer::write_u8(&mut cursor, 0x87)?;
        writer::write_i32(&mut cursor, self.world_seed)?;
        writer::write_i32(&mut cursor, self.generator_version)?;
        writer::write_i32(&mut cursor, self.gamemode)?;
        writer::write_i32(&mut cursor, self.entity_id)?;
        writer::write_vector3(&mut cursor, &self.position)?;
        Ok(())
    }
}

