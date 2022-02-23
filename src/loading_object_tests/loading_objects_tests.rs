use std::rc::Rc;
use crate::engine::api::game_object::GameObject;
use crate::engine::api::engine_utilities::EngineUtilities;
use crate::engine::api::maths::rectangle::Rectangle;
use crate::engine::api::render_util::RenderUtil;
use crate::engine::serialization::{GameObjectDto, GameObjectMapper, ProjectDto};
use crate::engine::api::maths::vertex::ColoredVertexDataLayout;

pub struct LoadingObjectsTests {
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
            rects
        }
    }
}

impl GameObject for LoadingObjectsTests {
    fn render(&mut self, render_util: &RenderUtil) {
        self.rects.iter_mut().for_each(|rect| rect.render(render_util));
    }
}