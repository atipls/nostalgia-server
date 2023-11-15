use std::io::{Cursor, Read, Result, Write};

#[derive(Clone, Debug, Default)]
pub struct RequestedChunk {
    pub data: Vec<u8>,
}

impl RequestedChunk {
    pub fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut data = Vec::new();
        cursor.read_to_end(&mut data)?;

        Ok(Self { data })
    }

    pub fn serialize(&self, cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        cursor.write_all(&self.data)?;
        Ok(())
    }
}
