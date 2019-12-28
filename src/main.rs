#[macro_use]
extern crate failure;
extern crate floating_duration;
extern crate gl;
extern crate nalgebra;
#[macro_use]
extern crate render_gl_derive;
extern crate sdl2;
extern crate vec_2_10_10_10;

use std::path::Path;
use std::time::Instant;

use floating_duration::TimeAsFloat;
use nalgebra as na;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Scancode;

use crate::camera::TargetCamera;
use crate::render_gl::{ColorBuffer, data, Viewport};
use crate::resources::Resources;
use crate::world::World;

mod debug;
pub mod camera;
pub mod render_gl;
pub mod resources;
pub mod world;

fn main() {
    if let Err(e) = run() {
        println!("{}", debug::failure_to_string(e));
    }
}

fn run() -> Result<(), failure::Error> {
    let res = Resources::from_relative_exe_path(Path::new("assets"))?;

    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let initial_window_size: (i32, i32) = (900, 700);

    let window = video_subsystem
        .window(
            "Game",
            initial_window_size.0 as u32,
            initial_window_size.1 as u32,
        )
        .opengl()
        .resizable()
        .build()?;

    let _gl_context = window.gl_create_context().unwrap();
    let gl = gl::Gl::load_with(|s| {
        video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });

    let mut viewport = Viewport::for_window(900, 700);
    let color_buffer = ColorBuffer::from_color(na::Vector3::new(0.3, 0.3, 0.5));

    let mut world = World::new(&res, &gl)?;

    let mut camera = TargetCamera::new(
        initial_window_size.0 as f32 / initial_window_size.1 as f32,
        3.14 / 2.0,
        0.01,
        1000.0,
        3.14 / 4.0,
        0.0,
    );

    viewport.set_used(&gl);
    color_buffer.set_used(&gl);

    let mut time = Instant::now();

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::Window {
                    win_event: WindowEvent::Resized(w, h),
                    ..
                } => {
                    viewport.update_size(w, h);
                    viewport.set_used(&gl);
                }
                e => handle_camera_event(&mut camera, &e),
            }
        }

        let delta = time.elapsed().as_fractional_secs();
        time = Instant::now();
        camera.apply_movement(delta as f32);

        let view_matrix = camera.get_view_matrix();
        let projection_matrix = camera.get_projection_matrix();
        unsafe {
            gl.Enable(gl::CULL_FACE);
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl.Enable(gl::DEPTH_TEST);
        }

        color_buffer.clear(&gl);
        world.update(&gl);
        world.draw(&gl, &view_matrix, &projection_matrix, &camera.project_pos().coords);

        window.gl_swap_window();
    }

    Ok(())
}

fn handle_camera_event(camera: &mut camera::TargetCamera, e: &sdl2::event::Event) {
    match *e {
        Event::KeyDown {
            scancode: Some(scancode),
            ..
        } => match scancode {
            Scancode::LCtrl | Scancode::RCtrl => camera.movement.faster = true,
            Scancode::A => camera.movement.left = true,
            Scancode::W => camera.movement.forward = true,
            Scancode::S => camera.movement.backward = true,
            Scancode::D => camera.movement.right = true,
            Scancode::Space => camera.movement.up = true,
            Scancode::LShift | Scancode::RShift => camera.movement.down = true,
            _ => (),
        },
        Event::KeyUp {
            scancode: Some(scancode),
            ..
        } => match scancode {
            Scancode::LCtrl | Scancode::RCtrl => camera.movement.faster = false,
            Scancode::A => camera.movement.left = false,
            Scancode::W => camera.movement.forward = false,
            Scancode::S => camera.movement.backward = false,
            Scancode::D => camera.movement.right = false,
            Scancode::Space => camera.movement.up = false,
            Scancode::LShift | Scancode::RShift => camera.movement.down = false,
            _ => (),
        },
        Event::MouseMotion {
            xrel,
            yrel,
            mousestate,
            ..
        } => {
            if mousestate.right() {
                camera.rotate(&na::Vector2::new(xrel as f32, -yrel as f32));
            }
        }
        _ => (),
    }
}
