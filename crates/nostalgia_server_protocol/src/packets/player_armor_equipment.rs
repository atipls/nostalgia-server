use crate::{reader, writer};
use std::io::{Cursor, Result};

#[derive(Clone, Debug)]
pub struct PlayerArmorEquipment {
    pub entity_id: i32,
    pub slot1: u16,
    pub slot2: u16,
    pub slot3: u16,
    pub slot4: u16,
}

impl PlayerArmorEquipment {
    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(Self {
            entity_id: reader::read_i32(&mut cursor)?,
            slot1: reader::read_u16(&mut cursor)?,
            slot2: reader::read_u16(&mut cursor)?,
            slot3: reader::read_u16(&mut cursor)?,
            slot4: reader::read_u16(&mut cursor)?,
        })
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        writer::write_u8(&mut cursor, 0xA1)?;
        writer::write_i32(&mut cursor, self.entity_id)?;
        writer::write_u16(&mut cursor, self.slot1)?;
        writer::write_u16(&mut cursor, self.slot2)?;
        writer::write_u16(&mut cursor, self.slot3)?;
        writer::write_u16(&mut cursor, self.slot4)?;
        Ok(())
    }
}

