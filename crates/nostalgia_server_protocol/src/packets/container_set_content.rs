use crate::{reader, writer};
use std::io::{Cursor, Read, Result};
use types::ItemInstance;

#[derive(Clone, Debug)]
pub struct ContainerSetContent {
    pub window_id: u8,
    pub items: Vec<ItemInstance>,
    pub unknown: Vec<u8>,
}

impl ContainerSetContent {
    fn read_unknown(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Vec<u8>> {
        let length = reader::read_u16(&mut cursor)? as usize * 4;
        let mut unknown = vec![0; length];
        cursor.read_exact(&mut unknown)?;
        Ok(unknown)
    }

    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(Self {
            window_id: reader::read_u8(&mut cursor)?,
            items: reader::read_item_instance_list(&mut cursor)?,
            unknown: Self::read_unknown(&mut cursor)?,
        })
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        writer::write_u8(&mut cursor, 0xB4)?;
        writer::write_u8(&mut cursor, self.window_id)?;
        writer::write_iteminstance_list(&mut cursor, &self.items)?;
        Ok(())
    }
}
