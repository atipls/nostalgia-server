use crate::{reader, writer};
use std::io::{Cursor, Result};

#[derive(Clone, Debug)]
pub struct SetTime {
    pub time: i32,
    pub started: bool,
}

impl SetTime {
    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(Self {
            time: reader::read_i32(&mut cursor)?,
            started: reader::read_u8(&mut cursor)? & 0x80 > 0,
        })
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        writer::write_u8(&mut cursor, 0x86)?;
        writer::write_i32(&mut cursor, self.time)?;
        writer::write_u8(&mut cursor, if self.started { 0x80 } else { 0x00 })?;
        Ok(())
    }
}
