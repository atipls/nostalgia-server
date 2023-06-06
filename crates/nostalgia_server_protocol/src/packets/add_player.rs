use crate::{reader, writer};
use std::io::{Cursor, Result};
use types::Vector3;

#[derive(Clone, Debug)]
pub struct AddPlayer {
    pub player_id: u64,
    pub username: String,
    pub entity_id: i32,
    pub pos: Vector3,
    pub yaw: u8,
    pub pitch: u8,
    pub item_id: u16,
    pub item_aux_value: u16,
}

impl AddPlayer {
    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(Self {
            player_id: reader::read_u64(&mut cursor)?,
            username: reader::read_string(&mut cursor)?,
            entity_id: reader::read_i32(&mut cursor)?,
            pos: reader::read_vector3(&mut cursor)?,
            yaw: reader::read_u8(&mut cursor)?,
            pitch: reader::read_u8(&mut cursor)?,
            item_id: reader::read_u16(&mut cursor)?,
            item_aux_value: reader::read_u16(&mut cursor)?,
        })
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        writer::write_u8(&mut cursor, 0x89)?;
        writer::write_u64(&mut cursor, self.player_id)?;
        writer::write_string(&mut cursor, &self.username)?;
        writer::write_i32(&mut cursor, self.entity_id)?;
        writer::write_vector3(&mut cursor, &self.pos)?;
        writer::write_u8(&mut cursor, self.yaw)?;
        writer::write_u8(&mut cursor, self.pitch)?;
        writer::write_u16(&mut cursor, self.item_id)?;
        writer::write_u16(&mut cursor, self.item_aux_value)?;
        Ok(())
    }
}

