use crate::world::Position;

pub type Block = u8;

pub const AIR: Block = 0;
pub const STONE: Block = 1;
pub const DIRT: Block = 2;
pub const GRASS: Block = 3;
pub const LOG: Block = 4;

pub struct BlockFace {
    pub normal: Position,
    pub vertices: [Position; 4],
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
    normal: Position::new(0, 1, 0),
    vertices: [
        Position::new(0, 1, 0),
        Position::new(1, 1, 0),
        Position::new(1, 0, 0),
        Position::new(0, 0, 0),
    ],
};
const EAST_FACE: BlockFace = BlockFace {
    normal: Position::new(1, 0, 0),
    vertices: [
        Position::new(1, 0, 0),
        Position::new(1, 1, 0),
        Position::new(1, 1, 1),
        Position::new(1, 0, 1),
    ],
};
const SOUTH_FACE: BlockFace = BlockFace {
    normal: Position::new(0, -1, 0),
    vertices: [
        Position::new(0, 0, 0),
        Position::new(1, 0, 0),
        Position::new(1, 0, 1),
        Position::new(0, 0, 1),
    ],
};
const WEST_FACE: BlockFace = BlockFace {
    normal: Position::new(-1, 0, 0),
    vertices: [Position::new(0, 1, 0),
        Position::new(0, 0, 0),
        Position::new(0, 0, 1),
        Position::new(0, 1, 1),
    ],
};
const TOP_FACE: BlockFace = BlockFace {
    normal: Position::new(0, 0, 1),
    vertices: [Position::new(0, 0, 1),
        Position::new(1, 0, 1),
        Position::new(1, 1, 1),
        Position::new(0, 1, 1),
    ],
};
const BOTTOM_FACE: BlockFace = BlockFace {
    normal: Position::new(0, 0, -1),
    vertices: [
        Position::new(1, 1, 0),
        Position::new(0, 1, 0),
        Position::new(0, 1, 1),
        Position::new(1, 1, 1),
    ],
};
