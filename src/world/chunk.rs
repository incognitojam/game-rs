use std::ops::{Index, IndexMut};
use crate::data;
use crate::render_gl::{buffer, Texture};

use super::{CHUNK_SIZE, CHUNK_VOLUME, Position};
use super::block::{self, Block, BLOCK_FACES, BlockFace};

// TODO: replace with block?
#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = 0]
    pos: data::f32_f32_f32,
    #[location = 1]
    uv: data::f16_f16,
}

pub struct ChunkMesh {
    texture: Texture,
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    vao: buffer::VertexArray,
    vbo: buffer::ArrayBuffer,
    ebo: buffer::ElementArrayBuffer,
    index_count: i32,
}

impl ChunkMesh {
    fn new(gl: &gl::Gl, texture: &Texture) -> ChunkMesh {
        ChunkMesh {
            texture: texture.clone(),
            vertices: Vec::new(),
            indices: Vec::new(),
            vbo: buffer::ArrayBuffer::new(gl),
            vao: buffer::VertexArray::new(gl),
            ebo: buffer::ElementArrayBuffer::new(gl),
            index_count: 0,
        }
    }

    fn update(&mut self, data: &ChunkData<Block>) {
        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    let block_position = Position::new(x, y, z);
                    let block = data[block_position];
                    let face_uvs = self.texture.uv_from_index(block as u32);

                    if block == block::AIR {
                        // Do not render AIR blocks.
                        continue;
                    }

                    for block_face in &BLOCK_FACES {
                        let neighbor_position = block_position + block_face.normal;
                        if neighbor_position.x < 0 || neighbor_position.y < 0 || neighbor_position.z < 0
                            || neighbor_position.x >= CHUNK_SIZE || neighbor_position.y >= CHUNK_SIZE || neighbor_position.z >= CHUNK_SIZE
                            || data[neighbor_position] == block::AIR {
                            self.add_block_face(
                                block_face,
                                &block_position,
                                &face_uvs,
                            );
                        }
                    }
                }
            }
        }
    }

    fn add_block_face(
        &mut self,
        block_face: &BlockFace,
        block_position: &Position,
        face_uvs: &[data::f16_f16; 4],
    ) {
        let face_vertices = &block_face.vertices;
        let index = self.vertices.len() as u32;

        for i in 0..4 {
            let vertex_position = Position::from(face_vertices[i]) + *block_position;

            let pos = data::f32_f32_f32::from(vertex_position);
            let uv = face_uvs[i];

            self.vertices.push(Vertex {
                pos,
                uv,
            });
        }

        self.indices.push(index);
        self.indices.push(index + 1);
        self.indices.push(index + 2);
        self.indices.push(index + 2);
        self.indices.push(index + 3);
        self.indices.push(index);
    }

    pub fn flush(&mut self, gl: &gl::Gl) {
        self.vbo.bind();
        self.vbo.static_draw_data::<Vertex>(&self.vertices);
        self.vbo.unbind();

        self.ebo.bind();
        self.ebo.static_draw_data::<u32>(&self.indices);
        self.ebo.unbind();

        self.vao.bind();
        self.vbo.bind();
        self.ebo.bind();
        Vertex::vertex_attrib_pointers(&gl);
        self.vbo.unbind();
        self.vao.unbind();
        self.ebo.unbind();

        self.index_count = self.indices.len() as i32;
        self.vertices.clear();
        self.indices.clear();
    }

    pub fn draw(&self, gl: &gl::Gl) {
        self.vao.bind();

        unsafe {
            gl.DrawElements(
                gl::TRIANGLES, // drawing mode
                self.index_count, // index vertex count
                gl::UNSIGNED_INT, // index type
                ::std::ptr::null(), /* ptr to indices (we are using ebo
                                         configured at vao creation) */
            );
        }
    }
}

pub struct Chunk {
    pub position: Position,
    data: ChunkData<Block>,
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
            data: ChunkData::new(block::STONE),
            mesh: ChunkMesh::new(gl, texture),
            mesh_invalidated: true,
        };

        chunk.generate();

        Ok(chunk)
    }

    fn generate(&mut self) {
        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    let block_position = Position::new(x, y, z);
                    let block = if z == 15 { block::GRASS as Block } else if z > 11 { block::DIRT as Block } else { block::STONE as Block };
                    self.data[block_position] = block;
                }
            }
        }
    }

    pub fn update(&mut self, gl: &gl::Gl) {
        if self.mesh_invalidated {
            self.mesh.update(&self.data);
            self.mesh.flush(gl);
            self.mesh_invalidated = false;
        }
    }

    pub fn draw(&self, gl: &gl::Gl) {
        self.mesh.draw(gl);
    }
}
