use crate::{reader, writer};
use std::io::{Cursor, Result};
use types::Vector3;

#[derive(Clone, Debug)]
pub struct UseItem {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub block: u16,
    pub meta: u8,
    pub id: i32,
    pub f_pos: Vector3,
    pub pos: Vector3,
}

impl UseItem {
    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(Self {
            x: reader::read_i32(&mut cursor)?,
            y: reader::read_i32(&mut cursor)?,
            z: reader::read_i32(&mut cursor)?,
            block: reader::read_u16(&mut cursor)?,
            meta: reader::read_u8(&mut cursor)?,
            id: reader::read_i32_le(&mut cursor)?,
            f_pos: reader::read_vector3(&mut cursor)?,
            pos: reader::read_vector3(&mut cursor)?,
        })
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        writer::write_u8(&mut cursor, 0xA3)?;
        writer::write_i32(&mut cursor, self.x)?;
        writer::write_i32(&mut cursor, self.y)?;
        writer::write_i32(&mut cursor, self.z)?;
        writer::write_u16(&mut cursor, self.block)?;
        writer::write_u8(&mut cursor, self.meta)?;
        writer::write_i32(&mut cursor, self.id)?;
        writer::write_vector3(&mut cursor, &self.f_pos)?;
        writer::write_vector3(&mut cursor, &self.pos)?;
        Ok(())
    }
}
