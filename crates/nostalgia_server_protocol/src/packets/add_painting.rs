use crate::{reader, writer};
use std::io::{Cursor, Result};

#[derive(Clone, Debug)]
pub struct AddPainting {
    pub entity_id: i32,
    pub x: i32,
    pub y: i32,
    pub direction: i32,
    pub title: String,
}

impl AddPainting {
    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(Self {
            entity_id: reader::read_i32(&mut cursor)?,
            x: reader::read_i32(&mut cursor)?,
            y: reader::read_i32(&mut cursor)?,
            direction: reader::read_i32(&mut cursor)?,
            title: reader::read_string(&mut cursor)?,
        })
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        writer::write_u8(&mut cursor, 0x99)?;
        writer::write_i32(&mut cursor, self.entity_id)?;
        writer::write_i32(&mut cursor, self.x)?;
        writer::write_i32(&mut cursor, self.y)?;
        writer::write_i32(&mut cursor, self.direction)?;
        writer::write_string(&mut cursor, &self.title)?;
        Ok(())
    }
}

