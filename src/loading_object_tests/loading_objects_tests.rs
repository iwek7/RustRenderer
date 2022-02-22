use std::rc::Rc;
use crate::engine::api::drawable::Drawable;
use crate::engine::api::engine_utilities::EngineUtilities;
use crate::engine::api::render_util::RenderUtil;
use crate::engine::base_object::BaseObject;

pub struct LoadingObjectsTests {

}

impl LoadingObjectsTests {
    pub(crate) fn new(engine_utils: Rc<EngineUtilities>) -> LoadingObjectsTests {
        let buf_reader = engine_utils.get_resource_manager().read_file("loading-objects-tests/test_object.json");
        let base_object: BaseObject = serde_json::from_reader(buf_reader).unwrap();
        println!("object {:?}", base_object);

        LoadingObjectsTests {}
    }
}

impl Drawable for LoadingObjectsTests {
    fn render(&mut self, render_util: &RenderUtil) {
        todo!()
    }
}