use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::{MouseButton, MouseWheelDirection};

use crate::engine::api::camera::CameraGameObject;
use crate::engine::api::drawable::{Drawable, UpdateContext};
use crate::engine::api::game_api::{CameraConfig, GameController};
use crate::engine::api::maths::quadrangle::Quadrangle;
use crate::engine::api::maths::vertex::ColoredVertexData;
use crate::engine::api::render_util::RenderUtil;
use crate::engine::opengl_context::OpenglContext;

const CAMERA_SPEED: f32 = 0.3;


pub struct GamesRoot {
    games: Vec<Box<dyn Drawable>>,
    camera: CameraGameObject, //todo: camera should be game object
}

impl GamesRoot {
    pub fn new(games: Vec<Box<dyn Drawable>>) -> GamesRoot {
        GamesRoot {
            games,
            camera: CameraGameObject::new(
                glam::vec3(-10.0, -10.0, 20.0),
                glam::vec3(0.0, 1.0, 0.0),
                glam::vec3(-10.0, -10.0, -1.0),
            ),
        }
    }
}

impl Drawable for GamesRoot {
    fn render(&self, render_util: &RenderUtil) {
        for game in self.games.iter() {
            game.render(render_util);
        }
    }

    fn update(&mut self, update_context: &UpdateContext) {
        for game in self.games.iter_mut() {
            game.update(update_context);
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
            sdl2::event::Event::MouseWheel {
                y, direction, ..
            } => {
                if y != &0 {
                    let amount = *y as f32 * {
                        if direction == &MouseWheelDirection::Normal { 1.0 } else { -1.0 }
                    };
                    self.camera.zoom_by(amount);
                }
            },
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

