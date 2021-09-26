use crate::maths::triangle::Drawable;

pub trait GameController {
    fn get_camera_config(&self) -> CameraConfig;
}

#[derive(Clone)]
pub struct CameraConfig {
    eye_position: glam::Vec3,
    up_vector: glam::Vec3,
    direction: glam::Vec3,
}

impl CameraConfig {
    pub fn new(eye_position: glam::Vec3, up_vector: glam::Vec3, direction: glam::Vec3) -> CameraConfig {
        CameraConfig {
            eye_position,
            up_vector,
            direction,
        }
    }

    pub fn get_eye_position(&self) -> &glam::Vec3 {
        &self.eye_position
    }

    pub fn get_up_vector(&self) -> &glam::Vec3 {
        &self.up_vector
    }

    pub fn get_direction(&self) -> &glam::Vec3 {
        &self.direction
    }

    pub fn get_view_matrix(&self) -> glam::Mat4 {
        glam::Mat4::look_at_rh(
            self.get_eye_position().clone(),
            self.get_direction().clone(),
            self.get_up_vector().clone(),
        )
    }
}