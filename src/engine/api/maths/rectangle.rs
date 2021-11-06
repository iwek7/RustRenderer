use glam::Vec3;
use sdl2::event::Event;

use crate::engine::api::colour::{Colour, WHITE};
use crate::engine::api::drawable::{Drawable, UpdateContext};
use crate::engine::api::maths::quadrangle::Quadrangle;
use crate::engine::api::maths::shapes_common::Area;
use crate::engine::api::maths::vertex;
use crate::engine::api::maths::vertex::{ColoredVertexDataLayout, TexturedVertexDataLayout, VertexShaderDataLayout};
use crate::engine::api::render_util::RenderUtil;
use crate::engine::api::texture::{Sprite, TextureCoords};
use crate::engine::opengl_context::OpenglContext;
use crate::engine::rendering::material::{Material, UniformKind};

const RECT_INDICES: [i32; 6] = [0, 1, 3, 1, 2, 3];

pub struct Rectangle<T> where T: VertexShaderDataLayout {
    quad: Quadrangle<T>,
}


impl Rectangle<ColoredVertexDataLayout> {
    pub fn new_colored(bottom_left: &glam::Vec3, size: &glam::Vec2, material: Material, clr: Colour) -> Rectangle<ColoredVertexDataLayout> {
        // todo assert that size is possitive
        Rectangle {
            quad: Quadrangle::new(
                create_colored_vertex_data_layout(size, clr),
                RECT_INDICES.clone(),
                material,
                None,
                bottom_left.clone(),
            )
        }
    }
}

impl Rectangle<TexturedVertexDataLayout> {
    pub fn new_textured(bottom_left: &glam::Vec3, size: &glam::Vec2, material: Material, sprite: Sprite) -> Rectangle<TexturedVertexDataLayout> {
        // todo assert that size is possitive

        let mut quad = Quadrangle::new(
            create_textured_vertex_data_layout(
                size,
                &sprite.get_texture_coords(),
                &WHITE,
            ),
            RECT_INDICES.clone(),
            material,
            Some(sprite),
            bottom_left.clone(),
        );

        quad.set_scaling_point(glam::vec3(size.x / 2.0, size.y / 2.0, 0.0));

        Rectangle {
            quad
        }
    }

    pub fn new_from_spritesheet(bottom_left: &glam::Vec3, size: &glam::Vec2, material: Material, sprite: Sprite, sprite_sheet_row: u32, sprite_sheet_col: u32) -> Rectangle<TexturedVertexDataLayout> {

       let mut quad = Quadrangle::new(
           create_textured_vertex_data_layout(
               size,
               &sprite.get_texture_coords_from_spritesheet(sprite_sheet_row, sprite_sheet_col),
               &WHITE,
           ),
           RECT_INDICES.clone(),
           material,
           Some(sprite),
           bottom_left.clone(),
       );

        quad.set_scaling_point(glam::vec3(size.x / 2.0, size.y / 2.0, 0.0));

        Rectangle {
            quad
        }
    }
}

impl<T: VertexShaderDataLayout> Rectangle<T> {
    pub fn set_material_variable(&mut self, name: &str, kind: UniformKind) {
        self.quad.set_material_variable(name, kind);
    }
}

fn create_colored_vertex_data_layout(size: &glam::Vec2, clr: Colour) -> [ColoredVertexDataLayout; 4] {
    return [
        vertex::ColoredVertexDataLayout { pos: (size.x, size.y, 0.0).into(), clr: clr.into() },
        vertex::ColoredVertexDataLayout { pos: (size.x, 0.0, 0.0).into(), clr: clr.into() },
        vertex::ColoredVertexDataLayout { pos: (0.0, 0.0, 0.0).into(), clr: clr.into() },
        vertex::ColoredVertexDataLayout { pos: (0.0, size.y, 0.0).into(), clr: clr.into() },
    ];
}

fn create_textured_vertex_data_layout(size: &glam::Vec2, sprite_coords: &TextureCoords, clr: &Colour) -> [TexturedVertexDataLayout; 4] {
    return [
        vertex::TexturedVertexDataLayout { pos: (size.x, size.y, 0.0).into(), clr: clr.clone().into(), tx_coords: sprite_coords.top_right.into() },
        vertex::TexturedVertexDataLayout { pos: (size.x, 0.0, 0.0).into(), clr: clr.clone().into(), tx_coords: sprite_coords.bottom_right.into() },
        vertex::TexturedVertexDataLayout { pos: (0.0, 0.0, 0.0).into(), clr: clr.clone().into(), tx_coords: sprite_coords.bottom_left.into() },
        vertex::TexturedVertexDataLayout { pos: (0.0, size.y, 0.0).into(), clr: clr.clone().into(), tx_coords: sprite_coords.top_left.into() },
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

impl<T: VertexShaderDataLayout> Area for Rectangle<T> {
    fn contains_point(&self, point: &(f32, f32)) -> bool {
        self.quad.contains_point(point)
    }

    fn area(&self) -> f32 {
        self.quad.area()
    }

    fn num_vertices(&self) -> usize {
        self.quad.num_vertices()
    }

    fn get_pos(&self) -> &glam::Vec3 {
        self.quad.get_pos()
    }

    fn move_to(&mut self, final_position: Vec3) {
        self.quad.move_to(final_position)
    }

    fn move_by(&mut self, offset: Vec3) {
        self.quad.move_by(offset)
    }

    fn get_scale(&self) -> &Vec3 {
        self.quad.get_scale()
    }

    fn set_scale(&mut self, new_scale: Vec3) {
        self.quad.set_scale(new_scale)
    }
}