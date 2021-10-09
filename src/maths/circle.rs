use std::convert::TryInto;
use std::f32::consts::PI;
use std::ops::Add;
use std::rc::Rc;

use crate::maths::vertex::{ColoredVertexData, VertexShaderDataConfigurer};
use crate::render_gl::Program;
use crate::render_gl::shape_drawing_component::ShapeDrawingComponent;

pub struct Circle<T> where T: VertexShaderDataConfigurer {
    drawing_component: ShapeDrawingComponent<T>,
    vertices: [T; 300],
    indices: [i32; 900],
}

impl<T: VertexShaderDataConfigurer> Circle<T> {
    fn new_colored(position: glam::Vec3, color: glam::Vec4, radius: f32, program: Rc<Program>) -> Circle<ColoredVertexData> {
        let num_vertices = 300;

        let mut vertices = vec!();
        let mut indices = vec!();

        // todo: is it possible to use macro instead of this?
        for i in 0..num_vertices {
            let angle = 2.0 * PI * (i as f32) / num_vertices as f32;
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
        for i in 0..num_vertices {
            indices.push(0);
            indices.push(1);
            indices.push(2);
        }

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
        }
    }
}