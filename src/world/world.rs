use nalgebra as na;

use crate::render_gl::Program;
use crate::resources::Resources;

use super::Chunk;

pub struct World {
    chunk: Chunk,
    program: Program,
    program_view_projection_location: i32,
}

impl World {
    pub fn new(res: &Resources, gl: &gl::Gl) -> Result<World, failure::Error> {
        // setup shader program
        let program = Program::from_res("shaders/triangle", gl, res)?;
        let program_view_projection_location = program.get_uniform_location("ViewProjection")?;

        // generate a chunk
        let chunk = Chunk::new(gl)?;

        Result::Ok(World {
            chunk,
            program,
            program_view_projection_location,
        })
    }

    pub fn draw(&self, gl: &gl::Gl, vp_matrix: &na::Matrix4<f32>) {
        self.program.set_used();
        self.program.set_uniform_matrix4fv(self.program_view_projection_location, &vp_matrix);
        self.chunk.draw(gl);
    }
}
