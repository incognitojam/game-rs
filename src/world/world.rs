use nalgebra as na;

use crate::render_gl::Program;
use crate::resources::Resources;

use super::Chunk;

pub struct World {
    chunk: Chunk,
    program: Program,
    program_view_projection_location: i32,
    camera_pos_location: i32,
}

impl World {
    pub fn new(res: &Resources, gl: &gl::Gl) -> Result<World, failure::Error> {
        // setup shader program
        let program = Program::from_res("shaders/cube", gl, res)?;
        let program_view_projection_location = program.get_uniform_location("ViewProjection")?;
        let camera_pos_location = program.get_uniform_location("CameraPos")?;

        // generate a chunk
        let chunk = Chunk::new(gl)?;

        Result::Ok(World {
            chunk,
            program,
            program_view_projection_location,
            camera_pos_location,
        })
    }

    pub fn draw(&self, gl: &gl::Gl, vp_matrix: &na::Matrix4<f32>, camera_pos: &na::Vector3<f32>) {
        self.program.set_used();
        self.program.set_uniform_matrix4fv(self.program_view_projection_location, vp_matrix);
        self.program.set_uniform_3f(self.camera_pos_location, camera_pos);
        self.chunk.draw(gl);
    }
}
