use std::rc::Rc;

use engine::api::colour::Colour;
use engine::api::maths::vertex;
use engine::api::maths::vertex::{ColoredVertexData, TexturedVertexData};
use engine::api::resource_manager::ResourceManager;
use engine::api::texture::SpriteCoords;
use engine::opengl_context::OpenglContext;
use osu::osu_game::OsuGame;

use crate::chess::chess_game::ChessGame;
use crate::engine::api::audio::AudioManager;
use crate::engine::api::colour::WHITE;
use crate::engine::api::engine_utilities::EngineUtilities;
use crate::engine::engine::Engine;
use crate::games_root::GamesRoot;

mod chess;
mod engine;
mod games_root;
mod osu;

fn main() {
    let opengl_context = OpenglContext::init();
    let mut resource_manager = Rc::new(ResourceManager::new());
    let audio_manager = Rc::new(AudioManager::new());
    let engine_utilities = Rc::new(EngineUtilities::new(resource_manager, audio_manager));

    let osu_game = OsuGame::new(Rc::clone(&engine_utilities));
    let chess_game = ChessGame::new(Rc::clone(&engine_utilities));
    let games_root = GamesRoot::new(vec![Box::new(osu_game), Box::new(chess_game)]);
    let mut engine = Engine::new(games_root, engine_utilities, opengl_context);
    engine.start();
}

// todo: this should be encapsulated into shapes
fn create_rect_coords_deprecated(pos: (f32, f32, f32), size: (f32, f32), sprite_coords: &SpriteCoords) -> [TexturedVertexData; 4] {
    return [
        vertex::TexturedVertexData { pos: (pos.0 + size.0, pos.1 + size.1, pos.2).into(), clr: (1.0, 1.0, 0.0, 1.0).into(), tx_coords: sprite_coords.top_right.into() },
        vertex::TexturedVertexData { pos: (pos.0 + size.0, pos.1, pos.2).into(), clr: (1.0, 1.0, 0.0, 1.0).into(), tx_coords: sprite_coords.bottom_right.into() },
        vertex::TexturedVertexData { pos: (pos.0, pos.1, pos.2).into(), clr: (1.0, 1.0, 1.0, 1.0).into(), tx_coords: sprite_coords.bottom_left.into() },
        vertex::TexturedVertexData { pos: (pos.0, pos.1 + size.1, pos.2).into(), clr: (1.0, 1.0, 1.0, 1.0).into(), tx_coords: sprite_coords.top_left.into() },
    ];
}

fn create_rect_coords(pos: &glam::Vec3, size: &glam::Vec2, sprite_coords: &SpriteCoords) -> [TexturedVertexData; 4] {
    create_colored_rect_coords(pos, size, sprite_coords, &WHITE)
}

fn create_colored_rect_coords(pos: &glam::Vec3, size: &glam::Vec2, sprite_coords: &SpriteCoords, clr: &Colour) -> [TexturedVertexData; 4] {
    return [
        vertex::TexturedVertexData { pos: (pos.x + size.x, pos.y + size.y, pos.z).into(), clr: clr.clone().into(), tx_coords: sprite_coords.top_right.into() },
        vertex::TexturedVertexData { pos: (pos.x + size.x, pos.y, pos.z).into(), clr: clr.clone().into(), tx_coords: sprite_coords.bottom_right.into() },
        vertex::TexturedVertexData { pos: (pos.x, pos.y, pos.z).into(), clr: clr.clone().into(), tx_coords: sprite_coords.bottom_left.into() },
        vertex::TexturedVertexData { pos: (pos.x, pos.y + size.y, pos.z).into(), clr: clr.clone().into(), tx_coords: sprite_coords.top_left.into() },
    ];
}

// todo: size should be u32
fn create_rect_coords_colored_deprecated(pos: (f32, f32, f32), size: (f32, f32), clr: (f32, f32, f32, f32)) -> [ColoredVertexData; 4] {
    return [
        vertex::ColoredVertexData { pos: (pos.0 + size.0, pos.1 + size.1, pos.2).into(), clr: clr.into() },
        vertex::ColoredVertexData { pos: (pos.0 + size.0, pos.1, pos.2).into(), clr: clr.into() },
        vertex::ColoredVertexData { pos: (pos.0, pos.1, pos.2).into(), clr: clr.into() },
        vertex::ColoredVertexData { pos: (pos.0, pos.1 + size.1, pos.2).into(), clr: clr.into() },
    ];
}

fn create_rect_coords_colored(pos: &glam::Vec3, size: &glam::Vec2, clr: Colour) -> [ColoredVertexData; 4] {
    return [
        vertex::ColoredVertexData { pos: (pos.x + size.x, pos.y + size.y, pos.z).into(), clr: clr.into() },
        vertex::ColoredVertexData { pos: (pos.x + size.x, pos.y, pos.z).into(), clr: clr.into() },
        vertex::ColoredVertexData { pos: (pos.x, pos.y, pos.z).into(), clr: clr.into() },
        vertex::ColoredVertexData { pos: (pos.x, pos.y + size.y, pos.z).into(), clr: clr.into() },
    ];
}



