use crate::{reader, writer};
use std::io::{Cursor, Result};

#[derive(Clone, Debug)]
pub struct SetEntityMotion {
    pub unk0: u8,
    pub entity_id: i32,
    pub x: u16,
    pub y: u16,
    pub z: u16,
}

impl SetEntityMotion {
    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(Self {
            unk0: reader::read_u8(&mut cursor)?,
            entity_id: reader::read_i32(&mut cursor)?,
            x: reader::read_u16(&mut cursor)?,
            y: reader::read_u16(&mut cursor)?,
            z: reader::read_u16(&mut cursor)?,
        })
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        writer::write_u8(&mut cursor, 0xA8)?;
        writer::write_u8(&mut cursor, self.unk0)?;
        writer::write_i32(&mut cursor, self.entity_id)?;
        writer::write_u16(&mut cursor, self.x)?;
        writer::write_u16(&mut cursor, self.y)?;
        writer::write_u16(&mut cursor, self.z)?;
        Ok(())
    }
}
