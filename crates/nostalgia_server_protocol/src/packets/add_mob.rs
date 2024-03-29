use crate::{interop::SyncedEntityData, reader, writer};
use std::io::{Cursor, Result};
use types::Vector3;

#[derive(Clone, Debug)]
pub struct AddMob {
    pub entity_id: i32,
    pub entity_type: i32,
    pub pos: Vector3,
    pub yaw: u8,
    pub pitch: u8,
    pub metadata: SyncedEntityData,
}

impl AddMob {
    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(Self {
            entity_id: reader::read_i32(&mut cursor)?,
            entity_type: reader::read_i32(&mut cursor)?,
            pos: reader::read_vector3(&mut cursor)?,
            yaw: reader::read_u8(&mut cursor)?,
            pitch: reader::read_u8(&mut cursor)?,
            metadata: SyncedEntityData::parse(&mut cursor)?,
        })
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        writer::write_u8(&mut cursor, 0x88)?;
        writer::write_i32(&mut cursor, self.entity_id)?;
        writer::write_i32(&mut cursor, self.entity_type)?;
        writer::write_vector3(&mut cursor, &self.pos)?;
        writer::write_u8(&mut cursor, self.yaw)?;
        writer::write_u8(&mut cursor, self.pitch)?;
        self.metadata.serialize(&mut cursor)?;
        Ok(())
    }
}
