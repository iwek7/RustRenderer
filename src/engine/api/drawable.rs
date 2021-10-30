use std::rc::Rc;

use sdl2::event::Event;

use crate::engine::api::engine_utilities::EngineUtilities;
use crate::engine::api::game_api::CameraConfig;
use crate::engine::api::render_util::RenderUtil;
use crate::engine::opengl_context::OpenglContext;

// todo rename
pub trait Drawable {
    fn render(&mut self, render_util: &RenderUtil);
    fn update(&mut self, update_context: &UpdateContext) {}
    // todo: this should return some result so that event is not propagated further once consumed
    fn handle_event(&mut self, event: &Event, context: &OpenglContext, update_context: &UpdateContext) {}
}

pub struct UpdateContext<> {
    sdl_mouse_position: glam::Vec2,
    camera_config: CameraConfig,
    engine_utilities: Rc<EngineUtilities>
}

impl UpdateContext {

    pub fn new(sdl_mouse_position: glam::Vec2, camera_config: CameraConfig, engine_utilities: Rc<EngineUtilities>) -> UpdateContext{
        UpdateContext {
            sdl_mouse_position,
            camera_config,
            engine_utilities
        }
    }

    pub fn get_engine_utilities(&self) -> &EngineUtilities {
        &self.engine_utilities
    }

    pub fn get_sdl_mouse_position(&self) -> &glam::Vec2 {
        &self.sdl_mouse_position
    }

    pub fn get_camera_config(&self) -> &CameraConfig {
        &self.camera_config
    }
}