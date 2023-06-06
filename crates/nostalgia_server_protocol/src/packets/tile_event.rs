use crate::{reader, writer};
use std::io::{Cursor, Result};

#[derive(Clone, Debug)]
pub struct TileEvent {
    pub x: i32,
    pub y: i32,
    pub case1: i32,
    pub case2: i32,
}

impl TileEvent {
    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(Self {
            x: reader::read_i32(&mut cursor)?,
            y: reader::read_i32(&mut cursor)?,
            case1: reader::read_i32(&mut cursor)?,
            case2: reader::read_i32(&mut cursor)?,
        })
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        writer::write_u8(&mut cursor, 0x9C)?;
        writer::write_i32(&mut cursor, self.x)?;
        writer::write_i32(&mut cursor, self.y)?;
        writer::write_i32(&mut cursor, self.case1)?;
        writer::write_i32(&mut cursor, self.case2)?;
        Ok(())
    }
}

