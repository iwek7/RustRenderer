use crate::engine::game_controller::CameraConfig;

pub struct CameraGameObject {
    position: glam::Vec3,
}

impl CameraGameObject {
    pub fn new(position: glam::Vec3) -> CameraGameObject {
        CameraGameObject { position }
    }

    pub fn get_current_config(&self) -> CameraConfig {
        CameraConfig::new(self.position, glam::vec3(0.0, 1.0, 0.0), glam::vec3(4.0, 4.0, 0.0))
    }

    pub fn set_position(&mut self, new_position: glam::Vec3) {
        self.position = new_position
    }

    pub fn move_by(&mut self, offset: glam::Vec3) {
        self.position = self.position + offset;
    }

}

