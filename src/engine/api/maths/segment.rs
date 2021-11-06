use std::ops::Add;

use glam::Vec3;

use crate::engine::api::drawable::Drawable;
use crate::engine::api::maths::shapes_common::Area;
use crate::engine::api::maths::vertex::VertexShaderDataLayout;
use crate::engine::api::render_util::RenderUtil;
use crate::engine::glam_utils::to_glam_vec;
use crate::engine::rendering::material::Material;
use crate::engine::rendering::shape_drawing_component::ShapeDrawingComponent;

pub struct Segment<T> where T: VertexShaderDataLayout {
    drawing_component: ShapeDrawingComponent<T>,
    vertices: [T; 2],
    indices: [i32; 2],
    material: Material,
    world_position: glam::Vec3,
    scale: glam::Vec3,
}

/**
 usage example:
```
   Segment::new(
            [
                ColoredVertexDataLayout { pos: (-100.0, 0.0, 0.0).into(), clr: clr.into() },
                ColoredVertexDataLayout { pos: (100.0, 0.0, 0.0).into(), clr: clr.into() },
            ],
            [0, 1],
            material.clone(),
            glam::vec3(0.0, 0.0, 0.0),
        );
```
 */
impl<'a, T: VertexShaderDataLayout> Segment<T> {
    pub fn new(vertices: [T; 2], indices: [i32; 2], material: Material, world_position: glam::Vec3) -> Segment<T> {
        let drawing_component = ShapeDrawingComponent::new(
            &vertices,
            &indices,
            None,
        );

        Segment {
            drawing_component,
            vertices,
            indices,
            material,
            world_position,
            scale: glam::vec3(1.0, 1.0, 1.0),
        }
    }
}

impl<'a, T: VertexShaderDataLayout> Drawable for Segment<T> {
    fn render(&mut self, render_util: &RenderUtil) {
        self.drawing_component.render(
            self.indices.len() as i32,
            gl::LINES,
            *self.get_pos(),
            render_util,
            &mut self.material,
            self.scale.clone(),
            glam::vec3(0.0, 0.0, 0.0),
        )
    }
}

impl<T: VertexShaderDataLayout> Area for Segment<T> {
    fn contains_point(&self, point: &(f32, f32)) -> bool {
        todo!()
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
        &self.scale
    }

    fn set_scale(&mut self, new_scale: Vec3) {
        self.scale = new_scale
    }
}

