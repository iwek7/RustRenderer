use std::rc::Rc;

use crate::api::drawable::Drawable;
use crate::api::resource_manager::ResourceManager;
use crate::create_rect_coords_colored;
use crate::maths::circle::Circle;
use crate::maths::quadrangle::Quadrangle;
use crate::maths::vertex::ColoredVertexData;
use crate::renderer::RenderUtil;

pub struct OsuGame {
    test_quad: Quadrangle<ColoredVertexData>,
    test_circle: Circle
}

impl OsuGame {
    pub fn new(resource_manager: Rc<ResourceManager>) -> OsuGame {
        let shader = resource_manager.fetch_shader_program("shaders/triangle");
        let test_quad = Quadrangle::new(
            create_rect_coords_colored((-2.0, -2.0, 0.0), (2.0, 2.0), (0.5, 0.5, 0.5, 1.0)),
            [0, 1, 3, 1, 2, 3],
            Rc::clone(&shader),
            None,
        );

        let test_circle = Circle::new_colored(
            glam::vec3(-2.0, -2.0, 0.0),
            glam::vec4(0.5, 0.5, 0.5, 1.0),
            1.0,
            Rc::clone(&shader)
        );

        OsuGame {
            test_quad,
            test_circle,
        }
    }
}

impl<'a> Drawable for OsuGame {
    fn render(&self, render_util: &RenderUtil)
    {
        self.test_circle.render(render_util);
    }
}