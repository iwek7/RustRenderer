use std::rc::Rc;
use crate::engine::api::game_object::{BaseGameObject, GameObject};
use crate::engine::api::engine_utilities::EngineUtilities;
use crate::engine::api::render_util::RenderUtil;

pub struct PacmanGame {
    // map: tiled::Map
    base_game_object: BaseGameObject,
}

impl PacmanGame {
    pub(crate) fn new(engine_utilities: Rc<EngineUtilities>) -> PacmanGame {

        // let map = engine_utilities.get_resource_manager().fetch_tiled_map("pacman/tiled/Pacman.tmx");
        //


        PacmanGame{
            // map
            base_game_object: BaseGameObject::new()
        }
    }

}

impl GameObject for PacmanGame {
    fn render(&mut self, render_util: &RenderUtil) {

    }

    fn base_game_object(&mut self) -> &mut BaseGameObject {
        &mut self.base_game_object
    }
}