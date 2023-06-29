#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum BlockID {
    #[default]
    Air = 0,
    Stone = 1,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Block {
    pub id: BlockID,
    pub sky_light: u8,
    pub block_light: u8,
    pub metadata: u8,
}

impl Block {
    pub fn new(id: BlockID) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    pub fn existing(id: BlockID, sky_light: u8, block_light: u8, metadata: u8) -> Self {
        Self {
            id,
            sky_light,
            block_light,
            metadata,
        }
    }
}
