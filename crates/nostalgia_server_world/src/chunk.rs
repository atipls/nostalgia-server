use byteorder::{LittleEndian, ReadBytesExt};

use crate::{Block, BlockID};
use std::io::{Cursor, Error, ErrorKind, Read, Result};

pub const CHUNK_SIZE_X: usize = 16;
pub const CHUNK_SIZE_Z: usize = 16;
pub const CHUNK_SIZE_Y: usize = 128;

#[derive(Clone, Debug)]
pub struct Chunk {
    pub blocks: [Block; CHUNK_SIZE_Y * CHUNK_SIZE_Z * CHUNK_SIZE_X],
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            blocks: [Block::new(BlockID::Air); CHUNK_SIZE_Y * CHUNK_SIZE_Z * CHUNK_SIZE_X],
        }
    }

    pub fn get_block_index(x: usize, y: usize, z: usize) -> usize {
        // +1 in y => offset increases by 1
        // +1 in z => offset increases by 16
        // +1 in x => offset increases by 256
        y + x * CHUNK_SIZE_Y + z * CHUNK_SIZE_Y * CHUNK_SIZE_Z
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

        const SLICE_SIZE: usize = CHUNK_SIZE_X * CHUNK_SIZE_Y * CHUNK_SIZE_Z;
        let mut block_data = [0u8; SLICE_SIZE];
        let mut block_metadata = [0u8; SLICE_SIZE];
        let mut sky_light = [0u8; SLICE_SIZE];
        let mut block_light = [0u8; SLICE_SIZE];

        block_data.copy_from_slice(&chunk_buffer[0..SLICE_SIZE]);
        Self::decompress_block_metadata(
            &chunk_buffer[SLICE_SIZE..(SLICE_SIZE + SLICE_SIZE / 2)],
            &mut block_metadata,
        );
        Self::decompress_block_metadata(
            &chunk_buffer[(SLICE_SIZE + SLICE_SIZE / 2)..(SLICE_SIZE + SLICE_SIZE)],
            &mut sky_light,
        );
        Self::decompress_block_metadata(
            &chunk_buffer[(SLICE_SIZE + SLICE_SIZE)..(SLICE_SIZE + SLICE_SIZE + SLICE_SIZE / 2)],
            &mut block_light,
        );

        for x in 0..CHUNK_SIZE_Z {
            for z in 0..CHUNK_SIZE_X {
                for y in 0..CHUNK_SIZE_Y {
                    let block_index = Self::get_block_index(x, y, z);
                    chunk.blocks[block_index] = Block::existing(
                        unsafe { std::mem::transmute::<u8, BlockID>(block_data[block_index]) },
                        sky_light[block_index],
                        block_light[block_index],
                        block_metadata[block_index],
                    );
                }
            }
        }

        Ok(chunk)
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> Block {
        let block_index = Self::get_block_index(x, y, z);
        self.blocks[block_index]
    }

    pub fn get_ref(&self, x: usize, y: usize, z: usize) -> &Block {
        let block_index = Self::get_block_index(x, y, z);
        &self.blocks[block_index]
    }

    pub fn get_mut(&mut self, x: usize, y: usize, z: usize) -> &mut Block {
        let block_index = Self::get_block_index(x, y, z);
        &mut self.blocks[block_index]
    }
}
