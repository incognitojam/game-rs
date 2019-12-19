use crate::maths::i32x3;

use super::CHUNK_SIZE;

pub type Position = i32x3;

impl From<i32> for Position {
    fn from(index: i32) -> Self {
        Position {
            d0: index % CHUNK_SIZE,
            d1: index / CHUNK_SIZE,
            d2: index / (CHUNK_SIZE * CHUNK_SIZE),
        }
    }
}

impl From<&Position> for i32 {
    fn from(position: &Position) -> Self {
        position.d0
            + position.d1 * CHUNK_SIZE
            + position.d2 * CHUNK_SIZE * CHUNK_SIZE
    }
}
