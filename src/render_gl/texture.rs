use std::os::raw;

use failure;
use gl;

use crate::render_gl::data::f16_f16;
use crate::resources::Resources;

pub struct TextureOptions<'a> {
    resource_name: &'a str,
    format: gl::types::GLenum,
    pub gen_minimaps: bool,
    rows: u16,
}

impl<'a> TextureOptions<'a> {
    pub fn from_res_rgb(resource_name: &str) -> TextureOptions {
        TextureOptions {
            resource_name,
            format: gl::RGB,
            gen_minimaps: false,
            rows: 1,
        }
    }

    pub fn from_res_rgba(resource_name: &str) -> TextureOptions {
        TextureOptions {
            resource_name,
            format: gl::RGBA,
            gen_minimaps: false,
            rows: 1,
        }
    }

    pub fn load(self, gl: &gl::Gl, res: &Resources) -> Result<Texture, failure::Error> {
        Texture::from_res(self, gl, res)
    }

    pub fn with_gen_minimaps(mut self) -> Self {
        self.gen_minimaps = true;
        self
    }

    pub fn with_atlas_rows(mut self, rows: u16) -> Self {
        self.rows = rows;
        self
    }
}

pub struct Texture {
    gl: gl::Gl,
    obj: gl::types::GLuint,
    uv_size: f32,
}

impl Texture {
    pub fn from_res_rgb(resource_name: &str) -> TextureOptions {
        TextureOptions::from_res_rgb(resource_name)
    }

    pub fn from_res_rgba(resource_name: &str) -> TextureOptions {
        TextureOptions::from_res_rgba(resource_name)
    }

    pub fn from_res<'a>(
        options: TextureOptions<'a>,
        gl: &gl::Gl,
        res: &Resources,
    ) -> Result<Texture, failure::Error> {
        let mut obj: gl::types::GLuint = 0;
        unsafe {
            gl.GenTextures(1, &mut obj);
        }

        let texture = Texture {
            gl: gl.clone(),
            obj,
            uv_size: 1.0 / options.rows as f32,
        };

        texture.update(options, res)?;

        Ok(texture)
    }

    pub fn update<'a>(
        &self,
        options: TextureOptions<'a>,
        res: &Resources,
    ) -> Result<(), failure::Error> {
        let gl = &self.gl;

        self.bind();

        // Use "nearest neighbour" scaling method for all textures
        unsafe {
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as gl::types::GLint);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as gl::types::GLint);
        }

        match options.format {
            gl::RGB => {
                let img = res.load_rgb_image(options.resource_name)?;

                if options.gen_minimaps {
                    unsafe {
                        gl.TexImage2D(
                            gl::TEXTURE_2D,
                            0,
                            gl::RGB8 as gl::types::GLint,
                            img.width() as i32,
                            img.height() as i32,
                            0,
                            gl::RGB,
                            gl::UNSIGNED_BYTE,
                            img.as_ptr() as *const raw::c_void,
                        );
                        gl.GenerateMipmap(gl::TEXTURE_2D);
                    }
                } else {
                    unsafe {
                        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_BASE_LEVEL, 0);
                        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAX_LEVEL, 0);
                        gl.TexImage2D(
                            gl::TEXTURE_2D,
                            0,
                            gl::RGB8 as gl::types::GLint,
                            img.width() as i32,
                            img.height() as i32,
                            0,
                            gl::RGB,
                            gl::UNSIGNED_BYTE,
                            img.as_ptr() as *const raw::c_void,
                        );
                    }
                }
            }
            gl::RGBA => {
                let img = res.load_rgba_image(options.resource_name)?;

                if options.gen_minimaps {
                    unsafe {
                        gl.TexImage2D(
                            gl::TEXTURE_2D,
                            0,
                            gl::RGBA8 as gl::types::GLint,
                            img.width() as i32,
                            img.height() as i32,
                            0,
                            gl::RGBA,
                            gl::UNSIGNED_BYTE,
                            img.as_ptr() as *const raw::c_void,
                        );
                        gl.GenerateMipmap(gl::TEXTURE_2D);
                    }
                } else {
                    unsafe {
                        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_BASE_LEVEL, 0);
                        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAX_LEVEL, 0);
                        gl.TexImage2D(
                            gl::TEXTURE_2D,
                            0,
                            gl::RGBA8 as gl::types::GLint,
                            img.width() as i32,
                            img.height() as i32,
                            0,
                            gl::RGBA,
                            gl::UNSIGNED_BYTE,
                            img.as_ptr() as *const raw::c_void,
                        );
                    }
                }
            }
            _ => unreachable!("Only RGB or RGBA images can be constructed"),
        }

        self.unbind();

        Ok(())
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.BindTexture(gl::TEXTURE_2D, self.obj);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn bind_at(&self, index: u32) {
        unsafe {
            self.gl.ActiveTexture(gl::TEXTURE0 + index);
        }
        self.bind();
    }

    pub fn uv_from_index(&self, index: u32) -> [f16_f16; 4] {
        let rows = (1.0 / self.uv_size) as u32;

        // assuming texture atlas is square
        let x = index % rows;
        let y = index / rows;

        [
            self.uv_from_x_y(x, y),
            self.uv_from_x_y(x + 1, y),
            self.uv_from_x_y(x + 1, y + 1),
            self.uv_from_x_y(x, y + 1),
        ]
    }

    fn uv_from_x_y(&self, x: u32, y: u32) -> f16_f16 {
        f16_f16::from((
            self.uv_size * x as f32,
            self.uv_size * y as f32,
        ))
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteTextures(1, &mut self.obj) };
    }
}
