use nalgebra as na;

use super::NoClipMovement;

pub struct TargetCamera {
    pub target: na::Point3<f32>,
    distance: f32,
    pub rotation: na::UnitQuaternion<f32>,
    projection: na::Perspective3<f32>,
    pub movement: NoClipMovement,
    invalidated: bool,
}

impl TargetCamera {
    pub fn new(
        aspect: f32,
        fov: f32,
        znear: f32,
        zfar: f32,
        _initial_tilt: f32,
        initial_distance: f32,
    ) -> TargetCamera {
        TargetCamera {
            target: na::Point3::origin(),
            distance: 1.0,
            rotation: na::UnitQuaternion::from_axis_angle(
                &na::Vector3::x_axis(),
                ::std::f32::consts::PI / 4.0,
            ),
            projection: na::Perspective3::new(aspect, fov, znear, zfar),
            movement: NoClipMovement::new(),
            invalidated: true,
        }
    }

    /// Calculate position of camera from the view matrix.
    pub fn project_pos(&self) -> na::Point3<f32> {
        na::Translation3::<f32>::from(self.target.coords)
            * self.rotation
            * na::Translation3::<f32>::from(na::Vector3::z() * self.distance)
            * na::Point3::<f32>::origin()
    }

    fn get_view_matrix(&self) -> na::Matrix4<f32> {
        (na::Translation3::<f32>::from(self.target.coords)
            * self.rotation
            * na::Translation3::<f32>::from(na::Vector3::z() * self.distance)).inverse()
            .to_homogeneous()
    }

    pub fn get_projection_matrix(&self) -> na::Matrix4<f32> {
        self.projection.into_inner()
    }

    pub fn get_view_projection_matrix(&self) -> na::Matrix4<f32> {
        self.get_projection_matrix() * self.get_view_matrix()
    }

    pub fn rotate(&mut self, rel: &na::Vector2<f32>) {
        let around_x =
            na::UnitQuaternion::from_axis_angle(&na::Vector3::x_axis(), rel.y as f32 * 0.005);
        let around_z =
            na::UnitQuaternion::from_axis_angle(&na::Vector3::z_axis(), -rel.x as f32 * 0.005);

        self.rotation = around_z * self.rotation * around_x;
    }

    /// Update camera position according to the movement state.
    pub fn apply_movement(&mut self, delta: f32) {
        if !self.movement.has_movement() && !self.invalidated {
            return;
        }

        if self.movement.has_movement() {
            let mut mov3 = self.movement.get_vector();

            let camera_pos = self.project_pos();
            if camera_pos.z < self.target.z {
                mov3.y = -mov3.y;
            }

            let mov3_rotated = self.rotation * na::Vector3::new(mov3.x, mov3.y, 0.0);
            let xy = na::Vector2::new(mov3_rotated.x, mov3_rotated.y).try_normalize(0.01);

            let combined_movement = na::Vector3::new(
                xy.map(|v| v.x).unwrap_or(0.0),
                xy.map(|v| v.y).unwrap_or(0.0),
                mov3.z,
            ).try_normalize(0.01);

            if let Some(combined_movement) = combined_movement {
                let movement_translation = combined_movement
                    * (if self.movement.faster { 15.0 } else { 5.0 })
                    * delta;

                self.target += na::Vector3::new(
                    movement_translation.x,
                    movement_translation.y,
                    movement_translation.z,
                );
            }
        }

        self.invalidated = false;
    }
}
