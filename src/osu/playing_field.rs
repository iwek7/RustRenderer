use std::rc::Rc;

use sdl2::event::Event;

use crate::{create_rect_coords, create_rect_coords_colored, create_rect_coords_colored_deprecated, create_rect_coords_deprecated};
use crate::api::colour::Colour;
use crate::api::drawable::{Drawable, UpdateContext};
use crate::api::resource_manager::ResourceManager;
use crate::maths::quadrangle::Quadrangle;
use crate::maths::vertex::{ColoredVertexData, TexturedVertexData};
use crate::opengl_context::OpenglContext;
use crate::osu::ring::Ring;
use crate::renderer::RenderUtil;

pub struct PlayingField {
    background: Quadrangle<TexturedVertexData>,
    rings: Vec<Ring>,
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

        let test_ring = Ring::new(&glam::vec3(-10.0, -10.0, 0.0), resource_manager);

        PlayingField {
            background,
            rings: vec!(test_ring),
        }
    }
}

impl Drawable for PlayingField {
    fn render(&self, render_util: &RenderUtil) {
        self.background.render(render_util);
        self.rings.iter().for_each(|ring| ring.render(render_util));
    }

    fn handle_event(&mut self, event: &Event, context: &OpenglContext, update_context: &UpdateContext) {
        match context.sdl_space_to_world_space_at_z0(update_context.get_sdl_mouse_position(), &update_context.get_camera_config()) {
            None => {}
            Some(world_mouse_position) => {
                match event {
                    sdl2::event::Event::MouseButtonDown { .. } => {
                        for ring in self.rings.iter() {
                            if ring.contains_point(&world_mouse_position) {
                                println!("INSIDE")
                            } else {
                                println!("OUTSIDE")
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        self.rings.iter_mut().for_each(|ring| ring.handle_event(event, context, update_context));
    }
}
