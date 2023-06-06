use crate::{reader, writer};
use std::io::{Cursor, Result};

#[derive(Clone, Debug)]
pub struct Message {
    pub username: String,
    pub message: String,
}

impl Message {
    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(Self {
            username: reader::read_string(&mut cursor)?,
            message: reader::read_string(&mut cursor)?,
        })
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        writer::write_u8(&mut cursor, 0x85)?;
        writer::write_string(&mut cursor, &self.username)?;
        writer::write_string(&mut cursor, &self.message)?;
        Ok(())
    }
}

