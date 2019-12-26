pub use chunk::Chunk;
pub use position::Position;
pub use world::World;

pub mod block;
mod chunk;
mod position;
mod world;

pub const CHUNK_SIZE: i64 = 16;
pub const CHUNK_AREA: i64 = CHUNK_SIZE * CHUNK_SIZE;
pub const CHUNK_VOLUME: i64 = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;
