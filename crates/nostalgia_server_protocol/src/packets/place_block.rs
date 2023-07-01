use crate::{reader, writer};
use std::io::{Cursor, Result};

#[derive(Clone, Debug)]
pub struct PlaceBlock {
    pub entity_id: i32,
    pub x: i32,
    pub z: i32,
    pub y: u8,
    pub block: u8,
    pub meta: u8,
    pub face: u8,
}

impl PlaceBlock {
    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(Self {
            entity_id: reader::read_i32(&mut cursor)?,
            x: reader::read_i32(&mut cursor)?,
            z: reader::read_i32(&mut cursor)?,
            y: reader::read_u8(&mut cursor)?,
            block: reader::read_u8(&mut cursor)?,
            meta: reader::read_u8(&mut cursor)?,
            face: reader::read_u8(&mut cursor)?,
        })
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        writer::write_u8(&mut cursor, 0x96)?;
        writer::write_i32(&mut cursor, self.entity_id)?;
        writer::write_i32(&mut cursor, self.x)?;
        writer::write_i32(&mut cursor, self.z)?;
        writer::write_u8(&mut cursor, self.y)?;
        writer::write_u8(&mut cursor, self.block)?;
        writer::write_u8(&mut cursor, self.meta)?;
        writer::write_u8(&mut cursor, self.face)?;
        Ok(())
    }
}
