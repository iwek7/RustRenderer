use crate::engine::api::game_object::{BaseGameObject, GameObject};
use crate::engine::api::render_util::RenderUtil;

pub struct TileMap {
    base_game_object: BaseGameObject,
    tiled_map: tiled::Map,
}

impl TileMap {
    pub fn new(tiled_map: tiled::Map) -> TileMap {
        TileMap {
            base_game_object: BaseGameObject::new(),
            tiled_map,
        }
    }
}

impl GameObject for TileMap {
    fn render(&mut self, render_util: &RenderUtil) {}

    fn base_game_object(&mut self) -> &mut BaseGameObject {
        &mut self.base_game_object
    }
}