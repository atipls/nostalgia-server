use crate::{reader, writer};
use std::io::{Cursor, Result};

#[derive(Clone, Debug)]
pub struct LevelEvent {
    pub event_id: u16,
    pub x: u16,
    pub y: u16,
    pub z: u16,
    pub data: i32,
}

impl LevelEvent {
    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(Self {
            event_id: reader::read_u16(&mut cursor)?,
            x: reader::read_u16(&mut cursor)?,
            y: reader::read_u16(&mut cursor)?,
            z: reader::read_u16(&mut cursor)?,
            data: reader::read_i32(&mut cursor)?,
        })
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        writer::write_u8(&mut cursor, 0x9B)?;
        writer::write_u16(&mut cursor, self.event_id)?;
        writer::write_u16(&mut cursor, self.x)?;
        writer::write_u16(&mut cursor, self.y)?;
        writer::write_u16(&mut cursor, self.z)?;
        writer::write_i32(&mut cursor, self.data)?;
        Ok(())
    }
}

