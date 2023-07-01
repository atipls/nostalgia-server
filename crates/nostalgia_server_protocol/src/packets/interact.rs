use crate::{reader, writer};
use std::io::{Cursor, Result};

#[derive(Clone, Debug)]
pub struct Interact {
    pub action: u8,
    pub entity_id: i32,
    pub target_id: i32,
}

impl Interact {
    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(Self {
            action: reader::read_u8(&mut cursor)?,
            entity_id: reader::read_i32(&mut cursor)?,
            target_id: reader::read_i32(&mut cursor)?,
        })
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        writer::write_u8(&mut cursor, 0xA2)?;
        writer::write_u8(&mut cursor, self.action)?;
        writer::write_i32(&mut cursor, self.entity_id)?;
        writer::write_i32(&mut cursor, self.target_id)?;
        Ok(())
    }
}
