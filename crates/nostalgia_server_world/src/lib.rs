mod block;
mod chunk;

use std::{
    io::{self, Cursor, Error, Seek},
    path::PathBuf,
};

pub use block::*;
pub use chunk::*;
use nbt::{Nbt, Tag};

use byteorder::{LittleEndian, ReadBytesExt};

pub struct World {
    pub name: String,
    pub spawn_mobs: bool,
    pub seed: i64,
    pub time: i64,
    pub spawn_position: (i32, i32, i32),
    pub platform: i32,
    pub game_type: i32,
    pub storage_version: i32,
    pub day_cycle_stop_time: i64,
    pub last_played: i64,

    pub chunks: Vec<Chunk>,
    pub entities: Vec<Tag>,
}

impl World {
    fn read_chunk_metadata(cursor: &mut Cursor<Vec<u8>>) -> io::Result<Vec<Vec<u32>>> {
        let mut metadata = vec![vec![0u32; 16]; 16];
        for offset in (0..4096).step_by(4) {
            let chunk_metadata = cursor.read_u32::<LittleEndian>()?;
            if chunk_metadata == 0 {
                continue;
            }

            let x = (offset >> 2) % 32;
            let z = (offset >> 2) / 32;
            metadata[x as usize][z as usize] = (chunk_metadata >> 8) * 4096;
        }

        Ok(metadata)
    }

    fn read_level_data(buffer: Vec<u8>) -> io::Result<Nbt> {
        let mut cursor = Cursor::new(buffer);
        if cursor.read_i32::<LittleEndian>()? != 3 {
            return Err(Error::new(
                io::ErrorKind::InvalidData,
                "Invalid level.dat version",
            ));
        }

        // File length
        cursor.read_i32::<LittleEndian>()?;

        Nbt::from_bytes(&mut cursor)
    }

    fn read_entity_data(buffer: Vec<u8>) -> io::Result<Nbt> {
        let mut cursor = Cursor::new(buffer);

        cursor.seek(io::SeekFrom::Start(12))?; // TODO, header

        Nbt::from_bytes(&mut cursor)
    }

    pub fn from_file(path: PathBuf) -> io::Result<Self> {
        macro_rules! not_found {
            ($name: ident) => {
                Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("{} not found", stringify!($name)),
                )
            };
        }

        let level = std::fs::read(path.join("level.dat"))?;
        let entities = std::fs::read(path.join("entities.dat"))?;

        let level = Self::read_level_data(level)?;
        let entities = Self::read_entity_data(entities)?;

        let level_root = level.root();
        let entities_root = entities.root();

        let chunks = std::fs::read(path.join("chunks.dat"))?;
        let mut chunks = Cursor::new(chunks);
        let chunk_metadata = Self::read_chunk_metadata(&mut chunks)?;

        let mut chunk_list = Vec::new();
        for x in 0..16 {
            for z in 0..16 {
                let offset = chunk_metadata[x][z] as usize;
                if offset == 0 {
                    continue;
                }

                chunks.seek(io::SeekFrom::Start(offset as u64))?;
                let chunk = Chunk::from_bytes(&mut chunks)?;
                chunk_list.push(chunk);
            }
        }

        Ok(Self {
            name: level_root
                .get_string("LevelName")
                .ok_or_else(|| not_found!(LevelName))?
                .clone(),
            spawn_mobs: *level_root
                .get_byte("spawnMobs")
                .ok_or_else(|| not_found!(SpawnMobs))?
                != 0,
            seed: *level_root
                .get_long("RandomSeed")
                .ok_or_else(|| not_found!(RandomSeed))?,
            time: *level_root
                .get_long("Time")
                .ok_or_else(|| not_found!(Time))?,
            spawn_position: (
                *level_root
                    .get_int("SpawnX")
                    .ok_or_else(|| not_found!(SpawnX))?,
                *level_root
                    .get_int("SpawnY")
                    .ok_or_else(|| not_found!(SpawnY))?,
                *level_root
                    .get_int("SpawnZ")
                    .ok_or_else(|| not_found!(SpawnZ))?,
            ),
            platform: *level_root
                .get_int("Platform")
                .ok_or_else(|| not_found!(Platform))?,
            game_type: *level_root
                .get_int("GameType")
                .ok_or_else(|| not_found!(GameType))?,
            storage_version: *level_root
                .get_int("StorageVersion")
                .ok_or_else(|| not_found!(StorageVersion))?,
            day_cycle_stop_time: *level_root
                .get_long("dayCycleStopTime")
                .ok_or_else(|| not_found!(dayCycleStopTime))?,
            last_played: *level_root
                .get_long("LastPlayed")
                .ok_or_else(|| not_found!(LastPlayed))?,
            chunks: chunk_list,
            entities: entities_root
                .get_list("Entities")
                .ok_or_else(|| not_found!(Entities))?
                .clone(),
        })
    }
}
