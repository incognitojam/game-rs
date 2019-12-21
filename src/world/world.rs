use nalgebra as na;

use crate::render_gl::{Program, Texture};
use crate::resources::Resources;

use super::Chunk;

pub struct World {
    chunk: Chunk,
    program: Program,
    view_location: i32,
    projection_location: i32,
    camera_pos_location: i32,
    texture: Texture,
    texture_specular: Texture,
}

impl World {
    pub fn new(res: &Resources, gl: &gl::Gl) -> Result<World, failure::Error> {
        // load textures
        let texture = Texture::from_res_rgb("textures/dice.png").load(gl, res)?;
        let texture_specular = Texture::from_res_rgb("textures/dice_specular.png").load(gl, res)?;

        // setup shader program
        let program = Program::from_res("shaders/cube", gl, res)?;
        let view_location = program.get_uniform_location("View")?;
        let projection_location = program.get_uniform_location("Projection")?;
        let camera_pos_location = program.get_uniform_location("CameraPos")?;

        // generate a chunk
        let chunk = Chunk::new(gl)?;

        Result::Ok(World {
            chunk,
            program,
            view_location,
            projection_location,
            camera_pos_location,
            texture,
            texture_specular,
        })
    }

    pub fn draw(
        &self,
        gl: &gl::Gl,
        view_matrix: &na::Matrix4<f32>,
        projection_matrix: &na::Matrix4<f32>,
        camera_pos: &na::Vector3<f32>
    ) {
        self.texture.bind_at(0);
        self.texture_specular.bind_at(1);

        self.program.set_used();
        self.program.set_uniform_1i(self.program.get_uniform_location("TexFace").unwrap(), 0);
        self.program.set_uniform_1i(self.program.get_uniform_location("TexSpecular").unwrap(), 1);
        self.program.set_uniform_matrix4fv(self.view_location, view_matrix);
        self.program.set_uniform_matrix4fv(self.projection_location, projection_matrix);
        self.program.set_uniform_3f(self.camera_pos_location, camera_pos);
        self.chunk.draw(gl);
    }
}
