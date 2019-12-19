pub use chunk::Chunk;
pub use position::Position;
pub use world::World;

mod chunk;
mod position;
mod world;

pub type Block = u8;

pub const CHUNK_SIZE: i32 = 8;
pub const CHUNK_AREA: i32 = CHUNK_SIZE * CHUNK_SIZE;
pub const CHUNK_VOLUME: i32 = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;
