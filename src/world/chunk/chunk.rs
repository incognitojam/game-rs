use std::ops::{Index, IndexMut};

use crate::render_gl::Texture;
use crate::world::{CHUNK_SIZE, CHUNK_VOLUME, Direction, Position};
use crate::world::block::{self, Block, BLOCK_FACES, BlockFace};
use crate::world::light::{self, LightLevel};

use super::chunk_mesh::ChunkMesh;

pub struct Chunk {
    pub position: Position,
    block_data: ChunkData<Block>,
    light_data: ChunkData<LightLevel>,
    mesh: ChunkMesh,
    mesh_invalidated: bool,
}

struct ChunkData<T> {
    data: [T; CHUNK_VOLUME as usize],
}

impl<T: Copy> ChunkData<T> {
    pub fn new(default: T) -> ChunkData<T> {
        ChunkData {
            data: [default; CHUNK_VOLUME as usize],
        }
    }
}

impl<T> Index<Position> for ChunkData<T> {
    type Output = T;

    fn index(&self, index: Position) -> &Self::Output {
        unsafe {
            self.data.get_unchecked(i64::from(&index) as usize)
        }
    }
}

impl<T> IndexMut<Position> for ChunkData<T> {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        unsafe {
            self.data.get_unchecked_mut(i64::from(&index) as usize)
        }
    }
}

impl Chunk {
    pub fn new(position: Position, gl: &gl::Gl, texture: &Texture) -> Result<Chunk, failure::Error> {
        let mut chunk = Chunk {
            position,
            block_data: ChunkData::new(block::material::STONE),
            light_data: ChunkData::new(16),
            mesh: ChunkMesh::new(gl, texture),
            mesh_invalidated: true,
        };

        Chunk::generate_blocks(&mut chunk.block_data);
        Chunk::calculate_lighting(&chunk.block_data, &mut chunk.light_data);

        Ok(chunk)
    }

    fn generate_blocks(block_data: &mut ChunkData<Block>) {
        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    let block_position = Position::new(x, y, z);
                    let block = match z {
                        0 ..= 11 => block::material::STONE,
                        12 ..= 14 => block::material::DIRT,
                        _ => block::material::GRASS
                    };
                    block_data[block_position] = block;
                }
            }
        }
    }

    fn calculate_lighting(block_data: &ChunkData<Block>, light_data: &mut ChunkData<LightLevel>) {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                let mut light_level: LightLevel = light::SUNLIGHT;

                for z in (0..CHUNK_SIZE).rev() {
                    let block_position = Position::new(x, y, z);
                    let block: Block = block_data[block_position];

                    light_data[block_position] = light_level;

                    if block != block::material::AIR && light_level > 1 {
                        light_level -= 1;
                    }
                }
            }
        }
    }

    pub fn update(&mut self, gl: &gl::Gl) {
        if self.mesh_invalidated {
            self.mesh.update(&self.block_data, &self.light_data);
            self.mesh.flush(gl);
            self.mesh_invalidated = false;
        }
    }

    pub fn draw(&self, gl: &gl::Gl) {
        self.mesh.draw(gl);
    }
}
