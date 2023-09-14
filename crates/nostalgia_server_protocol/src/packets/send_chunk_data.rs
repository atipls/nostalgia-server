use crate::interop::RequestedChunk;
use crate::{reader, writer};
use std::io::{Cursor, Result};

#[derive(Clone, Debug)]
pub struct SendChunkData {
    pub x: i32,
    pub z: i32,
    pub is_new: u8,
    pub chunk: RequestedChunk,
}

impl SendChunkData {
    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(Self {
            x: reader::read_i32(&mut cursor)?,
            z: reader::read_i32(&mut cursor)?,
            is_new: reader::read_u8(&mut cursor)?,
            chunk: RequestedChunk::parse(&mut cursor)?,
        })
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        writer::write_u8(&mut cursor, 0x9F)?;
        writer::write_i32(&mut cursor, self.x)?;
        writer::write_i32(&mut cursor, self.z)?;
        writer::write_u8(&mut cursor, self.is_new)?;
        self.chunk.serialize(&mut cursor)?;
        Ok(())
    }
}
