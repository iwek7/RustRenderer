use std::rc::Rc;
use crate::api::drawable::Drawable;
use crate::{ create_rect_coords_colored};
use crate::api::resource_manager::ResourceManager;
use crate::maths::quadrangle::Quadrangle;
use crate::maths::vertex::ColoredVertexData;
use crate::renderer::RenderUtil;

pub struct OsuGame {
    test_quad: Quadrangle<ColoredVertexData>
}

impl OsuGame {
    pub fn new(resource_manager: Rc<ResourceManager>) -> OsuGame {
        let shader = resource_manager.fetch_shader_program("shaders/triangle");
        let test_quad = Quadrangle::new(
            create_rect_coords_colored((-2.0, -2.0, 0.0), (2.0, 2.0), (0.5, 0.5, 0.5, 1.0)),
            [0, 1, 3, 1, 2, 3],
            shader,
            None,
        );

        OsuGame {
            test_quad
        }
    }
}

impl<'a> Drawable for OsuGame {
    fn render(&self, render_util: &RenderUtil)
    {
        self.test_quad.render(render_util);
    }
}