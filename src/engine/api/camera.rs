use std::ops::Mul;
use crate::engine::game_controller::CameraConfig;

pub struct CameraGameObject {
    position: glam::Vec3,
    up: glam::Vec3,
    look_at: glam::Vec3,
}

impl CameraGameObject {
    pub fn new(position: glam::Vec3, up: glam::Vec3, look_at: glam::Vec3) -> CameraGameObject {
        CameraGameObject {
            position,
            up,
            look_at,
        }
    }

    pub fn get_current_config(&self) -> CameraConfig {
        CameraConfig::new(self.position, self.up, self.look_at)
    }

    pub fn set_position(&mut self, new_position: glam::Vec3) {
        self.position = new_position
    }

    pub fn move_by(&mut self, offset: glam::Vec3) {
        self.position = self.position + offset;
    }

    pub fn zoom_by(&mut self, amount: f32) {
        println!("ZOOMING BY {:?}", amount);
        self.position = self.position + (self.position - self.look_at).normalize().mul(amount);
    }
}

