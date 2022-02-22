use std::rc::Rc;
use crate::engine::api::drawable::Drawable;
use crate::engine::api::engine_utilities::EngineUtilities;
use crate::engine::api::render_util::RenderUtil;

pub struct PacmanGame {
    // map: tiled::Map
}

impl PacmanGame {
    pub(crate) fn new(engine_utilities: Rc<EngineUtilities>) -> PacmanGame {

        // let map = engine_utilities.get_resource_manager().fetch_tiled_map("pacman/tiled/Pacman.tmx");
        //


        PacmanGame{
            // map
        }
    }

}

impl Drawable for PacmanGame {
    fn render(&mut self, render_util: &RenderUtil) {

    }
}