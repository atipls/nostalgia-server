use crate::{reader, writer};
use std::io::{Cursor, Result};

#[derive(Clone, Debug)]
pub struct RemovePlayer {
    pub entity_id: i32,
    pub player_id: u64,
}

impl RemovePlayer {
    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(Self {
            entity_id: reader::read_i32(&mut cursor)?,
            player_id: reader::read_u64(&mut cursor)?,
        })
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        writer::write_u8(&mut cursor, 0x8A)?;
        writer::write_i32(&mut cursor, self.entity_id)?;
        writer::write_u64(&mut cursor, self.player_id)?;
        Ok(())
    }
}
