use byteorder::{LittleEndian, ReadBytesExt};

use crate::{Block, BlockID};
use std::{
    io::{Cursor, Error, ErrorKind, Read, Result},
    ops::{Index, IndexMut},
};

pub const CHUNK_SIZE_X: usize = 16;
pub const CHUNK_SIZE_Z: usize = 16;
pub const CHUNK_SIZE_Y: usize = 128;

pub struct Chunk {
    pub blocks: [[[Block; CHUNK_SIZE_Z]; CHUNK_SIZE_X]; CHUNK_SIZE_Y],
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            blocks: [[[Block::default(); CHUNK_SIZE_Z]; CHUNK_SIZE_X]; CHUNK_SIZE_Y],
        }
    }

    fn decompress_block_metadata(buffer: &[u8], destination: &mut [u8]) {
        let mut output_buffer = [0u8; 32768];
        for offset in (0..output_buffer.len()).step_by(2) {
            let input_byte = buffer[offset / 2];
            output_buffer[offset] = input_byte & 0x0F;
            output_buffer[offset + 1] = input_byte >> 4;
        }

        destination.copy_from_slice(&output_buffer);
    }

    pub fn from_bytes(cursor: &mut Cursor<Vec<u8>>) -> Result<Self> {
        if cursor.read_u32::<LittleEndian>()? != 82180 {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid chunk header"));
        }

        let mut chunk_buffer = [0u8; 82176];
        cursor.read_exact(&mut chunk_buffer)?;

        let mut chunk = Self::new();

        let mut block_data = [0u8; 32768];
        let mut block_metadata = [0u8; 32768];
        let mut sky_light = [0u8; 32768];
        let mut block_light = [0u8; 32768];

        block_data.copy_from_slice(&chunk_buffer[0..32768]);
        Self::decompress_block_metadata(&chunk_buffer[32768..49152], &mut block_metadata);
        Self::decompress_block_metadata(&chunk_buffer[49152..65536], &mut sky_light);
        Self::decompress_block_metadata(&chunk_buffer[65536..81920], &mut block_light);

        for y in 0..CHUNK_SIZE_Y {
            for x in 0..CHUNK_SIZE_X {
                for z in 0..CHUNK_SIZE_Z {
                    let block_index = y * CHUNK_SIZE_X * CHUNK_SIZE_Z + z * CHUNK_SIZE_X + x;
                    chunk.blocks[y][x][z] = Block::existing(
                        unsafe { std::mem::transmute::<u8, BlockID>(block_data[block_index]) },
                        block_metadata[block_index],
                        sky_light[block_index],
                        block_light[block_index],
                    );
                }
            }
        }

        Ok(chunk)
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> Block {
        self.blocks[y][x][z]
    }

    pub fn get_ref(&self, x: usize, y: usize, z: usize) -> &Block {
        &self.blocks[y][x][z]
    }

    pub fn get_mut(&mut self, x: usize, y: usize, z: usize) -> &mut Block {
        &mut self.blocks[y][x][z]
    }
}
