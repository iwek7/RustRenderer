use std::borrow::BorrowMut;
use std::process::id;
use std::rc::Rc;
use std::time::SystemTime;

use rand::prelude::*;
use sdl2::event::Event;

use crate::engine::api::colour::Colour;
use crate::engine::api::drawable::{Drawable, UpdateContext};
use crate::engine::api::maths::quadrangle::Quadrangle;
use crate::engine::api::maths::rectangle::Rectangle;
use crate::engine::api::maths::shapes_common::Area;
use crate::engine::api::maths::vertex::{ColoredVertexDataLayout, TexturedVertexDataLayout};
use crate::engine::api::render_util::RenderUtil;
use crate::engine::api::resource_manager::ResourceManager;
use crate::engine::opengl_context::OpenglContext;
use crate::osu::ring::{Ring, RING_RADIUS, RingStateKind};

const SPAWN_INTERVAL_MILLIS: u128 = 500;

pub struct PlayingField {
    background: Rectangle<TexturedVertexDataLayout>,
    rings: Vec<Ring>,
    fade_offs: Vec<Ring>,
    expires: Vec<Ring>,
    total_score: i32,
    size: glam::Vec2,
    // todo: this should be part of rectangle,
    spawn_time: SystemTime,
}

impl PlayingField {
    pub fn new(position: &glam::Vec3, size: &glam::Vec2, resource_manager: Rc<dyn ResourceManager>) -> PlayingField {
        let material = resource_manager.fetch_shader_material("osu/shaders/texture");
        let bg_sprite = resource_manager.fetch_sprite("osu/textures/EVANGELION_BG.jpg");

        let background = Rectangle::new_textured(
            position,
            size,
            material,
            bg_sprite,
        );

        PlayingField {
            background,
            rings: vec!(),
            fade_offs: vec!(),
            expires: vec!(),
            total_score: 0,
            size: size.clone(),
            spawn_time: SystemTime::now(),
        }
    }

    pub fn calc_random_ring_position(pos: &glam::Vec3, size: &glam::Vec2) -> glam::Vec3 {
        let mut rng = thread_rng();

        let x = rng.gen_range((pos.x + RING_RADIUS)..(pos.x + size.x - RING_RADIUS));
        let y = rng.gen_range((pos.y + RING_RADIUS)..(pos.y + size.y - RING_RADIUS));
        glam::vec3(x, y, 0.0)
    }

    pub fn get_total_score(&self) -> &i32 {
        &self.total_score
    }
}

impl Drawable for PlayingField {
    fn render(&mut self, render_util: &RenderUtil) {
        self.background.render(render_util);
        self.rings.iter_mut().for_each(|ring| ring.render(render_util));
        self.fade_offs.iter_mut().for_each(|ring| ring.render(render_util));
        self.expires.iter_mut().for_each(|ring| ring.render(render_util));
    }

    fn update(&mut self, update_context: &UpdateContext) {
        // spawn new rings randomly
        let now = SystemTime::now();
        let difference = now.duration_since(self.spawn_time);
        if difference.unwrap().as_millis() > SPAWN_INTERVAL_MILLIS {
            self.spawn_time = now;

            let pos = glam::vec3(self.background.get_pos().x, self.background.get_pos().y, self.background.get_pos().z);

            let ring = Ring::new(&PlayingField::calc_random_ring_position(&pos, &self.size), update_context.get_engine_utilities().get_resource_manager());
            self.rings.push(ring);
        }
        // check if fade off effects are finished
        self.fade_offs.retain(|ring| !ring.is_fully_faded());
        self.expires.retain(|ring| !ring.is_fully_expired());

        // update everything
        self.fade_offs.iter_mut().for_each(|ring| ring.update(update_context));
        self.rings.iter_mut().for_each(|ring| ring.update(update_context));
        self.expires.iter_mut().for_each(|ring| ring.update(update_context));

        // check what is newly expired
        let mut expired = self.rings.drain_filter(|ring| !ring.is_alive()).collect::<Vec<_>>();
        self.total_score -= expired.iter()
            .map(|ring| ring.get_score())
            .fold(0, |accum, iter| accum + iter);
        self.expires.append(&mut expired);
    }

    fn handle_event(&mut self, event: &Event, context: &OpenglContext, update_context: &UpdateContext) {
        match context.sdl_space_to_world_space_at_z0(update_context.get_sdl_mouse_position(), &update_context.get_camera_config()) {
            None => {}
            Some(world_mouse_position) => {
                match event {
                    sdl2::event::Event::MouseButtonDown { .. } => {
                        self.rings
                            .drain_filter(|ring| ring.contains_point(&world_mouse_position))
                            .collect::<Vec<_>>()
                            .drain(0..)
                            .for_each(|mut ring|
                                match ring.handle_click() {
                                    RingStateKind::ALIVE => { panic!("Unexpected alive state returned when popping ring ") }
                                    RingStateKind::FADE_OFF => {
                                        self.total_score += ring.get_score();
                                        self.fade_offs.push(ring);
                                    }
                                    RingStateKind::EXPIRE => {
                                        self.total_score -= ring.get_score();
                                        self.expires.push(ring);
                                    }
                                });
                        println!("TOTAL SCORE IS {:?}", self.total_score);
                    }
                    _ => {}
                }
            }
        }

        self.rings.iter_mut().for_each(|ring| ring.handle_event(event, context, update_context));
    }
}
