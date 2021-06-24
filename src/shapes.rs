use crate::{render_gl, vertex};
use crate::render_gl::buffer;
use crate::vertex::{VertexDataSetter};
use crate::render_gl::buffer::{ArrayBuffer, VertexArray};

pub struct Triangle<'a, T: VertexDataSetter>  {
    pub program: &'a render_gl::Program,
    pub vbo: ArrayBuffer,
    pub vao: VertexArray,
    vertices: Vec<T>
}

impl<'a, T: VertexDataSetter> Triangle<'a, T> {
    pub fn new(v1: T, v2: T, v3: T, program: &render_gl::Program) -> Triangle<T> {
        let vertices = vec![v1, v2, v3];

        let vbo = buffer::ArrayBuffer::new();
        vbo.bind();
        vbo.static_draw_data(&vertices);
        vbo.unbind();

        let vao = render_gl::buffer::VertexArray::new();
        vao.bind();
        vbo.bind();
        vertex::VertexColored::set_vertex_shader_data();
        vbo.unbind();
        vao.unbind();

        Triangle {
            program,
            vbo,
            vao,
            vertices
        }
    }

    // some algebra lib?
    pub fn move_by(&mut self, x: f32, y: f32, z: f32) {
        for vertex in self.vertices.iter_mut() {
            vertex.transpose(x, y, z);
        }
        self.vbo.bind();
        self.vbo.static_draw_data(&self.vertices);
        self.vbo.unbind();
    }

}

impl<'a, T: VertexDataSetter> Drawable for Triangle<'a, T> {
    fn render(&self) {
        self.program.set_used();
        self.vao.bind();
        unsafe {
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index in the enabled arrays
                3,             // number of indices to be rendered
            );
        }
    }
}

// todo: reduce duplication https://users.rust-lang.org/t/how-to-implement-inheritance-like-feature-for-rust/31159
pub struct Quadrangle<'a, T> where T: VertexDataSetter {
    pub program: &'a render_gl::Program,
    pub vbo: ArrayBuffer,
    pub vao: VertexArray,
    vertices: Vec<T>
}

impl<'a, T: VertexDataSetter> Quadrangle<'a, T> {
    pub fn new(v1: T, v2: T, v3: T, v4: T, program: &render_gl::Program) -> Quadrangle<T> {
        let vertices = vec![v1, v2, v3, v4];

        let vbo = buffer::ArrayBuffer::new();
        vbo.bind();
        vbo.static_draw_data(&vertices);
        vbo.unbind();

        let vao = render_gl::buffer::VertexArray::new();
        vao.bind();
        vbo.bind();
        vertex::VertexColored::set_vertex_shader_data();
        vbo.unbind();
        vao.unbind();

        Quadrangle {
            program,
            vbo,
            vao,
            vertices
        }
    }

    // some algebra lib?
    pub fn move_by(&mut self, x: f32, y: f32, z: f32) {
        for vertex in self.vertices.iter_mut() {
            vertex.transpose(x, y, z);
        }
        self.vbo.bind();
        self.vbo.static_draw_data(&self.vertices);
        self.vbo.unbind();
    }

}

impl<'a, T: VertexDataSetter> Drawable for Quadrangle<'a, T> {
    fn render(&self) {
        self.program.set_used();
        self.vao.bind();
        unsafe {
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index in the enabled arrays
                4,             // number of indices to be rendered
            );
        }
    }
}

pub trait Drawable {
    fn render(&self);
}
