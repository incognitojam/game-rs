use std::ops::Add;

use crate::render_gl::data::f32_f32_f32;
use crate::world::CHUNK_SIZE;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Position {
    pub const fn new(x: i64, y: i64, z: i64) -> Position {
        Position {
            x,
            y,
            z,
        }
    }

    pub const fn add(&self, other: &Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl From<Position> for f32_f32_f32 {
    fn from(other: Position) -> Self {
        f32_f32_f32::new(other.x as f32, other.y as f32, other.z as f32)
    }
}

impl Into<(i64, i64, i64)> for Position {
    fn into(self) -> (i64, i64, i64) {
        (self.x, self.y, self.z)
    }
}

impl From<(i64, i64, i64)> for Position {
    fn from(other: (i64, i64, i64)) -> Self {
        Position::new(other.0, other.1, other.2)
    }
}

impl From<i64> for Position {
    fn from(index: i64) -> Self {
        Position {
            x: index % CHUNK_SIZE,
            y: index / CHUNK_SIZE,
            z: index / (CHUNK_SIZE * CHUNK_SIZE),
        }
    }
}

impl From<&Position> for i64 {
    fn from(position: &Position) -> Self {
        position.x
            + position.y * CHUNK_SIZE
            + position.z * CHUNK_SIZE * CHUNK_SIZE
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Self::Output {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
