use std::rc::Rc;
use crate::engine::api::maths::shapes_common::{Area, is_point_within_convex_polygon};
use crate::engine::render_gl;
use crate::engine::render_gl::shape_drawing_component::ShapeDrawingComponent;
use crate::engine::api::texture::Texture;
use crate::vertex::VertexShaderDataConfigurer;
use crate::glam_utils::to_glam_vec;
use crate::renderer::RenderUtil;
use crate::engine::api::drawable::Drawable;

pub struct Triangle<T: VertexShaderDataConfigurer> {
    open_gl_context: ShapeDrawingComponent<T>,
    vertices: [T; 3],
    indices: [i32; 3],
}

// todo: pass reference of texture here
impl<'a, T: VertexShaderDataConfigurer> Triangle<T> {
    pub fn new(vertices: [T; 3], indices: [i32; 3], program:Rc<render_gl::ShaderProgram>, texture: Option<Rc<Texture>>) -> Triangle<T> {
        let open_gl_context = ShapeDrawingComponent::new(
            &vertices,
            &indices,
            texture,
            program,
        );

        Triangle {
            open_gl_context,
            vertices,
            indices
        }
    }

    // some algebra lib?
    pub fn move_by(&mut self, x: f32, y: f32, z: f32) {
        for vertex in self.vertices.iter_mut() {
            vertex.transpose_deprecated(x, y, z);
        }
        self.open_gl_context.bind_data(&self.vertices)
    }
}

impl<'a, T: VertexShaderDataConfigurer> Drawable for Triangle<T> {
    fn render(&self,  render_util: &RenderUtil) {
        self.open_gl_context.render(self.indices.len() as i32, gl::TRIANGLES, to_glam_vec(&self.get_pos()), render_util)
    }
}

impl<'a, T: VertexShaderDataConfigurer> Area for Triangle<T> {
    fn contains_point(&self, point: &(f32, f32)) -> bool {
        return is_point_within_convex_polygon(point,
                                              &self.vertices.iter()
                                                  .map(|v| -> (f32, f32) {
                                                     let p = v.get_pos_deprecated();
                                                      (p.0, p.1)
                                                  })
                                                  .collect(), );
    }

    fn area(&self) -> f32 {
        todo!()
    }

    fn num_vertices(&self) -> usize {
        return self.vertices.len();
    }

    fn get_pos(&self) -> (f32, f32, f32) {
        todo!()
    }
}

