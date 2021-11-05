use std::ops::Add;
use std::rc::Rc;
use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::timer::Timer;

use crate::engine::api::colour::{GREEN, RED, WHITE};
use crate::engine::api::countdown_timer::CountdownTimer;
use crate::engine::api::drawable::{Drawable, UpdateContext};
use crate::engine::api::maths::circle::Circle;
use crate::engine::api::maths::quadrangle::Quadrangle;
use crate::engine::api::maths::rectangle::Rectangle;
use crate::engine::api::maths::shapes_common::Area;
use crate::engine::api::maths::vertex::TexturedVertexDataLayout;
use crate::engine::api::render_util::RenderUtil;
use crate::engine::api::resource_manager::ResourceManager;
use crate::engine::opengl_context::OpenglContext;
use crate::engine::rendering::material::UniformKind;

pub const RING_RADIUS: f32 = 0.9;
const MAX_RING_GROWTH: f32 = 0.25;
const MAX_RING_COLLAPSE: f32 = 0.75;

pub struct Ring {
    hitbox: Circle,
    rectangle: Rectangle<TexturedVertexDataLayout>,
    alive_timer: CountdownTimer,
    fade_off_timer: CountdownTimer,
    expired_effect_timer: CountdownTimer,
}

impl Ring {
    pub fn new(position: &glam::Vec3, resource_manager: Rc<dyn ResourceManager>) -> Ring {
        let ring_shader_material = resource_manager.fetch_shader_material("osu/shaders/ring");
        let clr_shader_material = resource_manager.fetch_shader_material("osu/shaders/colour");
        let ring_sprite = resource_manager.fetch_sprite("osu/textures/ring.png");

        let tx_position = glam::vec3(position.x - RING_RADIUS, position.y - RING_RADIUS, position.z);
        let mut rectangle = Rectangle::new_textured(
            &tx_position,
            &glam::vec2(RING_RADIUS * 2.0, RING_RADIUS * 2.0),
            ring_shader_material,
            ring_sprite,
        );
        rectangle.set_material_variable("color", UniformKind::VEC_4 { value: WHITE.into() });

        let hitbox = Circle::new_colored(
            position,
            glam::vec4(0.5, 0.5, 0.5, 1.0).into(),
            RING_RADIUS,
            clr_shader_material,
        );
        let mut fade_off_timer = CountdownTimer::new(Duration::from_millis(250));
        fade_off_timer.pause();

        let mut expired_effect_timer = CountdownTimer::new(Duration::from_millis(500));
        expired_effect_timer.pause();

        Ring {
            rectangle,
            hitbox,
            alive_timer: CountdownTimer::new(Duration::from_secs(3)),
            fade_off_timer,
            expired_effect_timer,
        }
    }

    pub fn contains_point(&self, position: &glam::Vec3) -> bool {
        self.hitbox.contains_point(&(position.x, position.y))
    }

    pub fn get_score(&self) -> i32 {
        1
    }

    pub fn start_fade_off(&mut self) {
        self.fade_off_timer.unpause();
        self.alive_timer.pause();
    }

    // imagine implementing signals and removing this ugly pull model
    pub fn is_fully_faded(&self) -> bool {
        self.fade_off_timer.is_finished()
    }

    pub fn is_alive(&self) -> bool {
        !self.alive_timer.is_finished()
    }

    pub fn is_fully_expired(&self) -> bool {
        self.expired_effect_timer.is_finished()
    }
}

impl Drawable for Ring {
    fn render(&mut self, render_util: &RenderUtil) {
        self.rectangle.render(render_util);
        // self.hitbox.render(render_util); // for debugging
    }

    fn update(&mut self, update_context: &UpdateContext) {
        self.alive_timer.advance(*update_context.get_delta_time());
        self.fade_off_timer.advance(*update_context.get_delta_time());
        self.expired_effect_timer.advance(*update_context.get_delta_time());

        if self.alive_timer.is_finished() {
            self.expired_effect_timer.unpause();
            self.rectangle.set_material_variable("color", UniformKind::VEC_4 { value: RED.into() })
        }
        // todo make some nicer state machine with enum
        match self.expired_effect_timer.is_paused() {
            true => {
                match self.fade_off_timer.is_paused() {
                    true => {
                        self.rectangle.set_material_variable("fadeOffAlpha", UniformKind::FLOAT { value: 1.0 });
                    }
                    false => {
                        let scale_change = MAX_RING_GROWTH * self.fade_off_timer.get_percent_complete();
                        self.rectangle.set_scale(glam::vec3(1.0 + scale_change, 1.0 + scale_change, 1.0));
                        self.rectangle.set_material_variable("fadeOffAlpha", UniformKind::FLOAT { value: 1.0 - self.fade_off_timer.get_percent_complete() });
                    }
                }
            }
            false => {
                let scale_change = MAX_RING_COLLAPSE * self.expired_effect_timer.get_percent_complete();
                self.rectangle.set_scale(glam::vec3(1.0 - scale_change, 1.0 - scale_change, 1.0));
                self.rectangle.set_material_variable("fadeOffAlpha", UniformKind::FLOAT { value: 1.0 - self.expired_effect_timer.get_percent_complete() });
            }
        }
    }
}