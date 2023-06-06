use crate::{reader, writer};
use std::io::{Cursor, Result};

#[derive(Clone, Debug)]
pub struct ContainerOpen {
    pub window_id: u8,
    pub container_type: u8,
    pub slot: u8,
    pub title: String,
}

impl ContainerOpen {
    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(Self {
            window_id: reader::read_u8(&mut cursor)?,
            container_type: reader::read_u8(&mut cursor)?,
            slot: reader::read_u8(&mut cursor)?,
            title: reader::read_string(&mut cursor)?,
        })
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        writer::write_u8(&mut cursor, 0xB0)?;
        writer::write_u8(&mut cursor, self.window_id)?;
        writer::write_u8(&mut cursor, self.container_type)?;
        writer::write_u8(&mut cursor, self.slot)?;
        writer::write_string(&mut cursor, &self.title)?;
        Ok(())
    }
}

