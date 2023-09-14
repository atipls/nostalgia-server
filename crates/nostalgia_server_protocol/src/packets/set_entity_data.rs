use crate::interop::SyncedEntityData;
use crate::{reader, writer};
use std::io::{Cursor, Result};

#[derive(Clone, Debug)]
pub struct SetEntityData {
    pub entity_id: i32,
    pub metadata: SyncedEntityData,
}

impl SetEntityData {
    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(Self {
            entity_id: reader::read_i32(&mut cursor)?,
            metadata: SyncedEntityData::parse(&mut cursor)?,
        })
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        writer::write_u8(&mut cursor, 0xA7)?;
        writer::write_i32(&mut cursor, self.entity_id)?;
        self.metadata.serialize(&mut cursor)?;
        Ok(())
    }
}
