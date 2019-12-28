use nalgebra as na;

use crate::render_gl::{Program, Texture};
use crate::resources::Resources;

use super::Chunk;

pub struct World {
    chunk: Chunk,
    program: Program,
    texture: Texture,
    view_location: i32,
    projection_location: i32,
    tex_face_location: i32,
}

impl World {
    pub fn new(res: &Resources, gl: &gl::Gl) -> Result<World, failure::Error> {
        // setup shader program
        let program = Program::from_res("shaders/cube", gl, res)?;
        let view_location = program.get_uniform_location("View")?;
        let projection_location = program.get_uniform_location("Projection")?;
        let tex_face_location = program.get_uniform_location("TexFace")?;

        // load textures
        let texture = Texture::from_res_rgba("textures/minecraft.png")
            .with_atlas_rows(16)
            .load(gl, res)?;

        // generate a chunk
        let chunk = Chunk::new((0, 0, 0).into(), gl, &texture)?;

        Result::Ok(World {
            chunk,
            program,
            texture,
            view_location,
            projection_location,
//            camera_pos_location,
            tex_face_location,
        })
    }

    pub fn update(
        &mut self,
        gl: &gl::Gl,
    ) {
        self.chunk.update(gl);
    }

    pub fn draw(
        &self,
        gl: &gl::Gl,
        view_matrix: &na::Matrix4<f32>,
        projection_matrix: &na::Matrix4<f32>,
        camera_pos: &na::Vector3<f32>,
    ) {
        self.texture.bind_at(0);

        self.program.set_used();
        self.program.set_uniform_matrix4fv(self.view_location, view_matrix);
        self.program.set_uniform_matrix4fv(self.projection_location, projection_matrix);
        self.program.set_uniform_1i(self.tex_face_location, 0);

        self.chunk.draw(gl);
    }
}
