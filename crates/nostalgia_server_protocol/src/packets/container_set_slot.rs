use crate::{reader, writer};
use std::io::{Cursor, Result};
use types::ItemInstance;

#[derive(Clone, Debug)]
pub struct ContainerSetSlot {
    pub window_id: u8,
    pub slot: u16,
    pub item: ItemInstance,
}

impl ContainerSetSlot {
    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(Self {
            window_id: reader::read_u8(&mut cursor)?,
            slot: reader::read_u16(&mut cursor)?,
            item: reader::read_item_instance(&mut cursor)?,
        })
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        writer::write_u8(&mut cursor, 0xB2)?;
        writer::write_u8(&mut cursor, self.window_id)?;
        writer::write_u16(&mut cursor, self.slot)?;
        writer::write_iteminstance(&mut cursor, &self.item)?;
        Ok(())
    }
}
