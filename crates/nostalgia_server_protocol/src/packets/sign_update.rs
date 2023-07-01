use crate::{reader, writer};
use std::io::{Cursor, Result};

#[derive(Clone, Debug)]
pub struct SignUpdate {
    pub x: u16,
    pub y: u8,
    pub z: u16,
    pub lines: String,
}

impl SignUpdate {
    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(Self {
            x: reader::read_u16(&mut cursor)?,
            y: reader::read_u8(&mut cursor)?,
            z: reader::read_u16(&mut cursor)?,
            lines: reader::read_string(&mut cursor)?,
        })
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        writer::write_u8(&mut cursor, 0xB7)?;
        writer::write_u16(&mut cursor, self.x)?;
        writer::write_u8(&mut cursor, self.y)?;
        writer::write_u16(&mut cursor, self.z)?;
        writer::write_string(&mut cursor, &self.lines)?;
        Ok(())
    }
}
