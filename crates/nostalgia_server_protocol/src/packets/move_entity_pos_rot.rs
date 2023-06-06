use crate::{reader, writer};
use std::io::{Cursor, Result};
use types::Vector3;

#[derive(Clone, Debug)]
pub struct MoveEntityPosRot {
    pub entity_id: i32,
    pub pos: Vector3,
    pub yaw: u8,
    pub pitch: u8,
}

impl MoveEntityPosRot {
    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(Self {
            entity_id: reader::read_i32(&mut cursor)?,
            pos: reader::read_vector3(&mut cursor)?,
            yaw: reader::read_u8(&mut cursor)?,
            pitch: reader::read_u8(&mut cursor)?,
        })
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        writer::write_u8(&mut cursor, 0x93)?;
        writer::write_i32(&mut cursor, self.entity_id)?;
        writer::write_vector3(&mut cursor, &self.pos)?;
        writer::write_u8(&mut cursor, self.yaw)?;
        writer::write_u8(&mut cursor, self.pitch)?;
        Ok(())
    }
}

