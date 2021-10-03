use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::api::camera::CameraGameObject;
use crate::api::drawable::{Drawable, UpdateContext};
use crate::engine::game_controller::{CameraConfig, GameController};
use crate::maths::quadrangle::Quadrangle;
use crate::maths::vertex::ColoredVertexData;
use crate::opengl_context::OpenglContext;
use crate::renderer::RenderUtil;

const CAMERA_SPEED: f32 = 0.3;

pub struct GamesRoot {
    games: Vec<Box<dyn Drawable>>,
    camera: CameraGameObject,
}

impl GamesRoot {
    pub fn new(games: Vec<Box<dyn Drawable>>) -> GamesRoot {
        GamesRoot {
            games,
            camera: CameraGameObject::new(glam::vec3(0.0, 0.0, 20.0)),
        }
    }
}

impl Drawable for GamesRoot {
    fn render(&self, render_util: &RenderUtil) {
        for game in self.games.iter() {
            game.render(render_util);
        }
    }

    fn handle_event(&mut self, event: &Event, context: &OpenglContext, update_context: &UpdateContext) {
        match event {
            sdl2::event::Event::KeyDown { keycode, .. } => {
                match keycode.unwrap() {
                    Keycode::Left => {
                        self.camera.move_by(glam::Vec3::new(-CAMERA_SPEED, 0.0, 0.0))
                    }
                    Keycode::Right => {
                        self.camera.move_by(glam::Vec3::new(CAMERA_SPEED, 0.0, 0.0))
                    }
                    Keycode::Down => {
                        self.camera.move_by(glam::Vec3::new(0.0, -CAMERA_SPEED, 0.0))
                    }
                    Keycode::Up => {
                        self.camera.move_by(glam::Vec3::new(0.0, CAMERA_SPEED, 0.0))
                    }
                    _ => {}
                }
            }
            _ => {
                for game in self.games.iter_mut() {
                    game.handle_event(event, context, update_context);
                }
            }
        }
    }
}

impl GameController for GamesRoot {
    fn get_camera_config(&self) -> CameraConfig {
        self.camera.get_current_config()
    }
}

