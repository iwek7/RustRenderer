use std::convert::TryInto;
use std::f32::consts::PI;
use std::ops::Add;
use crate::engine::api::colour::Colour;

use crate::engine::api::drawable::Drawable;
use crate::engine::api::maths::shapes_common::Area;
use crate::engine::api::maths::vertex::ColoredVertexDataLayout;
use crate::engine::api::render_util::RenderUtil;
use crate::engine::rendering::data::f32_f32_f32;
use crate::engine::rendering::material::Material;
use crate::engine::rendering::shape_drawing_component::ShapeDrawingComponent;

const NUM_VERTICES: i32 = 32;

// we have n vertices that form n - 1 triangles. -1  because vertex in the middle is shared
// therefore number indices is 3 * (n - 1)
pub struct Circle {
    drawing_component: ShapeDrawingComponent<ColoredVertexDataLayout>,
    vertices: [ColoredVertexDataLayout; 32],
    indices: [i32; 93],
    middle: glam::Vec3,
    radius: f32,
    material: Material,
    color: Colour
}

impl Circle {
    // position here is the middle of a circle
    pub fn new_colored(position: &glam::Vec3, color: Colour, radius: f32, material: Material) -> Circle {
        // todo: unhadrcode it, right now those values are hardcoded in vertics and indices types definitions
        let vertices = Circle::init_vertices(NUM_VERTICES, position, color, radius);
        let indices = Circle::init_indices(NUM_VERTICES);

        let drawing_component = ShapeDrawingComponent::new(
            &vertices,
            &indices,
            None,
        );

        Circle {
            drawing_component,
            vertices: vertices.try_into().unwrap(),
            indices: indices.try_into().unwrap(),
            middle: position.clone(),
            radius,
            material,
            color
        }
    }

    pub fn resize(&mut self, new_radius: f32) {
        let vertices = Circle::init_vertices(
            NUM_VERTICES,
            &self.middle,
            self.color,
            new_radius
        );
        self.vertices = vertices.try_into().unwrap();
        self.drawing_component.bind_data(&self.vertices);
    }

    fn init_vertices(num_vertices: i32, position: &glam::Vec3, color: Colour, radius: f32) -> Vec<ColoredVertexDataLayout> {
        let mut vertices = vec!();
        // todo: is it possible to use macro instead of this?
        let p: f32_f32_f32 = position.clone().into();
        vertices.push(
            ColoredVertexDataLayout {
                pos: p,
                clr: color.into(),
            }
        );

        for i in 0..num_vertices - 1 {
            let angle = 2.0 * PI * (i as f32) / (num_vertices - 1) as f32;
            vertices.push(
                ColoredVertexDataLayout {
                    pos: position.add(glam::vec3(
                        angle.cos() * radius,
                        angle.sin() * radius,
                        0.0)
                    ).into(),
                    clr: color.into(),
                }
            )
        }
        vertices
    }

    fn init_indices(num_vertices: i32) -> Vec<i32> {
        // todo this could be done in signle loop with initializing vertices
        // todo we need some kind of wraping iterator / no idea what does it mean?
        let mut indices = vec!();
        for i in 0..num_vertices - 2 {
            indices.push(0);
            indices.push(i + 1);
            indices.push(i + 2);
        }

        indices.push(0);

        indices.push(1);
        indices.push(num_vertices - 2 + 1);
        indices
    }
}

impl Drawable for Circle {
    fn render(&mut self, render_util: &RenderUtil) {
        self.drawing_component.render(
            self.indices.len() as i32,
            gl::TRIANGLES,
            self.middle.clone(),
            render_util,
            &mut self.material,
        )
    }
}

impl Area for Circle {
    fn contains_point(&self, point: &(f32, f32)) -> bool {
        (self.middle.x - point.0).powf(2.0) + (self.middle.y - point.1).powf(2.0) <= self.radius.powf(2.0)
    }

    fn area(&self) -> f32 {
        PI * self.radius * self.radius
    }

    fn num_vertices(&self) -> usize {
        panic!("Circle does not have any vertices!")
    }

    fn get_pos(&self) -> (f32, f32, f32) {
        (self.middle.x, self.middle.y, self.middle.z)
    }
}