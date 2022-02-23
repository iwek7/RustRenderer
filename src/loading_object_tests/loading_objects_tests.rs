use std::rc::Rc;
use crate::engine::api::game_object::{BaseGameObject, GameObject};
use crate::engine::api::engine_utilities::EngineUtilities;
use crate::engine::api::maths::rectangle::Rectangle;
use crate::engine::api::render_util::RenderUtil;
use crate::engine::serialization::{GameObjectMapper, ProjectDto};
use crate::engine::api::maths::vertex::ColoredVertexDataLayout;

pub struct LoadingObjectsTests {
    base_game_object: BaseGameObject,
    rects: Vec<Box<Rectangle<ColoredVertexDataLayout>>>,
}

impl LoadingObjectsTests {
    pub(crate) fn new(engine_utils: Rc<EngineUtilities>) -> LoadingObjectsTests {
        let buf_reader = engine_utils.get_resource_manager().read_file("loading-objects-tests/test_object.json");
        let project: ProjectDto = serde_json::from_reader(buf_reader).unwrap();

        println!("project {:?}", project);
        let mapper = GameObjectMapper::new(engine_utils.clone());

        let rects = project.get_objects().iter()
            .map(|dto| mapper.map_dto_to_game_object(dto))
            .map(|object| Box::new(object))
            .collect();
        LoadingObjectsTests {
            base_game_object: BaseGameObject::new(),
            rects
        }
    }
}

impl GameObject for LoadingObjectsTests {
    fn render(&mut self, render_util: &RenderUtil) {
        self.rects.iter_mut().for_each(|rect| rect.render(render_util));
    }

    fn base_game_object(&mut self) -> &mut BaseGameObject {
        &mut self.base_game_object
    }
}