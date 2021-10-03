use std::rc::Rc;
use crate::maths::shapes_common::{Area, is_point_within_convex_polygon};
use crate::render_gl;
use crate::render_gl::shape_drawing_component::ShapeDrawingComponent;
use crate::texture::Texture;
use crate::vertex::VertexShaderDataConfigurer;
use crate::glam_utils::to_glam_vec;
use crate::renderer::RenderUtil;
use crate::api::drawable::Drawable;

// todo: reduce duplication https://users.rust-lang.org/t/how-to-implement-inheritance-like-feature-for-rust/31159
pub struct Quadrangle<T> where T: VertexShaderDataConfigurer {
    drawing_component: ShapeDrawingComponent<T>,
    vertices: [T; 4],
    indices: [i32; 6],
}


const REFERENCE_INDEX: usize = 2;

impl<'a, T: VertexShaderDataConfigurer> Quadrangle<T> {
    pub fn new(vertices: [T; 4],
               indices: [i32; 6],
               program: Rc<render_gl::Program>,
               texture: Option<Rc<Texture>>) -> Quadrangle<T> {
        let drawing_component = ShapeDrawingComponent::new(
            &vertices,
            &indices,
            texture,
            program,
        );
        Quadrangle {
            drawing_component,
            vertices,
            indices,
        }
    }

    // some algebra lib?
    // opengl coords :(
    pub fn move_by(&mut self, x: f32, y: f32, z: f32) {
        for vertex in self.vertices.iter_mut() {
            vertex.transpose_deprecated(x, y, z);
        }
        self.drawing_component.bind_data(&self.vertices)
    }

    // moves first vertex
    // does not support rotation
    pub fn move_to(&mut self, final_pos: &(f32, f32, f32)) {
        let current_pos = self.vertices[REFERENCE_INDEX].get_pos_deprecated();
        self.move_by(
            final_pos.0 - current_pos.0,
            final_pos.1 - current_pos.1,
            final_pos.2 - current_pos.2,
        );
    }
}

impl<T: VertexShaderDataConfigurer> Drawable for Quadrangle<T> {
    fn render(&self,  render_util: &RenderUtil) {
        self.drawing_component.render(self.indices.len() as i32, gl::TRIANGLES, to_glam_vec(&self.get_pos()), render_util)
    }
}

impl<T: VertexShaderDataConfigurer> Area for Quadrangle<T> {
    fn contains_point(&self, point: &(f32, f32)) -> bool {
        return is_point_within_convex_polygon(point,
                                              &self.vertices.iter()
                                                  .map(|v| -> (f32, f32){
                                                      let p = v.get_pos_deprecated();
                                                      (p.0, p.1)
                                                  })
                                                  .collect(),
        );
    }

    fn area(&self) -> f32 {
        return 1.0;
    }

    fn num_vertices(&self) -> usize {
        return self.vertices.len();
    }

    fn get_pos(&self) -> (f32, f32, f32) {
        self.vertices[REFERENCE_INDEX].get_pos_deprecated()
    }
}
