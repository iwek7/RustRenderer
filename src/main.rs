#![feature(duration_consts_2)]

use engine::api::maths::vertex;
use engine::api::maths::vertex::{TexturedVertexDataLayout};
use engine::api::texture::TextureCoords;
use osu::osu_game::OsuGame;

use crate::chess::chess_game::ChessGame;
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