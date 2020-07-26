use crate::data;
use crate::render_gl::{buffer, Texture};
use crate::world::{CHUNK_SIZE, CHUNK_VOLUME, Direction, Position};
use crate::world::block::{self, Block, BLOCK_FACES, BlockFace};
use crate::world::light::{self, LightLevel};

// TODO: replace with block?
#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = 0]
    pos: data::f32_f32_f32,
    #[location = 1]
    uv: data::f16_f16,
    #[location = 2]
    light_level: data::u8_,
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
    pub(super) fn new(gl: &gl::Gl, texture: &Texture) -> ChunkMesh {
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

    pub(super) fn update(&mut self, block_data: &ChunkData<Block>, light_data: &ChunkData<LightLevel>) {
        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    let block_position: Position = Position::new(x, y, z);
                    let block: Block = block_data[block_position];
                    let light_level = data::u8_::new(light_data[block_position]);

                    if block == block::material::AIR {
                        // Do not render AIR blocks.
                        continue;
                    }

                    for block_face in &BLOCK_FACES {
                        // TODO: calculate face texture index properly
                        let mut tex_id = block;

                        if block == block::material::GRASS {
                            match block_face.direction {
                                Direction::Top => {
                                    tex_id = 0
                                }
                                _ => {}
                            }
                        }

                        let face_uvs = self.texture.uv_from_index(tex_id as u32);

                        let neighbor_position = block_position + block_face.normal;
                        if neighbor_position.x < 0 || neighbor_position.y < 0 || neighbor_position.z < 0
                            || neighbor_position.x >= CHUNK_SIZE || neighbor_position.y >= CHUNK_SIZE || neighbor_position.z >= CHUNK_SIZE
                            || block_data[neighbor_position] == block::material::AIR {
                            self.add_block_face(
                                block_face,
                                &block_position,
                                &face_uvs,
                                &light_level,
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
        light_level: &data::u8_,
    ) {
        let face_vertices = &block_face.vertices;
        let index = self.vertices.len() as u32;

        for i in 0..4 {
            let vertex_position = Position::from(face_vertices[i]) + *block_position;

            self.vertices.push(Vertex {
                pos: vertex_position.into(),
                uv: face_uvs[i],
                light_level: *light_level,
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
