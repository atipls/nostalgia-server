use crate::{reader, writer};
use std::collections::HashMap;
use std::fmt::Debug;
use std::io::{Cursor, Result};
use types::{ItemInstance, Vector3};

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct EntityDataIndex(u8);

const ENTITY_DATA_INDEX_NAMES: [&str; 19] = [
    "Flags",
    "AirTicks",
    "Unknown2",
    "Unknown3",
    "Unknown4",
    "Unknown5",
    "Unknown6",
    "Unknown7",
    "Unknown8",
    "Unknown9",
    "Unknown10",
    "Unknown11",
    "Unknown12",
    "Unknown13",
    "IsBaby",
    "Unknown15",
    "IsSleepingOrShearInfo",
    "SleepPosition",
    "Unknown18",
];

fn get_entity_data_index_name(EntityDataIndex(index): &EntityDataIndex) -> &'static str {
    ENTITY_DATA_INDEX_NAMES
        .get(*index as usize)
        .copied()
        .unwrap_or("Unknown")
}

impl Debug for EntityDataIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "EntityDataIndex({}, {})",
            self.0,
            get_entity_data_index_name(self)
        ))
    }
}

#[derive(Clone, Debug)]
pub enum EntityData {
    Byte(i8),
    Short(i16),
    Int(i32),
    Float(f32),
    String(String),
    ItemInstance(ItemInstance),
    Position(Vector3),
}

#[derive(Clone, Debug, Default)]
struct MarkedEntityData {
    value: Option<EntityData>,
    dirty: bool,
}

impl MarkedEntityData {
    pub fn new(value: EntityData) -> Self {
        Self {
            value: Some(value),
            dirty: false,
        }
    }

    pub fn set(&mut self, value: EntityData) {
        self.value = Some(value);
        self.dirty = true;
    }
}

#[derive(Clone, Debug, Default)]
pub struct SyncedEntityData {
    data: HashMap<EntityDataIndex, MarkedEntityData>,
}

impl SyncedEntityData {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn from(items: &[(u8, EntityData)]) -> Self {
        let mut data = HashMap::new();
        for (index, value) in items {
            data.insert(
                EntityDataIndex(*index),
                MarkedEntityData::new(value.clone()),
            );
        }

        Self { data }
    }

    pub fn get(&self, index: u8) -> Option<&EntityData> {
        self.data
            .get(&EntityDataIndex(index))
            .and_then(|data| data.value.as_ref())
    }

    pub fn set(&mut self, index: u8, value: EntityData) {
        if let Some(data) = self.data.get_mut(&EntityDataIndex(index)) {
            data.set(value);
        } else {
            self.data
                .insert(EntityDataIndex(index), MarkedEntityData::new(value));
        }
    }

    pub fn parse(mut cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut data = HashMap::new();
        loop {
            let maybe_index_and_type = reader::read_u8(&mut cursor)?;
            if maybe_index_and_type == 0x7F {
                break;
            }

            let item_index = EntityDataIndex(maybe_index_and_type & 0x1F);
            let item_type = maybe_index_and_type >> 5;
            let value = match item_type {
                0 => EntityData::Byte(reader::read_i8(&mut cursor)?),
                1 => EntityData::Short(reader::read_i16(&mut cursor)?),
                2 => EntityData::Int(reader::read_i32(&mut cursor)?),
                3 => EntityData::Float(reader::read_f32(&mut cursor)?),
                4 => EntityData::String(reader::read_string(&mut cursor)?),
                5 => EntityData::ItemInstance(reader::read_item_instance(&mut cursor)?),
                6 => EntityData::Position(reader::read_vector3(&mut cursor)?),
                _ => panic!("Invalid entity data type"),
            };

            data.insert(item_index, MarkedEntityData::new(value));
        }

        Ok(Self { data })
    }

    pub fn serialize(&self, mut cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        for (index, data) in &self.data {
            let Some(value) = &data.value else {
                continue;
            };

            let item_type = match value {
                EntityData::Byte(_) => 0,
                EntityData::Short(_) => 1,
                EntityData::Int(_) => 2,
                EntityData::Float(_) => 3,
                EntityData::String(_) => 4,
                EntityData::ItemInstance(_) => 5,
                EntityData::Position(_) => 6,
            };

            writer::write_u8(&mut cursor, index.0 | (item_type << 5))?;

            match value {
                EntityData::Byte(value) => writer::write_i8(&mut cursor, *value)?,
                EntityData::Short(value) => writer::write_i16(&mut cursor, *value)?,
                EntityData::Int(value) => writer::write_i32(&mut cursor, *value)?,
                EntityData::Float(value) => writer::write_f32(&mut cursor, *value)?,
                EntityData::String(value) => writer::write_string(&mut cursor, value)?,
                EntityData::ItemInstance(value) => writer::write_iteminstance(&mut cursor, value)?,
                EntityData::Position(value) => writer::write_vector3(&mut cursor, value)?,
            }
        }

        writer::write_u8(&mut cursor, 0x7F)
    }
}
