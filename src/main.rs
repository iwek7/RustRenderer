#![feature(drain_filter)]
#![feature(const_fn_floating_point_arithmetic)]

use engine::api::maths::vertex;
use osu::osu_game::OsuGame;

use crate::chess::chess_game::ChessGame;
use crate::engine::engine::Engine;
use crate::games_root::GamesRoot;
use crate::submarine::submarine_game::SubmarineGame;

mod chess;
mod engine;
mod games_root;
mod osu;
mod submarine;

fn main() {
    let mut engine = Engine::new();
    let osu_game = OsuGame::new(engine.get_engine_utilities());
    let chess_game = ChessGame::new(engine.get_engine_utilities());
    let submarine_game = SubmarineGame::new(engine.get_engine_utilities());
    let games_root = GamesRoot::new(
        vec![
            Box::new(osu_game),
            Box::new(chess_game),
            Box::new(submarine_game),
        ]
    );
    engine.set_game(games_root);
    engine.start();
}