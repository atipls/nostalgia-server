use crate::{reader, writer};
use std::io::{Cursor, Result};

#[derive(Clone, Debug)]
pub struct HurtArmor {
    pub armor: u8,
}

impl HurtArmor {
    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(Self {
            armor: reader::read_u8(&mut cursor)?,
        })
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        writer::write_u8(&mut cursor, 0xA6)?;
        writer::write_u8(&mut cursor, self.armor)?;
        Ok(())
    }
}
