use nalgebra as na;

pub struct TargetCamera {
    pub target: na::Point3<f32>,
    distance: f32,
    pub rotation: na::UnitQuaternion<f32>,
    projection: na::Perspective3<f32>,
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
            distance: initial_distance,
            rotation: na::UnitQuaternion::from_axis_angle(
                &na::Vector3::x_axis(),
                ::std::f32::consts::PI / 4.0,
            ),
            projection: na::Perspective3::new(aspect, fov, znear, zfar),
        }
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
}
