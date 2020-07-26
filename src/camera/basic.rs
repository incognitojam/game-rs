use nalgebra as na;

use crate::camera::{FIELD_OF_VIEW, Z_FAR, Z_NEAR};

pub struct Camera {
    position: na::Point3<f32>,
    rotation: na::UnitQuaternion<f32>,
    projection: na::Perspective3<f32>,
}

impl Camera {
    fn new(
        aspect_ratio: f32,
    ) -> Camera {
        Camera {
            position: na::Point3::origin(),
            rotation: na::UnitQuaternion::identity(),
            projection: na::Perspective3::new(
                aspect_ratio,
                FIELD_OF_VIEW,
                Z_NEAR,
                Z_FAR,
            ),
        }
    }
}
