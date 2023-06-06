use crate::{reader, writer};
use std::io::{Cursor, Result};

#[derive(Clone, Debug)]
pub struct TakeItemEntity {
    pub target: i32,
    pub entity_id: i32,
}

impl TakeItemEntity {
    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(Self {
            target: reader::read_i32(&mut cursor)?,
            entity_id: reader::read_i32(&mut cursor)?,
        })
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        writer::write_u8(&mut cursor, 0x8F)?;
        writer::write_i32(&mut cursor, self.target)?;
        writer::write_i32(&mut cursor, self.entity_id)?;
        Ok(())
    }
}

