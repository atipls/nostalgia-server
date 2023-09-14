use crate::{reader, writer};
use std::io::{Cursor, Result};
use types::ItemInstance;

#[derive(Clone, Debug)]
pub struct DropItem {
    pub entity_id: i32,
    pub unk0: u8,
    pub item: ItemInstance,
}

impl DropItem {
    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(Self {
            entity_id: reader::read_i32(&mut cursor)?,
            unk0: reader::read_u8(&mut cursor)?,
            item: reader::read_item_instance(&mut cursor)?,
        })
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        writer::write_u8(&mut cursor, 0xAF)?;
        writer::write_i32(&mut cursor, self.entity_id)?;
        writer::write_u8(&mut cursor, self.unk0)?;
        writer::write_iteminstance(&mut cursor, &self.item)?;
        Ok(())
    }
}
