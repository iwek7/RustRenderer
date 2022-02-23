use std::rc::Rc;
use std::time::Duration;

use sdl2::event::Event;

use crate::engine::api::engine_utilities::EngineUtilities;
use crate::engine::api::game_api::CameraConfig;
use crate::engine::api::render_util::RenderUtil;
use crate::engine::opengl_context::OpenglContext;

pub trait GameObject {
    fn render(&mut self, render_util: &RenderUtil);
    fn update(&mut self, update_context: &UpdateContext) {}
    // todo: this should re turn some result so that event is not propagated further once consumed
    fn handle_event(&mut self, event: &Event, context: &OpenglContext, update_context: &UpdateContext) {}
    fn base_game_object(&mut self) -> &mut BaseGameObject;
}

pub struct UpdateContext<> {
    sdl_mouse_position: glam::Vec2,
    camera_config: CameraConfig,
    engine_utilities: Rc<EngineUtilities>,
    delta_time: Duration, // time since last update
}

impl UpdateContext {
    pub fn new(sdl_mouse_position: glam::Vec2, camera_config: CameraConfig, engine_utilities: Rc<EngineUtilities>, delta_time: Duration) -> UpdateContext {
        UpdateContext {
            sdl_mouse_position,
            camera_config,
            engine_utilities,
            delta_time,
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

    pub fn get_delta_time(&self) -> &Duration { &self.delta_time }
}

pub struct BaseGameObject {
    children: Vec<Box<dyn GameObject>>,
}

impl BaseGameObject {
    pub(crate) fn new() -> BaseGameObject {
        BaseGameObject {
            children: vec!()
        }
    }
}

impl GameObject for BaseGameObject {
    fn render(&mut self, render_util: &RenderUtil) {
        self.children.iter_mut().for_each(|child| child.render(render_util))
    }

    fn update(&mut self, update_context: &UpdateContext) {
        self.children.iter_mut().for_each(|child| child.update(update_context))
    }

    fn handle_event(&mut self, event: &Event, context: &OpenglContext, update_context: &UpdateContext) {
        self.children.iter_mut().for_each(|child| child.handle_event(event, context, update_context))
    }

    fn base_game_object(&mut self) -> &mut BaseGameObject {
        self
    }
}