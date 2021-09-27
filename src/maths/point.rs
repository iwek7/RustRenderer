use crate::maths::vertex::VertexShaderDataConfigurer;
use crate::render_gl::shape_drawing_component::ShapeDrawingComponent;
use crate::render_gl;
use crate::maths::triangle::Drawable;
use crate::renderer::RenderUtil;
use crate::glam_utils::to_glam_vec;
use std::ops::Sub;

pub struct Point<'a, T> where T: VertexShaderDataConfigurer {
    drawing_component: ShapeDrawingComponent<'a, T>,
    vertices: [T; 1],
}

impl<'a, T: VertexShaderDataConfigurer> Point<'a, T> {
    pub fn new(vertices: [T; 1], program: &render_gl::Program) -> Point<T> {
        let drawing_component = ShapeDrawingComponent::new(
            &vertices,
            &[0],
            None,
            program,
        );
        return Point {
            drawing_component,
            vertices
        }
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

impl<'a, T: VertexShaderDataConfigurer> Drawable for Point<'a, T> {
    fn render(&self, render_util: &RenderUtil) {
        self.drawing_component.render(1, gl::POINTS, self.get_pos(), render_util)
    }
}
