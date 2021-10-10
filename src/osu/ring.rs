use std::rc::Rc;
use sdl2::event::Event;
use crate::api::drawable::{Drawable, UpdateContext};
use crate::api::resource_manager::ResourceManager;
use crate::create_rect_coords;
use crate::maths::circle::Circle;
use crate::maths::quadrangle::Quadrangle;
use crate::maths::shapes_common::Area;
use crate::maths::vertex::TexturedVertexData;
use crate::opengl_context::OpenglContext;
use crate::renderer::RenderUtil;

pub const RING_RADIUS : f32 = 0.9;

pub struct Ring {
    hitbox: Circle,
    tx: Quadrangle<TexturedVertexData>,
}

impl Ring {
    pub fn new(position: &glam::Vec3, resource_manager: Rc<ResourceManager>) -> Ring {
        let tx_shader = resource_manager.fetch_shader_program("osu/shaders/texture");
        let clr_shader = resource_manager.fetch_shader_program("osu/shaders/colour");
        let ring_tx = resource_manager.fetch_texture("osu/textures/ring.png");


        let tx_position = glam::vec3(position.x - RING_RADIUS, position.y - RING_RADIUS, position.z);
        let tx = Quadrangle::new(
            create_rect_coords(&tx_position, &glam::vec2(RING_RADIUS * 2.0, RING_RADIUS * 2.0), &ring_tx.topology.get_sprite_coords(0, 0).unwrap()),
            [0, 1, 3, 1, 2, 3],
            tx_shader,
            Some(ring_tx),
        );

        let hitbox = Circle::new_colored(
            position,
            glam::vec4(0.5, 0.5, 0.5, 1.0),
            RING_RADIUS,
            clr_shader,
        );

        Ring {
            tx,
            hitbox
        }
    }

    pub fn contains_point(&self, position: &glam::Vec3) -> bool {
       self.hitbox.contains_point(&(position.x, position.y))
    }

    pub fn get_score(&self) -> i32 {
        1
    }
}

impl Drawable for Ring {
    fn render(&self, render_util: &RenderUtil) {
        self.tx.render(render_util);
        // self.hitbox.render(render_util);
    }

}