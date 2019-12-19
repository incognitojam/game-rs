use crate::render_gl::Program;
use crate::resources::Resources;

use super::Chunk;

pub struct World {
    chunk: Chunk,
    program: Program,
}

impl World {
    pub fn new(res: &Resources, gl: &gl::Gl) -> Result<World, failure::Error> {
        // setup shader program
        let program = Program::from_res(gl, res, "shaders/triangle")?;

        // generate a chunk
        let chunk = Chunk::new(gl)?;

        Result::Ok(World {
            chunk,
            program,
        })
    }

    pub fn draw(&self, gl: &gl::Gl) {
        self.program.set_used();
        self.chunk.draw(gl);
    }
}
