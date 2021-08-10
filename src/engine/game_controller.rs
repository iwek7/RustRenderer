use crate::maths::triangle::Drawable;

pub trait GameController {
    fn get_camera_config(&self) -> CameraConfig;
}

#[derive(Clone)]
pub struct CameraConfig {

}

impl CameraConfig {
    pub fn new() -> CameraConfig {
        CameraConfig{}
    }
}