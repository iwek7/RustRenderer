use std::rc::Rc;
use rand::prelude::*;
use std::time::SystemTime;
use sdl2::event::Event;

use crate::{create_rect_coords, create_rect_coords_colored, create_rect_coords_colored_deprecated, create_rect_coords_deprecated};
use crate::api::colour::Colour;
use crate::api::drawable::{Drawable, UpdateContext};
use crate::api::resource_manager::ResourceManager;
use crate::maths::quadrangle::Quadrangle;
use crate::maths::shapes_common::Area;
use crate::maths::vertex::{ColoredVertexData, TexturedVertexData};
use crate::opengl_context::OpenglContext;
use crate::osu::ring::{Ring, RING_RADIUS};
use crate::renderer::RenderUtil;

const SPAWN_INTERVAL_MILLIS : u128 = 500;

pub struct PlayingField {
    background: Quadrangle<TexturedVertexData>,
    rings: Vec<Ring>,
    total_score: i32,
    size: glam::Vec2, // todo: this should be part of rectangle,
    spawn_time: SystemTime
}

impl PlayingField {
    pub fn new(position: &glam::Vec3, size: &glam::Vec2, resource_manager: Rc<ResourceManager>) -> PlayingField {
        let shader = resource_manager.fetch_shader_program("osu/shaders/texture");
        let bg_tx = resource_manager.fetch_texture("osu/textures/EVANGELION_BG.jpg");
        let background = Quadrangle::new(
            create_rect_coords(position, size, &bg_tx.topology.get_sprite_coords(0, 0).unwrap()),
            [0, 1, 3, 1, 2, 3],
            shader,
            Some(bg_tx),
        );

        let ring = Ring::new(&PlayingField::calc_random_ring_position(position, size), resource_manager);

        PlayingField {
            background,
            rings: vec!(ring),
            total_score: 0,
            size: size.clone(),
            spawn_time: SystemTime::now()
        }
    }

    pub fn calc_random_ring_position(pos: &glam::Vec3, size: &glam::Vec2) -> glam::Vec3{
        let mut rng = thread_rng();

        let x = rng.gen_range((pos.x + RING_RADIUS)..(pos.x + size.x - RING_RADIUS));
        let y =  rng.gen_range((pos.y + RING_RADIUS)..(pos.y + size.y - RING_RADIUS));
        glam::vec3(x, y, 0.0)
    }
}

impl Drawable for PlayingField {
    fn render(&self, render_util: &RenderUtil) {
        self.background.render(render_util);
        self.rings.iter().for_each(|ring| ring.render(render_util));
    }

    fn update(&mut self, update_context: &UpdateContext) {
        let now = SystemTime::now();
        let difference = now.duration_since(self.spawn_time);
        if difference.unwrap().as_millis() > SPAWN_INTERVAL_MILLIS {
            self.spawn_time = now;

            let pos = glam::vec3(self.background.get_pos().0, self.background.get_pos().1, self.background.get_pos().2);

            let ring = Ring::new(&PlayingField::calc_random_ring_position(&pos, &self.size), Rc::clone(&update_context.resource_manager));
            self.rings.push(ring);
        }
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

                        for idx in to_remove {
                            self.total_score += self.rings[idx].get_score();
                            self.rings.remove(idx);
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
