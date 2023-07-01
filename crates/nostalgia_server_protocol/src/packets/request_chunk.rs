use crate::{reader, writer};
use std::io::{Cursor, Result};

#[derive(Clone, Debug)]
pub struct RequestChunk {
    pub x: i32,
    pub z: i32,
}

impl RequestChunk {
    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(Self {
            x: reader::read_i32(&mut cursor)?,
            z: reader::read_i32(&mut cursor)?,
        })
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        writer::write_u8(&mut cursor, 0x9E)?;
        writer::write_i32(&mut cursor, self.x)?;
        writer::write_i32(&mut cursor, self.z)?;
        Ok(())
    }
}
