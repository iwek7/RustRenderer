use std::rc::Rc;
use crate::engine::api::drawable::Drawable;
use crate::engine::api::engine_utilities::EngineUtilities;
use crate::engine::api::maths::rectangle::Rectangle;
use crate::engine::api::render_util::RenderUtil;
use crate::engine::serialization::{GameObjectDto, GameObjectMapper};
use crate::engine::api::maths::vertex::ColoredVertexDataLayout;

pub struct LoadingObjectsTests {
    rect: Rectangle<ColoredVertexDataLayout>
}

impl LoadingObjectsTests {
    pub(crate) fn new(engine_utils: Rc<EngineUtilities>) -> LoadingObjectsTests {
        let buf_reader = engine_utils.get_resource_manager().read_file("loading-objects-tests/test_object.json");
        let base_object: GameObjectDto = serde_json::from_reader(buf_reader).unwrap();

        println!("object {:?}", base_object);
        let mapper = GameObjectMapper::new(engine_utils.clone());
        let rect: Rectangle<ColoredVertexDataLayout> = mapper.map_dto_to_game_object(base_object);



        LoadingObjectsTests {
            rect
        }
    }
}

impl Drawable for LoadingObjectsTests {
    fn render(&mut self, render_util: &RenderUtil) {
        self.rect.render(render_util)
    }
}