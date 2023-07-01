use crate::{reader, writer};
use std::io::{Cursor, Result};

#[derive(Clone, Debug)]
pub struct ContainerSetData {
    pub window_id: u8,
    pub property: u16,
    pub value: u16,
}

impl ContainerSetData {
    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(Self {
            window_id: reader::read_u8(&mut cursor)?,
            property: reader::read_u16(&mut cursor)?,
            value: reader::read_u16(&mut cursor)?,
        })
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        writer::write_u8(&mut cursor, 0xB3)?;
        writer::write_u8(&mut cursor, self.window_id)?;
        writer::write_u16(&mut cursor, self.property)?;
        writer::write_u16(&mut cursor, self.value)?;
        Ok(())
    }
}
