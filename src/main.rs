use crate::chess::chess_game::ChessGame;
use crate::maths::vertex;
use crate::maths::vertex::{ColoredVertexData, TexturedVertexData};
use crate::opengl_context::OpenglContext;
use crate::texture::{SpriteCoords};

pub mod render_gl;
pub mod resources;
pub mod renderer;
pub mod opengl_context;
pub mod texture;
pub mod glam_utils;

mod maths;
mod chess;
mod engine;
mod api;

fn main() {
    let context = OpenglContext::init();
    let mut event_pump = context.sdl.event_pump().unwrap();
    let mut renderer = renderer::Renderer::new(&context);
    ChessGame::play(&mut renderer, &mut event_pump, &context);
}

// todo: this should be encapsulated into shapes
fn create_rect_coords_in_opengl_space(
    context: &OpenglContext, pos: (i32, i32, i32), size: (i32, i32), sprite_coords: &SpriteCoords) -> [TexturedVertexData; 4] {
    return [
        vertex::TexturedVertexData { pos: context.engine_to_opengl_space(&(pos.0 + size.0, pos.1 + size.1, pos.2)).into(), clr: (1.0, 1.0, 0.0, 1.0).into(), tx_coords: sprite_coords.top_right.into() },
        vertex::TexturedVertexData { pos: context.engine_to_opengl_space(&(pos.0 + size.0, pos.1, pos.2)).into(), clr: (1.0, 1.0, 0.0, 1.0).into(), tx_coords: sprite_coords.bottom_right.into() },
        vertex::TexturedVertexData { pos: context.engine_to_opengl_space(&(pos.0, pos.1, pos.2)).into(), clr: (1.0, 1.0, 1.0, 1.0).into(), tx_coords: sprite_coords.bottom_left.into() },
        vertex::TexturedVertexData { pos: context.engine_to_opengl_space(&(pos.0, pos.1 + size.1, pos.2)).into(), clr: (1.0, 1.0, 1.0, 1.0).into(), tx_coords: sprite_coords.top_left.into() },
    ];
}

// todo: size should be u32
fn create_rect_coords_in_opengl_space_colored(
    context: &OpenglContext, pos: (i32, i32, i32), size: (i32, i32), clr: (f32, f32, f32, f32)) -> [ColoredVertexData; 4] {
    return [
        vertex::ColoredVertexData { pos: context.engine_to_opengl_space(&(pos.0 + size.0, pos.1 + size.1, pos.2)).into(), clr: clr.into() },
        vertex::ColoredVertexData { pos: context.engine_to_opengl_space(&(pos.0 + size.0, pos.1, pos.2)).into(), clr: clr.into() },
        vertex::ColoredVertexData { pos: context.engine_to_opengl_space(&(pos.0, pos.1, pos.2)).into(), clr: clr.into() },
        vertex::ColoredVertexData { pos: context.engine_to_opengl_space(&(pos.0, pos.1 + size.1, pos.2)).into(), clr: clr.into() },
    ];
}



