#![feature(duration_consts_2)]

use engine::api::colour::Colour;
use engine::api::maths::vertex;
use engine::api::maths::vertex::{ColoredVertexDataLayout, TexturedVertexDataLayout};
use engine::api::texture::SpriteCoords;
use osu::osu_game::OsuGame;

use crate::chess::chess_game::ChessGame;
use crate::engine::api::colour::WHITE;
use crate::engine::engine::Engine;
use crate::games_root::GamesRoot;

mod chess;
mod engine;
mod games_root;
mod osu;

fn main() {

    let mut engine = Engine::new();
    let osu_game = OsuGame::new(engine.get_engine_utilities());
    let chess_game = ChessGame::new(engine.get_engine_utilities());
    let games_root = GamesRoot::new(vec![Box::new(osu_game), Box::new(chess_game)]);
    engine.set_game(games_root);
    engine.start();
}

// todo: use rectangle instead of those
#[deprecated(note="use rectangle struct instead")]
fn create_rect_coords_deprecated(pos: (f32, f32, f32), size: (f32, f32), sprite_coords: &SpriteCoords) -> [TexturedVertexDataLayout; 4] {
    return [
        vertex::TexturedVertexDataLayout { pos: (pos.0 + size.0, pos.1 + size.1, pos.2).into(), clr: (1.0, 1.0, 0.0, 1.0).into(), tx_coords: sprite_coords.top_right.into() },
        vertex::TexturedVertexDataLayout { pos: (pos.0 + size.0, pos.1, pos.2).into(), clr: (1.0, 1.0, 0.0, 1.0).into(), tx_coords: sprite_coords.bottom_right.into() },
        vertex::TexturedVertexDataLayout { pos: (pos.0, pos.1, pos.2).into(), clr: (1.0, 1.0, 1.0, 1.0).into(), tx_coords: sprite_coords.bottom_left.into() },
        vertex::TexturedVertexDataLayout { pos: (pos.0, pos.1 + size.1, pos.2).into(), clr: (1.0, 1.0, 1.0, 1.0).into(), tx_coords: sprite_coords.top_left.into() },
    ];
}

#[deprecated(note="use rectangle struct instead")]
fn create_rect_coords(pos: &glam::Vec3, size: &glam::Vec2, sprite_coords: &SpriteCoords) -> [TexturedVertexDataLayout; 4] {
    create_colored_rect_coords(pos, size, sprite_coords, &WHITE)
}

#[deprecated(note="use rectangle struct instead")]
fn create_colored_rect_coords(pos: &glam::Vec3, size: &glam::Vec2, sprite_coords: &SpriteCoords, clr: &Colour) -> [TexturedVertexDataLayout; 4] {
    return [
        vertex::TexturedVertexDataLayout { pos: (pos.x + size.x, pos.y + size.y, pos.z).into(), clr: clr.clone().into(), tx_coords: sprite_coords.top_right.into() },
        vertex::TexturedVertexDataLayout { pos: (pos.x + size.x, pos.y, pos.z).into(), clr: clr.clone().into(), tx_coords: sprite_coords.bottom_right.into() },
        vertex::TexturedVertexDataLayout { pos: (pos.x, pos.y, pos.z).into(), clr: clr.clone().into(), tx_coords: sprite_coords.bottom_left.into() },
        vertex::TexturedVertexDataLayout { pos: (pos.x, pos.y + size.y, pos.z).into(), clr: clr.clone().into(), tx_coords: sprite_coords.top_left.into() },
    ];
}

// todo: size should be u32
#[deprecated(note="use rectangle struct instead")]
fn create_rect_coords_colored_deprecated(pos: (f32, f32, f32), size: (f32, f32), clr: (f32, f32, f32, f32)) -> [ColoredVertexDataLayout; 4] {
    return [
        vertex::ColoredVertexDataLayout { pos: (pos.0 + size.0, pos.1 + size.1, pos.2).into(), clr: clr.into() },
        vertex::ColoredVertexDataLayout { pos: (pos.0 + size.0, pos.1, pos.2).into(), clr: clr.into() },
        vertex::ColoredVertexDataLayout { pos: (pos.0, pos.1, pos.2).into(), clr: clr.into() },
        vertex::ColoredVertexDataLayout { pos: (pos.0, pos.1 + size.1, pos.2).into(), clr: clr.into() },
    ];
}

#[deprecated(note="use rectangle struct instead")]
fn create_rect_coords_colored(pos: &glam::Vec3, size: &glam::Vec2, clr: Colour) -> [ColoredVertexDataLayout; 4] {
    return [
        vertex::ColoredVertexDataLayout { pos: (pos.x + size.x, pos.y + size.y, pos.z).into(), clr: clr.into() },
        vertex::ColoredVertexDataLayout { pos: (pos.x + size.x, pos.y, pos.z).into(), clr: clr.into() },
        vertex::ColoredVertexDataLayout { pos: (pos.x, pos.y, pos.z).into(), clr: clr.into() },
        vertex::ColoredVertexDataLayout { pos: (pos.x, pos.y + size.y, pos.z).into(), clr: clr.into() },
    ];
}



