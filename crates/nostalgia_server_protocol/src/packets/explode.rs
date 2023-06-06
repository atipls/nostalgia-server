use crate::{reader, writer};
use std::io::{Cursor, Result};
use types::Vector3;

#[derive(Clone, Debug)]
pub struct Explode {
    pub pos: Vector3,
    pub radius: f32,
    pub count: i32,
}

impl Explode {
    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(Self {
            pos: reader::read_vector3(&mut cursor)?,
            radius: reader::read_f32(&mut cursor)?,
            count: reader::read_i32(&mut cursor)?,
        })
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        writer::write_u8(&mut cursor, 0x9A)?;
        writer::write_vector3(&mut cursor, &self.pos)?;
        writer::write_f32(&mut cursor, self.radius)?;
        writer::write_i32(&mut cursor, self.count)?;
        Ok(())
    }
}

