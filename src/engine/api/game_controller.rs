pub trait GameController {
    fn get_camera_config(&self) -> CameraConfig;
}

#[derive(Clone)]
pub struct CameraConfig {
    eye_position: glam::Vec3,
    up_vector: glam::Vec3,
    look_at: glam::Vec3,
}

impl CameraConfig {
    pub fn new(eye_position: glam::Vec3, up_vector: glam::Vec3, look_at: glam::Vec3) -> CameraConfig {
        CameraConfig {
            eye_position,
            up_vector,
            look_at,
        }
    }

    pub fn get_eye_position(&self) -> &glam::Vec3 {
        &self.eye_position
    }

    pub fn get_up_vector(&self) -> &glam::Vec3 {
        &self.up_vector
    }

    pub fn get_look_at(&self) -> &glam::Vec3 {
        &self.look_at
    }

    pub fn get_view_matrix(&self) -> glam::Mat4 {
        glam::Mat4::look_at_rh(
            self.get_eye_position().clone(),
            self.get_look_at().clone(),
            self.get_up_vector().clone(),
        )
    }

    pub fn get_projection_matrix(&self, aspect_ratio: f32) -> glam::Mat4 {
       glam::Mat4::perspective_rh_gl(0.78, aspect_ratio, 0.1, 100.0)
    }
}