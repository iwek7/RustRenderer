use crate::engine::api::game_object::GameObject;
use crate::engine::api::render_util::RenderUtil;

pub struct TileMap {
    tiled_map: tiled::Map,
}

impl TileMap {
    pub fn new(tiled_map: tiled::Map) -> TileMap {
        TileMap { tiled_map }
    }
}

impl GameObject for TileMap {
    fn render(&mut self, render_util: &RenderUtil) {
    }
}