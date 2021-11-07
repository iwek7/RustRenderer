use std::ops::Add;
use std::rc::Rc;
use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::timer::Timer;

use crate::engine::api::colour::{BLUE, Colour, GREEN, RED, TRANSPARENT, WHITE};
use crate::engine::api::countdown_timer::CountdownTimer;
use crate::engine::api::drawable::{Drawable, UpdateContext};
use crate::engine::api::maths::circle::Circle;
use crate::engine::api::maths::quadrangle::Quadrangle;
use crate::engine::api::maths::rectangle::Rectangle;
use crate::engine::api::maths::shapes_common::Area;
use crate::engine::api::maths::util::{lerp_v3, lerp_v4};
use crate::engine::api::maths::vertex::TexturedVertexDataLayout;
use crate::engine::api::render_util::RenderUtil;
use crate::engine::api::resource_manager::ResourceManager;
use crate::engine::opengl_context::OpenglContext;
use crate::engine::rendering::material::UniformKind;

pub const RING_RADIUS: f32 = 0.9;
const MAX_RING_GROWTH: f32 = 0.25;
const MAX_RING_COLLAPSE: f32 = 0.75;
const RING_BORDER_SCALE: f32 = 1.5;

const ALIVE_TIMER_DURATION: Duration = Duration::from_secs(2);
const FADE_OFF_TIMER_DURATION: Duration = Duration::from_millis(250);
const EXPIRE_TIMER_DURATION: Duration = Duration::from_millis(400);
const CLICK_WINDOW_LENGTH: Duration = Duration::from_millis(500);

pub struct Ring {
    hit_box: Circle,
    ring_rectangle: Rectangle<TexturedVertexDataLayout>,
    ring_border_rectangle: Rectangle<TexturedVertexDataLayout>,
    state: RingState,
}

impl Ring {
    pub fn new(position: &glam::Vec3, resource_manager: Rc<dyn ResourceManager>) -> Ring {
        let ring_shader_material = resource_manager.fetch_shader_material("osu/shaders/ring");
        let clr_shader_material = resource_manager.fetch_shader_material("osu/shaders/colour");
        let ring_border_shader_material = resource_manager.fetch_shader_material("osu/shaders/ring_border");
        let ring_sprite = resource_manager.fetch_sprite("osu/textures/ring.png");
        let ring_border_sprite = resource_manager.fetch_sprite("osu/textures/rings_border.png");

        let tx_position = glam::vec3(position.x - RING_RADIUS, position.y - RING_RADIUS, position.z);
        let mut ring_rectangle = Rectangle::new_textured(
            &tx_position,
            &glam::vec2(RING_RADIUS * 2.0, RING_RADIUS * 2.0),
            ring_shader_material,
            ring_sprite.clone(),
        );
        ring_rectangle.set_material_variable("color", UniformKind::VEC_4 { value: BLUE.into() });

        let mut ring_border_rectangle = Rectangle::new_textured(
            &tx_position,
            &glam::vec2(RING_RADIUS * 2.0, RING_RADIUS * 2.0),
            ring_border_shader_material,
            ring_border_sprite,
        );
        ring_border_rectangle.set_material_variable("color", UniformKind::VEC_4 { value: BLUE.into() });
        ring_border_rectangle.set_scale(glam::vec3(RING_BORDER_SCALE, RING_BORDER_SCALE, 0.0));

        let hit_box = Circle::new_colored(
            position,
            glam::vec4(0.5, 0.5, 0.5, 1.0).into(),
            RING_RADIUS,
            clr_shader_material,
        );

        Ring {
            ring_rectangle,
            ring_border_rectangle,
            hit_box,
            state: RingState::new_alive(CountdownTimer::new(ALIVE_TIMER_DURATION)),
        }
    }

    pub fn contains_point(&self, position: &glam::Vec3) -> bool {
        self.hit_box.contains_point(&(position.x, position.y))
    }

    pub fn get_score(&self) -> i32 {
        1
    }

