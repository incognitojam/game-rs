pub type Block = u8;

pub struct BlockFace {
    pub normal: (f32, f32, f32),
    pub vertices: [(i64, i64, i64); 4],
}

pub const BLOCK_FACES: [BlockFace; 6] = [
    NORTH_FACE,
    EAST_FACE,
    SOUTH_FACE,
    WEST_FACE,
    TOP_FACE,
    BOTTOM_FACE,
];

const NORTH_FACE: BlockFace = BlockFace {
    normal: (0.0, 1.0, 0.0),
    vertices: [(0, 1, 0), (1, 1, 0), (1, 0, 0), (0, 0, 0)],
};
const EAST_FACE: BlockFace = BlockFace {
    normal: (1.0, 0.0, 0.0),
    vertices: [(1, 0, 0), (1, 1, 0), (1, 1, 1), (1, 0, 1)],
};
const SOUTH_FACE: BlockFace = BlockFace {
    normal: (0.0, -1.0, 0.0),
    vertices: [(0, 0, 0), (1, 0, 0), (1, 0, 1), (0, 0, 1)],
};
const WEST_FACE: BlockFace = BlockFace {
    normal: (-1.0, 0.0, 0.0),
    vertices: [(0, 1, 0), (0, 0, 0), (0, 0, 1), (0, 1, 1)],
};
const TOP_FACE: BlockFace = BlockFace {
    normal: (0.0, 0.0, 1.0),
    vertices: [(0, 0, 1), (1, 0, 1), (1, 1, 1), (0, 1, 1)],
};
const BOTTOM_FACE: BlockFace = BlockFace {
    normal: (0.0, 0.0, -1.0),
    vertices: [(1, 1, 0), (0, 1, 0), (0, 1, 1), (1, 1, 1)],
};
