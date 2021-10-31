use std::ops::Sub;
use std::rc::Rc;

use crate::engine::api::drawable::Drawable;
use crate::engine::api::maths::vertex::VertexShaderDataConfigurer;
use crate::engine::api::render_util::RenderUtil;
use crate::engine::glam_utils::to_glam_vec;
use crate::engine::rendering;
use crate::engine::rendering::material::Material;
use crate::engine::rendering::shape_drawing_component::ShapeDrawingComponent;

/*
example:
```
Point::new(
            [ColoredVertexData { pos: (0.0, -0.0, 0.0).into(), clr: (0.0, 0.0, 0.0, 1.0).into() }, ],
            Rc::clone(&shader_program),
        )
        ```
 */
pub struct Point<T> where T: VertexShaderDataConfigurer {
    drawing_component: ShapeDrawingComponent<T>,
    vertices: [T; 1],
    material: Material,
}

impl<T: VertexShaderDataConfigurer> Point<T> {
    pub fn new(vertices: [T; 1], material: Material) -> Point<T> {
        let drawing_component = ShapeDrawingComponent::new(
            &vertices,
            &[0],
            None,
        );
        return Point {
            drawing_component,
            vertices,
            material,
        };
    }

    pub fn get_pos(&self) -> glam::Vec3 {
        to_glam_vec(&self.vertices[0].get_pos_deprecated())
    }

    // this code is duplicated
    pub fn move_by(&mut self, offset: &glam::Vec3) {
        self.vertices[0].transpose(offset);
        self.drawing_component.bind_data(&self.vertices)
    }

    pub fn move_to(&mut self, final_pos: &glam::Vec3) {
        let current_pos = self.vertices[0].get_pos();
        self.move_by(&final_pos.sub(current_pos));
    }
}

impl<'a, T: VertexShaderDataConfigurer> Drawable for Point<T> {
    fn render(&mut self, render_util: &RenderUtil) {
        self.drawing_component.render(
            1,
            gl::POINTS,
            self.get_pos(),
            render_util,
            &mut self.material,
        )
    }
}
