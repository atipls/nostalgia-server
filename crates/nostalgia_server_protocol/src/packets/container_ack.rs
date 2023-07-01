use crate::{reader, writer};
use std::io::{Cursor, Result};

#[derive(Clone, Debug)]
pub struct ContainerAck {
    pub window_id: u8,
    pub unknown_first: u16,
    pub unknown_second: u8,
}

impl ContainerAck {
    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(Self {
            window_id: reader::read_u8(&mut cursor)?,
            unknown_first: reader::read_u16(&mut cursor)?,
            unknown_second: reader::read_u8(&mut cursor)?,
        })
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        writer::write_u8(&mut cursor, 0xB5)?;
        writer::write_u8(&mut cursor, self.window_id)?;
        writer::write_u16(&mut cursor, self.unknown_first)?;
        writer::write_u8(&mut cursor, self.unknown_second)?;
        Ok(())
    }
}
