use std::rc::Rc;
use sdl2::event::Event;

use crate::engine::api::colour::{Colour, WHITE};
use crate::engine::api::drawable::{Drawable, UpdateContext};
use crate::engine::api::maths::quadrangle::Quadrangle;
use crate::engine::api::maths::vertex;
use crate::engine::api::maths::vertex::{ColoredVertexDataLayout, TexturedVertexDataLayout, VertexShaderDataLayout};
use crate::engine::api::render_util::RenderUtil;
use crate::engine::api::texture::{SpriteCoords, Texture};
use crate::engine::opengl_context::OpenglContext;
use crate::engine::rendering::material::{Material, UniformKind};

const RECT_INDICES: [i32; 6] = [0, 1, 3, 1, 2, 3];

pub struct Rectangle<T> where T: VertexShaderDataLayout {
    quad: Quadrangle<T>,
}


impl Rectangle<ColoredVertexDataLayout> {
    pub fn new_colored(bottom_left: glam::Vec3, size: glam::Vec2, material: Material, clr: Colour) -> Rectangle<ColoredVertexDataLayout> {
        // todo assert that size is possitive
        Rectangle {
            quad: Quadrangle::new(
                create_colored_vertex_data_layout(&bottom_left, &size, clr),
                RECT_INDICES.clone(),
                material,
                None,
            )
        }
    }
}

impl Rectangle<TexturedVertexDataLayout> {
    pub fn new_textured(bottom_left: &glam::Vec3, size: &glam::Vec2, material: Material, texture: Rc<Texture>) -> Rectangle<TexturedVertexDataLayout> {
        // todo assert that size is possitive

        Rectangle {
            quad: Quadrangle::new(
                create_textured_vertex_data_layout(
                    bottom_left,
                    size,
                    &texture.topology.get_sprite_coords(0, 0).unwrap(),
                    &WHITE,
                ),
                RECT_INDICES.clone(),
                material,
                Some(texture),
            )
        }
    }
}

impl<T: VertexShaderDataLayout> Rectangle<T> {
    pub fn set_material_variable(&mut self, name: &str, kind: UniformKind) {
        self.quad.set_material_variable(name, kind);
    }
}

fn create_colored_vertex_data_layout(pos: &glam::Vec3, size: &glam::Vec2, clr: Colour) -> [ColoredVertexDataLayout; 4] {
    return [
        vertex::ColoredVertexDataLayout { pos: (pos.x + size.x, pos.y + size.y, pos.z).into(), clr: clr.into() },
        vertex::ColoredVertexDataLayout { pos: (pos.x + size.x, pos.y, pos.z).into(), clr: clr.into() },
        vertex::ColoredVertexDataLayout { pos: (pos.x, pos.y, pos.z).into(), clr: clr.into() },
        vertex::ColoredVertexDataLayout { pos: (pos.x, pos.y + size.y, pos.z).into(), clr: clr.into() },
    ];
}

fn create_textured_vertex_data_layout(pos: &glam::Vec3, size: &glam::Vec2, sprite_coords: &SpriteCoords, clr: &Colour) -> [TexturedVertexDataLayout; 4] {
    return [
        vertex::TexturedVertexDataLayout { pos: (pos.x + size.x, pos.y + size.y, pos.z).into(), clr: clr.clone().into(), tx_coords: sprite_coords.top_right.into() },
        vertex::TexturedVertexDataLayout { pos: (pos.x + size.x, pos.y, pos.z).into(), clr: clr.clone().into(), tx_coords: sprite_coords.bottom_right.into() },
        vertex::TexturedVertexDataLayout { pos: (pos.x, pos.y, pos.z).into(), clr: clr.clone().into(), tx_coords: sprite_coords.bottom_left.into() },
        vertex::TexturedVertexDataLayout { pos: (pos.x, pos.y + size.y, pos.z).into(), clr: clr.clone().into(), tx_coords: sprite_coords.top_left.into() },
    ];
}

impl<T: VertexShaderDataLayout> Drawable for Rectangle<T> {
    fn render(&mut self, render_util: &RenderUtil) {
        self.quad.render(render_util)
    }

    fn update(&mut self, update_context: &UpdateContext) {
        self.quad.update(update_context)
    }

    fn handle_event(&mut self, event: &Event, context: &OpenglContext, update_context: &UpdateContext) {
        self.quad.handle_event(event, context, update_context)
    }
}