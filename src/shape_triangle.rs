use crate::{render_gl, Draggable};
use crate::render_gl::buffer;
use crate::render_gl::buffer::{ArrayBuffer, ElementArrayBuffer, VertexArray};
use crate::shape_triangle::Side::{LEFT, NONE, RIGHT};
use crate::texture::Texture;
use crate::vertex::VertexShaderDataSetter;

pub struct Triangle<'a, T: VertexShaderDataSetter> {
    pub program: &'a render_gl::Program,
    pub vbo: ArrayBuffer,
    pub vao: VertexArray,
    pub ebo: ElementArrayBuffer,
    vertices: [T; 3],
    indices: [i32; 3],
    texture: Option<Texture>,
    is_dragged: bool // todo it should not be here ...
}

impl<'a, T: VertexShaderDataSetter> Triangle<'a, T> {
    pub fn new(vertices: [T; 3], indices: [i32; 3], program: &render_gl::Program, texture: Option<Texture>) -> Triangle<T> {
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
        vbo.unbind(); // vao must be unbounded before ebo else ebo does not get saved!
        vao.unbind();
        ebo.unbind();


        Triangle {
            program,
            vbo,
            vao,
            ebo,
            vertices,
            indices,
            texture,
            is_dragged: false
        }
    }

    // some algebra lib?
    pub fn move_by(&mut self, x: f32, y: f32, z: f32) {
        for vertex in self.vertices.iter_mut() {
            vertex.transpose(x, y, z);
        }
        self.vbo.bind();
        self.vbo.bind_buffer_data(&self.vertices);
        self.vbo.unbind();
    }
}

impl<'a, T: VertexShaderDataSetter> Drawable for Triangle<'a, T> {
    fn render(&self) {
        self.program.set_used();
        self.vao.bind();
        self.ebo.bind(); // is this needed? as per https://stackoverflow.com/questions/24876647/understanding-glvertexattribpointer yes
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices.len() as i32,
                gl::UNSIGNED_INT,
                0 as *const gl::types::GLvoid,
            );
        }
        self.vao.unbind();
        self.ebo.unbind();
    }
}

impl<'a, T: VertexShaderDataSetter + Clone> Area for Triangle<'a, T> {
    fn contains_point(&self, point: &(f32, f32)) -> bool {
        return is_point_within_convex_polygon(point,
                                              &self.vertices.iter()
                                                  .map(|v| -> (f32, f32){ v.get_pos() })
                                                  .collect(), );
    }

    fn area(&self) -> f32 {
        todo!()
    }

    fn num_vertices(&self) -> usize {
        return self.vertices.len();
    }
}

impl<'a, T: VertexShaderDataSetter + Clone> Draggable for Triangle<'a, T> {
    fn is_mouse_over(&self, mouse_pos: &(f32, f32)) -> bool {
        self.contains_point(mouse_pos)
    }

    fn handle_start_drag(&mut self) {
        // nothing
    }

    fn handle_drop(&mut self) {
        // nothing
    }

    fn handle_drag_pointer_move(&mut self, offset: &(f32, f32)) {
        self.move_by(offset.0, offset.1, 0.0)
    }
}


// todo: to separate file
pub trait Drawable {
    fn render(&self);
}

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


