use std::ops::{Add, Sub};

use glam::Vec3;

use crate::engine::api::game_object::{BaseGameObject, GameObject};
use crate::engine::api::maths::shapes_common::{Area, is_point_within_convex_polygon};
use crate::engine::api::render_util::RenderUtil;
use crate::engine::api::texture::Sprite;
use crate::engine::rendering::material::{Material, UniformKind};
use crate::engine::rendering::shape_drawing_component::ShapeDrawingComponent;
use crate::engine::api::maths::vertex::VertexShaderDataLayout;

// todo: reduce duplication https://users.rust-lang.org/t/how-to-implement-inheritance-like-feature-for-rust/31159
pub struct Quadrangle<T> where T: VertexShaderDataLayout {
    base_game_object: BaseGameObject,
    drawing_component: ShapeDrawingComponent<T>,
    vertices: [T; 4],
    indices: [i32; 6],
    material: Material,
    world_position: glam::Vec3,
    scale: glam::Vec3,
    scaling_point: glam::Vec3
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

        let scaling_point = vertices[REFERENCE_INDEX].get_pos();
        Quadrangle {
            base_game_object: BaseGameObject::new(),
            drawing_component,
            vertices,
            indices,
            material,
            world_position,
            scale: glam::vec3(1.0, 1.0, 1.0),
            scaling_point
        }
    }

    pub fn set_material_variable(&mut self, name: &str, kind: UniformKind) {
        self.material.set_variable(name, kind);
    }

    pub fn set_scaling_point(&mut self, scaling_point: glam::Vec3) {
        self.scaling_point = scaling_point
    }
}

impl<T: VertexShaderDataLayout> GameObject for Quadrangle<T> {
    fn render(&mut self, render_util: &RenderUtil) {
        self.drawing_component.render(
            self.indices.len() as i32,
            gl::TRIANGLES,
            *self.get_pos(),
            render_util,
            &mut self.material,
            self.scale.clone(),
            self.vertices[REFERENCE_INDEX].get_pos().sub(self.scaling_point),
        )
    }

    fn base_game_object(&mut self) -> &mut BaseGameObject {
        &mut self.base_game_object
    }
}

impl<T: VertexShaderDataLayout> Area for Quadrangle<T> {
    // todo does not work with scale
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

    fn get_pos(&self) -> &glam::Vec3 {
        &self.world_position
    }

    fn move_to(&mut self, final_position: Vec3) {
        self.world_position = final_position
    }

    fn move_by(&mut self, offset: Vec3) {
        self.world_position = self.world_position.add(offset)
    }

    fn get_scale(&self) -> &Vec3 {
        &self.scale
    }

    fn set_scale(&mut self, new_scale: Vec3) {
        self.scale = new_scale
    }
}
