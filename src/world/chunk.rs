use crate::data;
use crate::render_gl::buffer;

use super::{Block, CHUNK_VOLUME, Position};

// TODO: replace with block?
#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = 0]
    pos: data::f32_f32_f32,
    #[location = 1]
    clr: data::u2_u10_u10_u10_rev_float,
    #[location = 2]
    normal: data::f32_f32_f32,
}

pub struct Chunk {
    blocks: [Block; CHUNK_VOLUME as usize],
    vao: buffer::VertexArray,
    _vbo: buffer::ArrayBuffer,
    _ebo: buffer::ElementArrayBuffer,
    index_count: i32,
}

impl Chunk {
    pub fn new(gl: &gl::Gl) -> Result<Chunk, failure::Error> {
        let v0 = (-1.0, -1.0, -1.0);
        let v1 = (1.0, -1.0, -1.0);
        let v2 = (-1.0, 1.0, -1.0);
        let v3 = (1.0, 1.0, -1.0);
        let v4 = (-1.0, -1.0, 1.0);
        let v5 = (1.0, -1.0, 1.0);
        let v6 = (-1.0, 1.0, 1.0);
        let v7 = (1.0, 1.0, 1.0);

        let vbo_data = vec![
            Vertex {
                pos: v0.into(),
                clr: (1.0, 0.0, 0.0, 1.0).into(),
                normal: (0.0, 0.0, -1.0).into(),
            }, // 0
            Vertex {
                pos: v1.into(),
                clr: (0.0, 1.0, 0.0, 1.0).into(),
                normal: (0.0, 0.0, -1.0).into(),
            }, // 1
            Vertex {
                pos: v2.into(),
                clr: (0.0, 0.0, 1.0, 1.0).into(),
                normal: (0.0, 0.0, -1.0).into(),
            }, // 2
            Vertex {
                pos: v3.into(),
                clr: (1.0, 1.0, 0.0, 1.0).into(),
                normal: (0.0, 0.0, -1.0).into(),
            }, // 3
            Vertex {
                pos: v4.into(),
                clr: (0.0, 0.3, 1.0, 1.0).into(),
                normal: (0.0, 0.0, 1.0).into(),
            }, // 4
            Vertex {
                pos: v5.into(),
                clr: (1.0, 0.0, 0.3, 1.0).into(),
                normal: (0.0, 0.0, 1.0).into(),
            }, // 5
            Vertex {
                pos: v6.into(),
                clr: (0.7, 0.5, 1.0, 1.0).into(),
                normal: (0.0, 0.0, 1.0).into(),
            }, // 6
            Vertex {
                pos: v7.into(),
                clr: (1.0, 0.7, 0.5, 1.0).into(),
                normal: (0.0, 0.0, 1.0).into(),
            }, // 7
            Vertex {
                pos: v0.into(),
                clr: (0.0, 1.0, 0.3, 1.0).into(),
                normal: (0.0, -1.0, 0.0).into(),
            }, // 8
            Vertex {
                pos: v1.into(),
                clr: (1.0, 0.0, 1.0, 1.0).into(),
                normal: (0.0, -1.0, 0.0).into(),
            }, // 9
            Vertex {
                pos: v4.into(),
                clr: (0.5, 0.7, 1.0, 1.0).into(),
                normal: (0.0, -1.0, 0.0).into(),
            }, // 10
            Vertex {
                pos: v5.into(),
                clr: (1.0, 0.5, 0.1, 1.0).into(),
                normal: (0.0, -1.0, 0.0).into(),
            }, // 11
            Vertex {
                pos: v2.into(),
                clr: (0.3, 1.0, 1.0, 1.0).into(),
                normal: (0.0, 1.0, 0.0).into(),
            }, // 12
            Vertex {
                pos: v3.into(),
                clr: (0.8, 0.0, 1.0, 1.0).into(),
                normal: (0.0, 1.0, 0.0).into(),
            }, // 13
            Vertex {
                pos: v6.into(),
                clr: (0.5, 0.5, 0.4, 1.0).into(),
                normal: (0.0, 1.0, 0.0).into(),
            }, // 14
            Vertex {
                pos: v7.into(),
                clr: (0.4, 0.0, 1.0, 1.0).into(),
                normal: (0.0, 1.0, 0.0).into(),
            }, // 15
            Vertex {
                pos: v0.into(),
                clr: (0.0, 0.4, 1.0, 1.0).into(),
                normal: (-1.0, 0.0, 0.0).into(),
            }, // 16
            Vertex {
                pos: v2.into(),
                clr: (1.0, 0.0, 0.4, 1.0).into(),
                normal: (-1.0, 0.0, 0.0).into(),
            }, // 17
            Vertex {
                pos: v4.into(),
                clr: (0.7, 0.5, 1.0, 1.0).into(),
                normal: (-1.0, 0.0, 0.0).into(),
            }, // 18
            Vertex {
                pos: v6.into(),
                clr: (1.0, 0.7, 0.5, 1.0).into(),
                normal: (-1.0, 0.0, 0.0).into(),
            }, // 19
            Vertex {
                pos: v1.into(),
                clr: (0.0, 1.0, 0.0, 1.0).into(),
                normal: (1.0, 0.0, 0.0).into(),
            }, // 20
            Vertex {
                pos: v3.into(),
                clr: (0.1, 0.0, 1.0, 1.0).into(),
                normal: (1.0, 0.0, 0.0).into(),
            }, // 21
            Vertex {
                pos: v5.into(),
                clr: (0.1, 0.7, 1.0, 1.0).into(),
                normal: (1.0, 0.0, 0.0).into(),
            }, // 22
            Vertex {
                pos: v7.into(),
                clr: (1.0, 0.1, 0.7, 1.0).into(),
                normal: (1.0, 0.0, 0.0).into(),
            }, // 23
        ];

        let ebo_data: Vec<u8> = vec![
            0, 2, 1, 1, 2, 3, 4, 5, 6, 6, 5, 7, 8, 11, 10, 8, 9, 11, 12, 14, 15, 12, 15, 13, 16,
            18, 17, 18, 19, 17, 20, 21, 22, 22, 21, 23,
        ];

        let vbo = buffer::ArrayBuffer::new(gl);
        vbo.bind();
        vbo.static_draw_data(&vbo_data);
        vbo.unbind();

        let ebo = buffer::ElementArrayBuffer::new(gl);
        ebo.bind();
        ebo.static_draw_data(&ebo_data);
        ebo.unbind();

        // setup vao
        let vao = buffer::VertexArray::new(gl);

        vao.bind();
        vbo.bind();
        ebo.bind();
        Vertex::vertex_attrib_pointers(gl);
        vbo.unbind();
        vao.unbind();
        ebo.unbind();

        Ok(Chunk {
            blocks: [0; CHUNK_VOLUME as usize],
            vao,
            _vbo: vbo,
            _ebo: ebo,
            index_count: ebo_data.len() as i32,
        })
    }

    pub fn get_block(&self, position: &Position) -> Block {
        self.blocks[i32::from(position) as usize]
    }

    pub fn draw(&self, gl: &gl::Gl) {
        self.vao.bind();

        unsafe {
            gl.DrawElements(
                gl::TRIANGLES, // drawing mode
                self.index_count, // index vertex count
                gl::UNSIGNED_BYTE, // index type
                ::std::ptr::null(), /* ptr to indices (we are using ebo
                                         configured at vao creation) */
            );
        }
    }
}
