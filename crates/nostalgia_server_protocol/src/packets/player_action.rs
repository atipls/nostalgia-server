use crate::{reader, writer};
use std::io::{Cursor, Result};

#[derive(Clone, Debug)]
pub struct PlayerAction {
    pub action: i32,
    pub x: i32,
    pub y: i32,
    pub face: i32,
    pub entity_id: i32,
}

impl PlayerAction {
    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(Self {
            action: reader::read_i32(&mut cursor)?,
            x: reader::read_i32(&mut cursor)?,
            y: reader::read_i32(&mut cursor)?,
            face: reader::read_i32(&mut cursor)?,
            entity_id: reader::read_i32(&mut cursor)?,
        })
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        writer::write_u8(&mut cursor, 0xA4)?;
        writer::write_i32(&mut cursor, self.action)?;
        writer::write_i32(&mut cursor, self.x)?;
        writer::write_i32(&mut cursor, self.y)?;
        writer::write_i32(&mut cursor, self.face)?;
        writer::write_i32(&mut cursor, self.entity_id)?;
        Ok(())
    }
}

