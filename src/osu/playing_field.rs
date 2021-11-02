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
use crate::osu::ring::{Ring, RING_RADIUS};

const SPAWN_INTERVAL_MILLIS: u128 = 500;

pub struct PlayingField {
    background: Rectangle<TexturedVertexDataLayout>,
    rings: Vec<Ring>,
    fade_offs: Vec<Ring>,
    total_score: i32,
    size: glam::Vec2,
    // todo: this should be part of rectangle,
    spawn_time: SystemTime,

}

impl PlayingField {
    pub fn new(position: &glam::Vec3, size: &glam::Vec2, resource_manager: Rc<dyn ResourceManager>) -> PlayingField {
        let material = resource_manager.fetch_shader_material("osu/shaders/texture");
        let bg_tx = resource_manager.fetch_texture("osu/textures/EVANGELION_BG.jpg");

        let background = Rectangle::new_textured(
            position,
            size,
            material,
            bg_tx
        );

        let ring = Ring::new(&PlayingField::calc_random_ring_position(position, size), resource_manager);

        PlayingField {
            background,
            rings: vec!(ring),
            fade_offs: vec!(),
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

    pub fn get_total_score(&self) -> &i32{
        &self.total_score
    }

}

impl Drawable for PlayingField {
    fn render(&mut self, render_util: &RenderUtil) {
        self.background.render(render_util);
        self.rings.iter_mut().for_each(|ring| ring.render(render_util));
        self.fade_offs.iter_mut().for_each(|ring| ring.render(render_util));
    }

    fn update(&mut self, update_context: &UpdateContext) {
        let now = SystemTime::now();
        let difference = now.duration_since(self.spawn_time);
        if difference.unwrap().as_millis() > SPAWN_INTERVAL_MILLIS {
            self.spawn_time = now;

            let pos = glam::vec3(self.background.get_pos().0, self.background.get_pos().1, self.background.get_pos().2);

            let ring = Ring::new(&PlayingField::calc_random_ring_position(&pos, &self.size), update_context.get_engine_utilities().get_resource_manager());
            self.rings.push(ring);
        }

        self.fade_offs.retain(|ring| !ring.is_faded());
        self.fade_offs.iter_mut().for_each(|ring| ring.update(update_context));
        self.rings.iter_mut().for_each(|ring| ring.update(update_context));
    }

    fn handle_event(&mut self, event: &Event, context: &OpenglContext, update_context: &UpdateContext) {
        match context.sdl_space_to_world_space_at_z0(update_context.get_sdl_mouse_position(), &update_context.get_camera_config()) {
            None => {}
            Some(world_mouse_position) => {
                match event {
                    sdl2::event::Event::MouseButtonDown { .. } => {
                        let mut to_remove = vec!();

                        for idx in 0..self.rings.len() {
                            if self.rings[idx].contains_point(&world_mouse_position) {
                                to_remove.push(idx);
                            }
                        }

                        for i in 0..to_remove.len() {
                            // this takes into account items
                            // that were already removed during iteration of this loop
                            // its super bad :D
                            let actual_index = to_remove[i] - i;
                            self.total_score += self.rings[actual_index].get_score();
                            let mut ring = self.rings.remove(actual_index);
                            ring.start_fade_off();
                            self.fade_offs.push(ring);
                            println!("TOTAL SCORE IS {:?}", self.total_score)
                        }
                    }
                    _ => {}
                }
            }
        }

        self.rings.iter_mut().for_each(|ring| ring.handle_event(event, context, update_context));
    }
}