    pub fn handle_click(&mut self) -> RingStateKind {
        return match self.state.timer.get_duration_left() <= CLICK_WINDOW_LENGTH {
            true => {
                self.state = RingState::new_fade_off(CountdownTimer::new(FADE_OFF_TIMER_DURATION));
                RingStateKind::FADE_OFF
            }
            false => {
                self.state = RingState::new_expire(CountdownTimer::new(EXPIRE_TIMER_DURATION));
                RingStateKind::EXPIRE
            }
        }
    }

    // imagine implementing signals and removing this ugly pull model
    pub fn is_fully_faded(&self) -> bool {
        self.state.kind == RingStateKind::FADE_OFF && self.state.timer.is_finished()
    }

    pub fn is_alive(&self) -> bool {
        self.state.kind == RingStateKind::ALIVE && !self.state.timer.is_finished()
    }

    pub fn is_fully_expired(&self) -> bool {
        self.state.kind == RingStateKind::EXPIRE && self.state.timer.is_finished()
    }
}

impl Drawable for Ring {
    fn render(&mut self, render_util: &RenderUtil) {
        if self.state.kind == RingStateKind::ALIVE {
            self.ring_border_rectangle.render(render_util);
        }
        self.ring_rectangle.render(render_util);
    }

    fn update(&mut self, update_context: &UpdateContext) {
        self.state.timer.advance(*update_context.get_delta_time());

        match self.state.kind {
            RingStateKind::ALIVE => {
                match self.state.timer.is_finished() {
                    true => {
                        self.state = RingState::new_expire(CountdownTimer::new(EXPIRE_TIMER_DURATION));
                    }
                    false => {
                        self.ring_rectangle.set_material_variable("fadeOffAlpha", UniformKind::FLOAT { value: 1.0 });

                        let border_scale = 1.0 + (RING_BORDER_SCALE - 1.0) * (1.0 - self.state.timer.get_percent_complete());
                        self.ring_border_rectangle.set_scale(glam::vec3(border_scale, border_scale, 0.0));
                    }
                }
            }
            RingStateKind::FADE_OFF => {
                match self.state.timer.is_finished() {
                    true => {}
                    false => {
                        let scale_change = MAX_RING_GROWTH * self.state.timer.get_percent_complete();
                        self.ring_rectangle.set_scale(glam::vec3(1.0 + scale_change, 1.0 + scale_change, 1.0));
                        self.ring_rectangle.set_material_variable("fadeOffAlpha", UniformKind::FLOAT { value: 1.0 - self.state.timer.get_percent_complete() });
                    }
                }
            }
            RingStateKind::EXPIRE => {
                match self.state.timer.is_finished() {
                    true => {}
                    false => {
                        self.ring_rectangle.set_material_variable("color", UniformKind::VEC_4 { value: RED.into() });
                        let scale_change = MAX_RING_COLLAPSE * self.state.timer.get_percent_complete();
                        self.ring_rectangle.set_scale(glam::vec3(1.0 - scale_change, 1.0 - scale_change, 1.0));
                        self.ring_rectangle.set_material_variable("fadeOffAlpha", UniformKind::FLOAT { value: 1.0 - self.state.timer.get_percent_complete() })
                    }
                }
            }
        }
    }
}

struct RingState {
    timer: CountdownTimer,
    kind: RingStateKind,
}

impl RingState {
    fn new_alive(timer: CountdownTimer) -> RingState {
        RingState {
            kind: RingStateKind::ALIVE,
            timer,
        }
    }

    fn new_fade_off(timer: CountdownTimer) -> RingState {
        RingState {
            kind: RingStateKind::FADE_OFF,
            timer,
        }
    }

    fn new_expire(timer: CountdownTimer) -> RingState {
        RingState {
            kind: RingStateKind::EXPIRE,
            timer,
        }
    }
}

#[derive(PartialEq)]
pub enum RingStateKind {
    ALIVE,
    FADE_OFF,
    EXPIRE,
}