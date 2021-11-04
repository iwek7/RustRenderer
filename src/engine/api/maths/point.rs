use std::ops::{Add, Sub};

use crate::engine::api::drawable::Drawable;
use crate::engine::api::maths::shapes_common::Area;
use crate::engine::api::maths::vertex::VertexShaderDataLayout;
use crate::engine::api::render_util::RenderUtil;
use crate::engine::glam_utils::to_glam_vec;
use crate::engine::rendering::material::Material;
use crate::engine::rendering::shape_drawing_component::ShapeDrawingComponent;

/*
example:
```
Point::new(
            [ColoredVertexDataLayout { pos: (0.0, 0.0, 0.0).into(), clr: WHITE.into() }, ],
            material,
            glam::vec3(1.6843166, -9.1099205, 0.0),
        )
        ```
 */
pub struct Point<T> where T: VertexShaderDataLayout {
    drawing_component: ShapeDrawingComponent<T>,
    vertices: [T; 1],
    material: Material,
    world_position: glam::Vec3,
}

impl<T: VertexShaderDataLayout> Point<T> {
    pub fn new(vertices: [T; 1], material: Material, world_position: glam::Vec3) -> Point<T> {
        let drawing_component = ShapeDrawingComponent::new(
            &vertices,
            &[0],
            None,
        );
        return Point {
            drawing_component,
            vertices,
            material,
            world_position,
        };
    }

    // this code is duplicated
    pub fn move_by(&mut self, offset: &glam::Vec3) {
        self.world_position = self.world_position.add(*offset)
    }

    pub fn move_to(&mut self, final_pos: &glam::Vec3) {
        self.world_position = final_pos.clone()
    }
}

impl<T: VertexShaderDataLayout> Drawable for Point<T> {
    fn render(&mut self, render_util: &RenderUtil) {
        self.drawing_component.render(
            1,
            gl::POINTS,
            to_glam_vec(&self.get_pos()),
            render_util,
            &mut self.material,
            glam::vec3(1.0, 1.0, 1.0)
        )
    }
}

impl<T: VertexShaderDataLayout> Area for Point<T> {
    // todo this needs 3d
    fn contains_point(&self, point: &(f32, f32)) -> bool {
        false
    }

    fn area(&self) -> f32 {
        0.0
    }

    fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    fn get_pos(&self) -> (f32, f32, f32) {
        self.world_position.into()
    }
}