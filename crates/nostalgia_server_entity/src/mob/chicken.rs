use crate::{id, Entity, LivingEntity, EntityFlags};
use std::io::Cursor;
use std::io::Result;

use macros::entity;
use types::Vector3;

#[entity(id = id::MOB_CHICKEN)]
#[derive(Debug, Default)]
pub struct Chicken {}

impl Chicken {
    pub fn new(id: i32) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

mod handler {
    use protocol::{writer, interop::{SyncedEntityData, EntityData}};

    use super::*;

    pub(super) fn spawn(chicken: &mut Chicken) {
        println!("Chicken {} spawned", chicken.id);
    }

    pub(super) fn despawn(chicken: &mut Chicken) {
        println!("Chicken {} despawned", chicken.id);
    }

    pub(super) fn update(chicken: &mut Chicken) {
        println!("Chicken {} updated", chicken.id);
    }

    pub(super) fn serialize(chicken: &Chicken, cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        let synced_entity_data = SyncedEntityData::from(&[
            (1, EntityData::Short(chicken.air() as i16)), // Air
            (14, EntityData::Byte(0)),
            (0, EntityData::Byte(chicken.flags() as i8)),
        ]);

        synced_entity_data.serialize(cursor)
    }

    pub(super) fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<Chicken> {
        println!("Chicken parsed");
        Ok(Chicken::default())
    }
}
