use crate::api::drawable::Drawable;
use crate::renderer::RenderUtil;

pub struct OsuGame {
    //test_quad: Quadrangle<'a, ColoredVertexData>
}

impl OsuGame {
    pub fn new() -> OsuGame {
        OsuGame {}
    }
}

impl<'a> Drawable for OsuGame {
    fn render(&self, render_util: &RenderUtil) {
        println!("RENDERING OSU!!!")
    }
}