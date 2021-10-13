use std::rc::Rc;
use crate::engine::api::render_util::RenderUtil;
use crate::engine::opengl_context::OpenglContext;
use sdl2::event::Event;
use crate::engine::api::resource_manager::ResourceManager;
use crate::engine::game_controller::CameraConfig;

// todo rename
pub trait Drawable {
    fn render(&self, render_util: &RenderUtil);
    fn update(&mut self, update_context: &UpdateContext) {}
    // todo: this should return some result so that event is not propagated further once consumed
    fn handle_event(&mut self, event: &Event, context: &OpenglContext, update_context: &UpdateContext) {}
}

pub struct UpdateContext<> {
    pub sdl_mouse_position: glam::Vec2,
    pub camera_config: CameraConfig,
    pub resource_manager:  Rc<ResourceManager>
}

impl UpdateContext {

    pub fn new(sdl_mouse_position: glam::Vec2, camera_config: CameraConfig, resource_manager:  Rc<ResourceManager>) -> UpdateContext{
        UpdateContext {
            sdl_mouse_position,
            camera_config,
            resource_manager
        }
    }

    pub fn get_sdl_mouse_position(&self) -> &glam::Vec2 {
        &self.sdl_mouse_position
    }

    pub fn get_camera_config(&self) -> &CameraConfig {
        &self.camera_config
    }
}