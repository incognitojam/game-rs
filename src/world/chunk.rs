use crate::data;
use crate::render_gl::{buffer, Texture};

use super::{CHUNK_SIZE, CHUNK_VOLUME, Position};
use super::block::{Block, BLOCK_FACES, BlockFace};

// TODO: replace with block?
#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = 0]
    pos: data::f32_f32_f32,
    #[location = 1]
    normal: data::f32_f32_f32,
    #[location = 2]
    uv: data::f16_f16,
}

pub struct ChunkMesh {
    vao: buffer::VertexArray,
    _vbo: buffer::ArrayBuffer,
    _ebo: buffer::ElementArrayBuffer,
    index_count: i32,
}

impl ChunkMesh {
    fn draw(&self, gl: &gl::Gl) {
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

pub struct ChunkMeshBuilder {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl ChunkMeshBuilder {
    fn new() -> ChunkMeshBuilder {
        ChunkMeshBuilder {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    fn add_block_face(
        &mut self,
        block_face: &BlockFace,
        block_position: &Position,
        face_uvs: &[data::f16_f16; 4],
    ) {
        let face_vertices = &block_face.vertices;
        let normal = data::f32_f32_f32::from(block_face.normal);
        let index = self.vertices.len() as u32;

        for i in 0..4 {
            let vertex_position = Position::from(face_vertices[i]) + *block_position;

            let pos = data::f32_f32_f32::from(vertex_position);
            let uv = face_uvs[i];

            self.vertices.push(Vertex {
                pos,
                normal,
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

    fn build(&self, gl: &gl::Gl) -> ChunkMesh {
        let vbo = buffer::ArrayBuffer::new(gl);
        vbo.bind();
        vbo.static_draw_data::<Vertex>(&self.vertices);
        vbo.unbind();

        let ebo = buffer::ElementArrayBuffer::new(gl);
        ebo.bind();
        ebo.static_draw_data::<u32>(&self.indices);
        ebo.unbind();

        // setup vao
        let vao = buffer::VertexArray::new(gl);

        vao.bind();
        vbo.bind();
        ebo.bind();
        Vertex::vertex_attrib_pointers(&gl);
        vbo.unbind();
        vao.unbind();
        ebo.unbind();

        ChunkMesh {
            vao,
            _vbo: vbo,
            _ebo: ebo,
            index_count: self.indices.len() as i32,
        }
    }
}

pub struct Chunk {
    _position: Position,
    blocks: [Block; CHUNK_VOLUME as usize],
    chunk_mesh: ChunkMesh,
}

impl Chunk {
    pub fn new(position: Position, gl: &gl::Gl, texture: &Texture) -> Result<Chunk, failure::Error> {
        let mut builder = ChunkMeshBuilder::new();

        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    let block_position = Position::new(x, y, z);
                    let face_uvs = texture.uv_from_index(1);

                    for block_face in &BLOCK_FACES {
                        builder.add_block_face(
                            block_face,
                            &block_position,
                            &face_uvs,
                        );
                    }
                }
            }
        }

        let chunk_mesh = builder.build(gl);

        Ok(Chunk {
            _position: position,
            blocks: [0; CHUNK_VOLUME as usize],
            chunk_mesh,
        })
    }

    pub fn get_block(&self, position: &Position) -> Block {
        self.blocks[i64::from(position) as usize]
    }

    pub fn draw(&self, gl: &gl::Gl) {
        self.chunk_mesh.draw(gl);
    }
}
