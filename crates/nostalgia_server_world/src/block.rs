#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum BlockID {
    #[default]
    Air = 0,
    Stone = 1,
    Grass = 2,
    Dirt = 3,
    Cobblestone = 4,
    WoodenPlanks = 5,
    Sapling = 6,
    Bedrock = 7,
    Water = 8,
    StillWater = 9,
    Lava = 10,
    StillLava = 11,
    Sand = 12,
    Gravel = 13,
    GoldOre = 14,
    IronOre = 15,
    CoalOre = 16,
    Wood = 17,
    Leaves = 18,
    Sponge = 19,
    Glass = 20,
    LapisOre = 21,
    LapisBlock = 22,
    Sandstone = 24,
    BedBlock = 26,
    Cobweb = 30,
    TallGrass = 31,
    DeadBush = 32,
    Wool = 35,
    Dandelion = 37,
    Flower = 38,
    BrownMushroom = 39,
    RedMushroom = 40,
    GoldBlock = 41,
    IronBlock = 42,
    DoubleSlabs = 43,
    Slab = 44,
    Bricks = 45,
    Tnt = 46,
    Bookshelf = 47,
    MossyStone = 48,
    Obsidian = 49,
    Torch = 50,
    Fire = 51,
    WoodenStairs = 53,
    Chest = 54,
    DiamondOre = 56,
    DiamondBlock = 57,
    CraftingTable = 58,
    WheatBlock = 59,
    Farmland = 60,
    Furnace = 61,
    LitFurnace = 62,
    SignPost = 63,
    WoodenDoorBlock = 64,
    Ladder = 65,
    CobblestoneStairs = 67,
    WallSign = 68,
    IronDoorBlock = 71,
    RedstoneOre = 73,
    GlowingRedstoneOre = 74,
    Snow = 78,
    Ice = 79,
    SnowBlock = 80,
    Cactus = 81,
    ClayBlock = 82,
    SugarcaneBlock = 83,
    Fence = 85,
    Pumpkin = 86,
    Netherrack = 87,
    SoulSand = 88,
    GlowStoneBlock = 89,
    JackOLantern = 91,
    CakeBlock = 92,
    Unknown = 95,
    Trapdoor = 96,
    StoneBrick = 98,
    IronBars = 101,
    GlassPane = 102,
    MelonBlock = 103,
    PumpkinStem = 104,
    MelonStem = 105,
    FenceGate = 107,
    BrickStairs = 108,
    StoneBrickStairs = 109,
    NetherBrick = 112,
    NetherBrickStairs = 114,
    SandstoneStairs = 128,
    SpruceWoodenStairs = 134,
    BirchWoodenStairs = 135,
    JungleWoodenStairs = 136,
    StoneWall = 139,
    CarrotBlock = 141,
    PotatoBlock = 142,
    QuartzBlock = 155,
    QuartzStairs = 156,
    DoubleWoodenSlab = 157,
    WoodenSlab = 158,
    HayBale = 170,
    Carpet = 171,
    CoalBlock = 173,
    BeetrootBlock = 244,
    StoneCutter = 245,
    GlowingObsidian = 246,
    NetherReactor = 247,
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
