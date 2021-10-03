use std::rc::Rc;
use osu::osu_game::OsuGame;
use crate::api::resource_manager::ResourceManager;
use crate::chess::chess_game::ChessGame;
use crate::engine::Engine;
use crate::games_root::GamesRoot;
use crate::maths::vertex;
use crate::maths::vertex::{ColoredVertexData, TexturedVertexData};
use crate::opengl_context::OpenglContext;
use crate::texture::SpriteCoords;

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
mod games_root;
mod osu;

fn main() {
    let opengl_context = OpenglContext::init();
    let osu_game = OsuGame::new();
    let mut resource_manager = Rc::new(ResourceManager::new());
    let chess_game = ChessGame::new(Rc::clone(&resource_manager));
    let games_root = GamesRoot::new(vec![Box::new(osu_game), Box::new(chess_game)]);
    let mut engine = Engine::new(games_root, resource_manager, opengl_context);
    engine.start();
}

// todo: this should be encapsulated into shapes
fn create_rect_coords(pos: (f32, f32, f32), size: (f32, f32), sprite_coords: &SpriteCoords) -> [TexturedVertexData; 4] {
    return [
        vertex::TexturedVertexData { pos: (pos.0 + size.0, pos.1 + size.1, pos.2).into(), clr: (1.0, 1.0, 0.0, 1.0).into(), tx_coords: sprite_coords.top_right.into() },
        vertex::TexturedVertexData { pos: (pos.0 + size.0, pos.1, pos.2).into(), clr: (1.0, 1.0, 0.0, 1.0).into(), tx_coords: sprite_coords.bottom_right.into() },
        vertex::TexturedVertexData { pos: (pos.0, pos.1, pos.2).into(), clr: (1.0, 1.0, 1.0, 1.0).into(), tx_coords: sprite_coords.bottom_left.into() },
        vertex::TexturedVertexData { pos: (pos.0, pos.1 + size.1, pos.2).into(), clr: (1.0, 1.0, 1.0, 1.0).into(), tx_coords: sprite_coords.top_left.into() },
    ];
}

// todo: size should be u32
fn create_rect_coords_colored(pos: (f32, f32, f32), size: (f32, f32), clr: (f32, f32, f32, f32)) -> [ColoredVertexData; 4] {
    return [
        vertex::ColoredVertexData { pos: (pos.0 + size.0, pos.1 + size.1, pos.2).into(), clr: clr.into() },
        vertex::ColoredVertexData { pos: (pos.0 + size.0, pos.1, pos.2).into(), clr: clr.into() },
        vertex::ColoredVertexData { pos: (pos.0, pos.1, pos.2).into(), clr: clr.into() },
        vertex::ColoredVertexData { pos: (pos.0, pos.1 + size.1, pos.2).into(), clr: clr.into() },
    ];
}



