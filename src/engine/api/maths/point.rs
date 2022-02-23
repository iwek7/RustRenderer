use std::ops::{Add, Sub};

use glam::Vec3;

use crate::engine::api::game_object::GameObject;
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
}

impl<T: VertexShaderDataLayout> GameObject for Point<T> {
    fn render(&mut self, render_util: &RenderUtil) {
        self.drawing_component.render(
            1,
            gl::POINTS,
            *self.get_pos(),
            render_util,
            &mut self.material,
            glam::vec3(1.0, 1.0, 1.0),
            glam::vec3(0.0, 0.0, 0.0),

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
       todo!()
    }

    fn set_scale(&mut self, new_scale: Vec3) {
        todo!()
    }
}