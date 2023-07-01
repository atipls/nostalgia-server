use crate::{reader, writer};
use std::io::{Cursor, Result};

#[derive(Clone, Debug)]
pub struct EntityEvent {
    pub entity_id: i32,
    pub event_id: u8,
}

impl EntityEvent {
    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(Self {
            entity_id: reader::read_i32(&mut cursor)?,
            event_id: reader::read_u8(&mut cursor)?,
        })
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        writer::write_u8(&mut cursor, 0x9D)?;
        writer::write_i32(&mut cursor, self.entity_id)?;
        writer::write_u8(&mut cursor, self.event_id)?;
        Ok(())
    }
}
