use crate::engine::game_controller::CameraConfig;

pub struct CameraGameObject {}

impl CameraGameObject {
    pub fn new() -> CameraGameObject {
        CameraGameObject {}
    }

    pub fn get_current_config() -> CameraConfig {
        CameraConfig::new()
    }
}

