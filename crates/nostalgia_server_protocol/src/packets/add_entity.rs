use crate::{reader, writer};
use std::io::{Cursor, Result};
use types::Vector3;

#[derive(Clone, Debug)]
pub struct AddEntity {
    pub entity_id: i32,
    pub entity_type: u8,
    pub pos: Vector3,
    pub moved: i32,
    pub velocity: Vector3,
}

impl AddEntity {
    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(Self {
            entity_id: reader::read_i32(&mut cursor)?,
            entity_type: reader::read_u8(&mut cursor)?,
            pos: reader::read_vector3(&mut cursor)?,
            moved: reader::read_i32(&mut cursor)?,
            velocity: reader::read_vector3(&mut cursor)?,
        })
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        writer::write_u8(&mut cursor, 0x8C)?;
        writer::write_i32(&mut cursor, self.entity_id)?;
        writer::write_u8(&mut cursor, self.entity_type)?;
        writer::write_vector3(&mut cursor, &self.pos)?;
        writer::write_i32(&mut cursor, self.moved)?;
        writer::write_vector3(&mut cursor, &self.velocity)?;
        Ok(())
    }
}

