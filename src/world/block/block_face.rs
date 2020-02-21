use crate::world::{Direction, Position};

pub struct BlockFace {
    pub direction: Direction,
    pub normal: Position,
    pub vertices: [Position; 4],
}

pub const BLOCK_FACES: [BlockFace; 6] = [
    NORTH,
    EAST,
    SOUTH,
    WEST,
    TOP,
    BOTTOM,
];

const NORTH: BlockFace = BlockFace {
    direction: Direction::North,
    normal: Position::new(0, 1, 0),
    vertices: [
        Position::new(0, 1, 0),
        Position::new(1, 1, 0),
        Position::new(1, 0, 0),
        Position::new(0, 0, 0),
    ],
};
const EAST: BlockFace = BlockFace {
    direction: Direction::East,
    normal: Position::new(1, 0, 0),
    vertices: [
        Position::new(1, 0, 0),
        Position::new(1, 1, 0),
        Position::new(1, 1, 1),
        Position::new(1, 0, 1),
    ],
};
const SOUTH: BlockFace = BlockFace {
    direction: Direction::South,
    normal: Position::new(0, -1, 0),
    vertices: [
        Position::new(0, 0, 0),
        Position::new(1, 0, 0),
        Position::new(1, 0, 1),
        Position::new(0, 0, 1),
    ],
};
const WEST: BlockFace = BlockFace {
    direction: Direction::West,
    normal: Position::new(-1, 0, 0),
    vertices: [Position::new(0, 1, 0),
        Position::new(0, 0, 0),
        Position::new(0, 0, 1),
        Position::new(0, 1, 1),
    ],
};
const TOP: BlockFace = BlockFace {
    direction: Direction::Top,
    normal: Position::new(0, 0, 1),
    vertices: [Position::new(0, 0, 1),
        Position::new(1, 0, 1),
        Position::new(1, 1, 1),
        Position::new(0, 1, 1),
    ],
};
const BOTTOM: BlockFace = BlockFace {
    direction: Direction::Bottom,
    normal: Position::new(0, 0, -1),
    vertices: [
        Position::new(1, 1, 0),
        Position::new(0, 1, 0),
        Position::new(0, 1, 1),
        Position::new(1, 1, 1),
    ],
};
