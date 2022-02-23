#![feature(drain_filter)]
#![feature(const_fn_floating_point_arithmetic)]
#[cfg(feature = "serde_derive")]
use engine::api::maths::vertex;
use osu::osu_game::OsuGame;

use crate::chess::chess_game::ChessGame;
use crate::engine::engine::Engine;
use crate::games_root::GamesRoot;
use crate::loading_object_tests::loading_objects_tests::LoadingObjectsTests;
use crate::pacman::pacman_game::PacmanGame;
use crate::submarine::submarine_game::SubmarineGame;

mod chess;
mod engine;
mod games_root;
mod osu;
mod submarine;
mod pacman;
mod loading_object_tests;

fn main() {
    let mut engine = Engine::new();
    let osu_game = OsuGame::new(engine.get_engine_utilities());
    let chess_game = ChessGame::new(engine.get_engine_utilities());
    let submarine_game = SubmarineGame::new(engine.get_engine_utilities());
    let pacman_game = PacmanGame::new(engine.get_engine_utilities());
    let loading_object_tests = LoadingObjectsTests::new(engine.get_engine_utilities());
    let games_root = GamesRoot::new(
        vec![
            // Box::new(osu_game),
            // Box::new(chess_game),
            // Box::new(submarine_game),
            // Box::new(pacman_game),
            Box::new(loading_object_tests)
        ]
    );
    engine.set_game(games_root);
    engine.start();
}