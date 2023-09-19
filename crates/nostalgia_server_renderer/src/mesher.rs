use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

use crate::uv::get_uv_coords_for_block_id;
use block_mesh::{
    ndshape::{ConstShape, ConstShape3u32},
    visible_block_faces, AxisPermutation, MergeVoxel, OrientedBlockFace, QuadCoordinateConfig,
    UnitQuadBuffer, Voxel, VoxelVisibility,
};
use world::BlockID;

pub const MINECRAFT_COORDINATE_CONFIG: QuadCoordinateConfig = QuadCoordinateConfig {
    faces: [
        OrientedBlockFace::new(-1, AxisPermutation::Xzy), // -X
        OrientedBlockFace::new(1, AxisPermutation::Xzy),  // +X
        OrientedBlockFace::new(-1, AxisPermutation::Yzx), // -Y
        OrientedBlockFace::new(1, AxisPermutation::Yzx),  // +Y
        OrientedBlockFace::new(-1, AxisPermutation::Zxy), // -Z
        OrientedBlockFace::new(1, AxisPermutation::Zxy),  // +Z
    ],
    u_flip_face: block_mesh::Axis::X,
};

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
struct Block(world::Block);

impl Voxel for Block {
    fn get_visibility(&self) -> VoxelVisibility {
        const TRANSPARENT_BLOCKS: &[BlockID] = &[
            BlockID::Water,
            BlockID::StillWater,
            BlockID::Torch,
            BlockID::Leaves,
            BlockID::PotatoBlock,
            BlockID::WheatBlock,
            BlockID::BrownMushroom,
            BlockID::RedMushroom,
            BlockID::SignPost,
            BlockID::DeadBush,
            BlockID::Flower,
            BlockID::Dandelion,
            BlockID::BeetrootBlock,
            BlockID::CarrotBlock,
            BlockID::Cactus,
        ];

        if TRANSPARENT_BLOCKS.contains(&self.0.id) {
            VoxelVisibility::Translucent
        } else if self.0.id == BlockID::Air {
            VoxelVisibility::Empty
        } else {
            VoxelVisibility::Opaque
        }
    }
}

impl MergeVoxel for Block {
    type MergeValue = BlockID;

    fn merge_value(&self) -> Self::MergeValue {
        self.0.id
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.0.id == other.0.id
    }
}

pub fn build_cube_meshes(chunk: &world::Chunk, corner_chunks: [Option<&world::Chunk>; 4]) -> Mesh {
    type ChunkShape = ConstShape3u32<18, 130, 18>;

    let mut chunk_data = vec![Block(world::Block::new(BlockID::Air)); 18 * 130 * 18];
    // Fill in the main chunk data
    for x in 0..16 {
        for y in 0..128 {
            for z in 0..16 {
                // +1 to account for the blocks that are only used for occlusion tests
                let index = ChunkShape::linearize([x + 1, y, z + 1]);
                let block = Block(chunk.get(x as usize, y as usize, z as usize));
                chunk_data[index as usize] = block;
            }
        }
    }

    // Fill in the adjacent chunk data, only 1 block thick, for occlusion tests

    // -X
    if let Some(corner_chunk) = corner_chunks[0] {
        for y in 0..128 {
            for z in 0..16 {
                let index = ChunkShape::linearize([0, y, z + 1]);
                let block = Block(corner_chunk.get(15, y as usize, z as usize));
                chunk_data[index as usize] = block;
            }
        }
    }

    // +X
    if let Some(corner_chunk) = corner_chunks[1] {
        for y in 0..128 {
            for z in 0..16 {
                let index = ChunkShape::linearize([17, y, z + 1]);
                let block = Block(corner_chunk.get(0, y as usize, z as usize));
                chunk_data[index as usize] = block;
            }
        }
    }

    // -Z
    if let Some(corner_chunk) = corner_chunks[2] {
        for y in 0..128 {
            for x in 0..16 {
                let index = ChunkShape::linearize([x + 1, y, 0]);
                let block = Block(corner_chunk.get(x as usize, y as usize, 15));
                chunk_data[index as usize] = block;
            }
        }
    }

    // +Z
    if let Some(corner_chunk) = corner_chunks[3] {
        for y in 0..128 {
            for x in 0..16 {
                let index = ChunkShape::linearize([x + 1, y, 17]);
                let block = Block(corner_chunk.get(x as usize, y as usize, 0));
                chunk_data[index as usize] = block;
            }
        }
    }

    let mut buffer = UnitQuadBuffer::new();
    visible_block_faces(
        &chunk_data,
        &ChunkShape {},
        [0; 3],
        [17, 129, 17],
        &MINECRAFT_COORDINATE_CONFIG.faces,
        &mut buffer,
    );

    let num_indices = buffer.num_quads() * 6;
    let num_vertices = buffer.num_quads() * 4;
    let mut indices = Vec::with_capacity(num_indices);
    let mut positions = Vec::with_capacity(num_vertices);
    let mut normals = Vec::with_capacity(num_vertices);
    let mut tex_coords = Vec::with_capacity(num_vertices);

    for (group, face) in buffer
        .groups
        .into_iter()
        .zip(MINECRAFT_COORDINATE_CONFIG.faces.into_iter())
    {
        for quad in group.into_iter() {
            indices.extend_from_slice(&face.quad_mesh_indices(positions.len() as u32));
            positions.extend_from_slice(&face.quad_mesh_positions(&quad.into(), 1.0));
            normals.extend_from_slice(&face.quad_mesh_normals());

            let block = &chunk_data[ChunkShape::linearize(quad.minimum) as usize];
            let minecraft_uv_coords = get_uv_coords_for_block_id(block.0.id);

            let u0 = ((minecraft_uv_coords[0] * 512.0).round() + 0.2) / 512.0;
            let v0 = ((minecraft_uv_coords[1] * 256.0).round() + 0.2) / 256.0;
            let u1 = ((minecraft_uv_coords[2] * 512.0).round() - 0.2) / 512.0;
            let v1 = ((minecraft_uv_coords[3] * 256.0).round() - 0.2) / 256.0;

            tex_coords.extend_from_slice(&[[u0, v0], [u0, v1], [u1, v0], [u1, v1]]);
        }
    }

    let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, tex_coords);
    render_mesh.set_indices(Some(Indices::U32(indices.clone())));

    render_mesh
}
