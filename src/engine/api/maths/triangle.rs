use crate::engine::api::drawable::Drawable;
use crate::engine::api::maths::shapes_common::{Area, is_point_within_convex_polygon};
use crate::engine::api::render_util::RenderUtil;
use crate::engine::api::texture::{Sprite};
use crate::engine::glam_utils::to_glam_vec;
use crate::engine::rendering::material::Material;
use crate::engine::rendering::shape_drawing_component::ShapeDrawingComponent;
use crate::vertex::VertexShaderDataLayout;

pub struct Triangle<T: VertexShaderDataLayout> {
    open_gl_context: ShapeDrawingComponent<T>,
    vertices: [T; 3],
    indices: [i32; 3],
    material: Material,
}

// todo: pass reference of texture here
impl<'a, T: VertexShaderDataLayout> Triangle<T> {
    pub fn new(vertices: [T; 3], indices: [i32; 3], material: Material, sprite: Option<Sprite>) -> Triangle<T> {
        let open_gl_context = ShapeDrawingComponent::new(
            &vertices,
            &indices,
            sprite,
        );

        Triangle {
            open_gl_context,
            vertices,
            indices,
            material,
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

impl<'a, T: VertexShaderDataLayout> Drawable for Triangle<T> {
    fn render(&mut self, render_util: &RenderUtil) {
        self.open_gl_context.render(
            self.indices.len() as i32,
            gl::TRIANGLES,
            to_glam_vec(&self.get_pos()),
            render_util,
            &mut self.material)
    }
}

impl<'a, T: VertexShaderDataLayout> Area for Triangle<T> {
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

