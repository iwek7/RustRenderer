use std::ops::Add;
use std::rc::Rc;
use std::time::{Duration, Instant};

use sdl2::event::Event;

use crate::engine::api::colour::WHITE;
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
const FADE_OFF_DURATION: Duration = Duration::from_millis(250);

pub struct Ring {
    hitbox: Circle,
    rectangle: Rectangle<TexturedVertexDataLayout>,
    fade_off_start: Option<Instant>,
}

impl Ring {
    pub fn new(position: &glam::Vec3, resource_manager: Rc<dyn ResourceManager>) -> Ring {
        let ring_shader_material = resource_manager.fetch_shader_material("osu/shaders/ring");
        let clr_shader_material = resource_manager.fetch_shader_material("osu/shaders/colour");
        let ring_sprite = resource_manager.fetch_sprite("osu/textures/ring.png");


        let tx_position = glam::vec3(position.x - RING_RADIUS, position.y - RING_RADIUS, position.z);
        let rectangle = Rectangle::new_textured(
            &tx_position,
            &glam::vec2(RING_RADIUS * 2.0, RING_RADIUS * 2.0),
            ring_shader_material,
            ring_sprite,
        );

        let hitbox = Circle::new_colored(
            position,
            glam::vec4(0.5, 0.5, 0.5, 1.0).into(),
            RING_RADIUS,
            clr_shader_material,
        );

        Ring {
            rectangle,
            hitbox,
            fade_off_start: None,
        }
    }

    pub fn contains_point(&self, position: &glam::Vec3) -> bool {
        self.hitbox.contains_point(&(position.x, position.y))
    }

    pub fn get_score(&self) -> i32 {
        1
    }

    pub fn start_fade_off(&mut self) {
        self.fade_off_start = Some(Instant::now());
    }

    // imagine implementing signals and removing this ugly pull model
    pub fn is_faded(&self) -> bool {
        match self.fade_off_start {
            None => { false }
            Some(start_time) => {
               let current_fade_off_duration = Instant::now().duration_since(start_time);
                current_fade_off_duration > FADE_OFF_DURATION
            }
        }
    }
}

impl Drawable for Ring {
    fn render(&mut self, render_util: &RenderUtil) {
        self.rectangle.render(render_util);
        // self.hitbox.render(render_util);
    }

    fn update(&mut self, update_context: &UpdateContext) {
        match self.fade_off_start {
            None => {
                self.rectangle.set_material_variable("fadeOffAlpha", UniformKind::FLOAT { value: 1.0 });
            }
            Some(start_time) => {
                let curr_time = Instant::now().duration_since(start_time).as_millis();
                let percentage_of_transition = curr_time as f32 / FADE_OFF_DURATION.as_millis() as f32;
                let scale_change = MAX_RING_GROWTH * percentage_of_transition;
                self.rectangle.set_scale(glam::vec3(1.0 + scale_change, 1.0 + scale_change, 1.0));
                self.rectangle.set_material_variable("fadeOffAlpha", UniformKind::FLOAT { value: 1.0 - percentage_of_transition });
            }
        }
    }
}