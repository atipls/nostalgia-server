use crate::{reader, writer};
use std::io::{Cursor, Result};
use types::ItemInstance;

#[derive(Clone, Debug)]
pub struct SendInventory {
    pub entity_id: i32,
    pub window_id: u8,
    pub items: Vec<ItemInstance>,
}

impl SendInventory {
    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(Self {
            entity_id: reader::read_i32(&mut cursor)?,
            window_id: reader::read_u8(&mut cursor)?,
            items: reader::read_item_instance_list(&mut cursor)?,
        })
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        writer::write_u8(&mut cursor, 0xAE)?;
        writer::write_i32(&mut cursor, self.entity_id)?;
        writer::write_u8(&mut cursor, self.window_id)?;
        writer::write_iteminstance_list(&mut cursor, &self.items)?;
        Ok(())
    }
}
