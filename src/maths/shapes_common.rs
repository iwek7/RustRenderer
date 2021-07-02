use std::marker::PhantomData;

use crate::maths::shapes_common::Side::{LEFT, NONE, RIGHT};
use crate::maths::vertex::VertexShaderDataSetter;
use crate::render_gl;
use crate::render_gl::buffer::{ArrayBuffer, ElementArrayBuffer, VertexArray};
use crate::render_gl::buffer;
use crate::texture::Texture;

pub trait Area {
    fn contains_point(&self, point: &(f32, f32)) -> bool;
    fn area(&self) -> f32;
    fn num_vertices(&self) -> usize;
}

pub fn is_point_within_convex_polygon(point: &(f32, f32), vertices: &Vec<(f32, f32)>) -> bool {
    let mut previous_side: Side = Side::NONE;
    let num_vertices = vertices.len();

    for idx in 0..num_vertices {
        let v1: &(f32, f32) = &vertices[idx];
        let v2: &(f32, f32) = if idx == num_vertices - 1 {
            &vertices[0]
        } else {
            &vertices[idx + 1]
        };
        // todo: why are those called affine
        let affine_segment = subtract_vectors(v2, v1);
        let affine_point = subtract_vectors(point, v1);
        let current_side = Side::calculate_side(&affine_segment, &affine_point);
        if !previous_side.is_on_the_same_side_as(&current_side) {
            return false;
        }
        previous_side = current_side;
    }
    return true;

    // this really should be on some vector object
    fn subtract_vectors(v1: &(f32, f32), v2: &(f32, f32)) -> (f32, f32) {
        return (v1.0 - v2.0, v1.1 - v2.1);
    }
}

// todo make enum ON_EDGE for case 0
#[derive(PartialEq, Debug)] // for comparisons
enum Side {
    NONE,
    RIGHT,
    LEFT,
}

impl Side {
    fn calculate_side(affine_segment: &(f32, f32), affine_point: &(f32, f32)) -> Side {
        let x = Side::cosine_sign(affine_segment, affine_point);
        return match x {
            _ if x < 0.0 => LEFT,
            _ if x > 0.0 => RIGHT,
            _ => NONE
        };
    }

    fn is_on_the_same_side_as(&self, other: &Side) -> bool {
        return match ((self == other) || (self == &NONE)) && (other != &NONE) {
            true => { true }
            false => { false }
        };
    }


    // wtf is this
    fn cosine_sign(v1: &(f32, f32), v2: &(f32, f32)) -> f32 {
        return v1.0 * v2.1 - v1.1 * v2.0;
    }
}

// todo: this should be moved away from maths package to opengl package
pub struct ShapeDrawingComponent<'a, T> where T: VertexShaderDataSetter {
    vbo: ArrayBuffer,
    vao: VertexArray,
    ebo: ElementArrayBuffer,
    texture: Option<Texture>,
    program: &'a render_gl::Program,
    _marker: PhantomData<T>,
}

impl<'a, T: VertexShaderDataSetter> ShapeDrawingComponent<'a, T> {
    pub fn new(vertices: &[T], indices: &[i32],
               texture: Option<Texture>, program: &'a render_gl::Program) -> ShapeDrawingComponent<'a, T> {
        let vbo = buffer::ArrayBuffer::new();
        let vao = render_gl::buffer::VertexArray::new();
        let ebo = buffer::ElementArrayBuffer::new();

        vao.bind();
        // bind buffer object and set pointer to data
        vbo.bind();
        vbo.bind_buffer_data(&vertices);

        // bind indices
        ebo.bind();
        ebo.bind_buffer_data(&indices);
        T::set_vertex_shader_data();

        // unbind everything
        vbo.unbind(); // vao must be unbind before ebo else ebo does not get saved!
        vao.unbind();
        ebo.unbind();
        ShapeDrawingComponent {
            vbo,
            vao,
            ebo,
            texture,
            program,
            _marker: ::std::marker::PhantomData,
        }
    }

    pub fn bind_data(&self, vertices: &[T]) {
        self.vbo.bind();
        self.vbo.bind_buffer_data(vertices);
        self.vbo.unbind();
    }

    pub fn render(&self, num_indices: i32, mode: gl::types::GLenum) {
        self.program.set_used();
        self.vao.bind();
        self.ebo.bind();
        unsafe {
            if self.texture.is_some() {
                self.texture.as_ref().unwrap().bind();
            }
            gl::DrawElements(
                mode,
                num_indices,
                gl::UNSIGNED_INT,
                0 as *const gl::types::GLvoid,
            );
            if self.texture.is_some() {
                self.texture.as_ref().unwrap().unbind();
            }
        }
        self.vao.unbind();
        self.ebo.unbind();
    }
}
