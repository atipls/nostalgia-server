use crate::{reader, writer};
use std::io::{Cursor, Result};

#[derive(Clone, Debug)]
pub struct PlayerEquipment {
    pub entity_id: i32,
    pub block: u16,
    pub meta: u16,
    pub slot: u8,
}

impl PlayerEquipment {
    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(Self {
            entity_id: reader::read_i32(&mut cursor)?,
            block: reader::read_u16(&mut cursor)?,
            meta: reader::read_u16(&mut cursor)?,
            slot: reader::read_u8(&mut cursor)?,
        })
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        writer::write_u8(&mut cursor, 0xA0)?;
        writer::write_i32(&mut cursor, self.entity_id)?;
        writer::write_u16(&mut cursor, self.block)?;
        writer::write_u16(&mut cursor, self.meta)?;
        writer::write_u8(&mut cursor, self.slot)?;
        Ok(())
    }
}

