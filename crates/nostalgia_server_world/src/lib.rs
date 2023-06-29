mod block;
mod chunk;

use std::{
    io::{self, Cursor, Error},
    path::PathBuf,
};

pub use block::*;
use byteorder::{LittleEndian, ReadBytesExt};
pub use chunk::*;

struct World {
    chunks: Vec<Chunk>,
    name: String,
    // entities: Vec<Entity>,
}

/*

public static int[,] ReadChunkMetadata(BinaryReader reader) {
    var metadata = new int[16, 16];
    for (var offset = 0; offset < SectorSize; offset += 4) {
        var chunkMetadata = reader.ReadInt32();
        if (chunkMetadata == 0)
            continue;

        var x = (offset >> 2) % 32;
        var z = (offset >> 2) / 32;

        metadata[x, z] = (chunkMetadata >> 8) * SectorSize;
    }

    return metadata;
}
*/

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

    fn from_file(path: PathBuf) -> io::Result<Self> {
        let mut cursor = Cursor::new(std::fs::read(path).unwrap());

        todo!("pls impl")
    }
}
