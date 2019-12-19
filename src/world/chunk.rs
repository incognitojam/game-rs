use crate::data;
use crate::render_gl::buffer;

use super::{Block, CHUNK_VOLUME, Position};

// TODO: replace with block?
#[derive(VertexAttribPointers)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = 0]
    pos: data::f32x3,
    #[location = 1]
    clr: data::u2_u10_u10_u10_rev_float,
}

pub struct Chunk {
    blocks: [Block; CHUNK_VOLUME as usize],
    _vbo: buffer::ArrayBuffer,
    vao: buffer::VertexArray,
}

impl Chunk {
    pub fn new(gl: &gl::Gl) -> Result<Chunk, failure::Error> {
        let vertices: Vec<Vertex> = vec![
            Vertex { pos: (0.5, -0.5, 0.0).into(), clr: (1.0, 0.0, 0.0, 1.0).into() }, // bottom right
            Vertex { pos: (-0.5, -0.5, 0.0).into(), clr: (0.0, 1.0, 0.0, 1.0).into() }, // bottom left
            Vertex { pos: (0.0, 0.5, 0.0).into(), clr: (0.0, 0.0, 1.0, 1.0).into() }, // top
        ];

        let vbo = buffer::ArrayBuffer::new(gl);
        vbo.bind();
        vbo.static_draw_data(&vertices);
        vbo.unbind();

        let vao = buffer::VertexArray::new(gl);

        vao.bind();
        vbo.bind();
        Vertex::vertex_attrib_pointers(gl);
        vbo.unbind();
        vao.unbind();

        Ok(Chunk {
            blocks: [0; CHUNK_VOLUME as usize],
            vao,
            _vbo: vbo,
        })
    }

    pub fn get_block(&self, position: &Position) -> Block {
        self.blocks[i32::from(position) as usize]
    }

    pub fn draw(&self, gl: &gl::Gl) {
        self.vao.bind();

        unsafe {
            gl.DrawArrays(
                gl::TRIANGLES, // mode
                0, // starting index in the enabled arrays
                3, // number of indices to be rendered
            );
        }
    }
}
