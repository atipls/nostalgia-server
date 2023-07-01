use crate::{reader, writer};
use std::io::{Cursor, Result};

#[derive(Clone, Debug)]
pub struct AdventureSettings {
    pub unknown_first: u8,
    pub unknown_second: u32,
}

impl AdventureSettings {
    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(Self {
            unknown_first: reader::read_u8(&mut cursor)?,
            unknown_second: reader::read_u32(&mut cursor)?,
        })
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        writer::write_u8(&mut cursor, 0xB8)?;
        writer::write_u8(&mut cursor, self.unknown_first)?;
        writer::write_u32(&mut cursor, self.unknown_second)?;
        Ok(())
    }
}
