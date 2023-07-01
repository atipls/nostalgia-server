use crate::{reader, writer};
use std::io::{Cursor, Result};

#[derive(Clone, Debug)]
pub struct LoginRequest {
    pub username: String,
    pub protocol_major: i32,
    pub protocol_minor: i32,
    pub client_id: u32,
    pub realms_data: String,
}

impl LoginRequest {
    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(Self {
            username: reader::read_string(&mut cursor)?,
            protocol_major: reader::read_i32(&mut cursor)?,
            protocol_minor: reader::read_i32(&mut cursor)?,
            client_id: reader::read_u32(&mut cursor)?,
            realms_data: reader::read_string(&mut cursor)?,
        })
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        writer::write_u8(&mut cursor, 0x82)?;
        writer::write_string(&mut cursor, &self.username)?;
        writer::write_i32(&mut cursor, self.protocol_major)?;
        writer::write_i32(&mut cursor, self.protocol_minor)?;
        writer::write_u32(&mut cursor, self.client_id)?;
        writer::write_string(&mut cursor, &self.realms_data)?;
        Ok(())
    }
}
