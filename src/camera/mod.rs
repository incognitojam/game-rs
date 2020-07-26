pub use basic::Camera;
pub use movement::NoClipMovement;
pub use target_camera::TargetCamera;

mod basic;
mod target_camera;
mod movement;

pub const FIELD_OF_VIEW: f32 = 60.0;
pub const Z_NEAR: f32 = 0.01;
pub const Z_FAR: f32 = 1000.0;
