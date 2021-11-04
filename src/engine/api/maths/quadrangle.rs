use crate::engine::api::drawable::Drawable;
use crate::engine::api::maths::shapes_common::{Area, is_point_within_convex_polygon};
use crate::engine::api::render_util::RenderUtil;
use crate::engine::api::texture::Sprite;
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
    world_position: glam::Vec3,
    scale: glam::Vec3,
}

const REFERENCE_INDEX: usize = 2;

impl<T: VertexShaderDataLayout> Quadrangle<T> {
    pub fn new(vertices: [T; 4],
               indices: [i32; 6],
               material: Material,
               sprite: Option<Sprite>,
               world_position: glam::Vec3,
    ) -> Quadrangle<T> {
        let drawing_component = ShapeDrawingComponent::new(
            &vertices,
            &indices,
            sprite,
        );
        Quadrangle {
            drawing_component,
            vertices,
            indices,
            material,
            world_position,
            scale: glam::vec3(1.0, 1.0, 1.0),
        }
    }

    // maybe they should be part of area
    pub fn move_by(&mut self, x: f32, y: f32, z: f32) {
        self.world_position = glam::vec3(self.world_position.x + x, self.world_position.y + y, self.world_position.z + z)
    }

    pub fn move_to(&mut self, final_pos: &(f32, f32, f32)) {
        self.world_position = glam::vec3(final_pos.0, final_pos.1, final_pos.2)
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
            &mut self.material,
            self.scale
        )
    }
}

impl<T: VertexShaderDataLayout> Area for Quadrangle<T> {
    fn contains_point(&self, point: &(f32, f32)) -> bool {
        return is_point_within_convex_polygon(point,
                                              &self.vertices.iter()
                                                  .map(|v| -> (f32, f32){
                                                      let p = v.get_pos_deprecated();
                                                      (p.0 + self.world_position.x, p.1 + self.world_position.y)
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
        self.world_position.into()
    }
}
