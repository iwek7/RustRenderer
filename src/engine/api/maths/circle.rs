use std::convert::TryInto;
use std::f32::consts::PI;
use std::ops::Add;
use std::rc::Rc;

use crate::engine::api::drawable::Drawable;
use crate::engine::api::maths::shapes_common::Area;
use crate::engine::api::maths::vertex::{ColoredVertexData};
use crate::engine::render_gl::data::f32_f32_f32;
use crate::engine::render_gl::ShaderProgram;
use crate::engine::render_gl::shape_drawing_component::ShapeDrawingComponent;
use crate::engine::api::render_util::RenderUtil;

// we have n vertices that form n - 1 triangles. -1  because vertex in the middle is shared
// therefore number indices is 3 * (n - 1)
pub struct Circle {
    drawing_component: ShapeDrawingComponent<ColoredVertexData>,
    vertices: [ColoredVertexData; 32],
    indices: [i32; 93],
    middle: glam::Vec3,
    radius: f32
}

impl Circle {
    // position here is the middle of a circle
    pub fn new_colored(position: &glam::Vec3, color: glam::Vec4, radius: f32, program: Rc<ShaderProgram>) -> Circle {
        let num_vertices = 32;

        let mut vertices = vec!();
        let mut indices = vec!();

        // todo: is it possible to use macro instead of this?

        let p: f32_f32_f32 = position.clone().into();
        vertices.push(
            ColoredVertexData {
                pos: p,
                clr: color.into(),
            }
        );

        for i in 0..num_vertices - 1 {
            let angle = 2.0 * PI * (i as f32) / (num_vertices - 1) as f32;
            vertices.push(
                ColoredVertexData {
                    pos: position.add(glam::vec3(
                        angle.cos() * radius,
                        angle.sin() * radius,
                        0.0)
                    ).into(),
                    clr: color.into(),
                }
            )
        }

        // todo move to single loop
        // todo we need some kind of wraping iterator
        for i in 0..num_vertices - 2 {
            indices.push(0);
            indices.push(i + 1);
            indices.push(i + 2);
        }

        indices.push(0);

        indices.push(1);
        indices.push(num_vertices - 2 + 1);


        let drawing_component = ShapeDrawingComponent::new(
            &vertices,
            &indices,
            None,
            program,
        );

        Circle {
            drawing_component,
            vertices: vertices.try_into().unwrap(),
            indices: indices.try_into().unwrap(),
            middle: position.clone(),
            radius
        }
    }
}

impl Drawable for Circle {
    fn render(&self, render_util: &RenderUtil) {
        self.drawing_component.render(self.indices.len() as i32, gl::TRIANGLES, self.middle.clone(), render_util)
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