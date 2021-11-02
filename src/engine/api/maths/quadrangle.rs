use std::rc::Rc;

use crate::engine::api::drawable::Drawable;
use crate::engine::api::maths::shapes_common::{Area, is_point_within_convex_polygon};
use crate::engine::api::render_util::RenderUtil;
use crate::engine::api::texture::Texture;
use crate::engine::glam_utils::to_glam_vec;
use crate::engine::rendering::material::{Material, UniformKind};
use crate::engine::rendering::shape_drawing_component::ShapeDrawingComponent;
use crate::vertex::VertexShaderDataLayout;

// todo: reduce duplication https://users.rust-lang.org/t/how-to-implement-inheritance-like-feature-for-rust/31159
pub struct Quadrangle<T> where T: VertexShaderDataLayout {
    drawing_component: ShapeDrawingComponent<T>,
    vertices: [T; 4],
    indices: [i32; 6],
    material: Material,
}

const REFERENCE_INDEX: usize = 2;

impl<T: VertexShaderDataLayout> Quadrangle<T> {
    pub fn new(vertices: [T; 4],
               indices: [i32; 6],
               material: Material,
               texture: Option<Rc<Texture>>) -> Quadrangle<T> {
        let drawing_component = ShapeDrawingComponent::new(
            &vertices,
            &indices,
            texture,
        );
        Quadrangle {
            drawing_component,
            vertices,
            indices,
            material,
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

    pub fn set_material_variable(&mut self, name: &str, kind: UniformKind) {
        self.material.set_variable(name, kind);
    }
}

impl<T: VertexShaderDataLayout> Drawable for Quadrangle<T> {
    fn render(&mut self, render_util: &RenderUtil) {
        self.drawing_component.render(
            self.indices.len() as i32,
            gl::TRIANGLES,
            to_glam_vec(&self.get_pos()),
            render_util,
            &mut self.material)
    }
}

impl<T: VertexShaderDataLayout> Area for Quadrangle<T> {
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