use crate::{render_gl, vertex};
use crate::render_gl::buffer;
use crate::render_gl::buffer::{ArrayBuffer, ElementArrayBuffer, VertexArray};
use crate::vertex::VertexDataSetter;

pub struct Triangle<'a, T: VertexDataSetter> {
    pub program: &'a render_gl::Program,
    pub vbo: ArrayBuffer,
    pub vao: VertexArray,
    pub ebo: ElementArrayBuffer,
    vertices: Vec<T>,
    indices: [i32; 3],
}

impl<'a, T: VertexDataSetter> Triangle<'a, T> {
    pub fn new(v1: T, v2: T, v3: T, program: &render_gl::Program) -> Triangle<T> {
        let vertices = vec![v1, v2, v3];
        let indices = [0, 1, 2];

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

impl<'a, T: VertexDataSetter> Drawable for Triangle<'a, T> {
    fn render(&self) {
        self.program.set_used();
        self.vao.bind();
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices.len() as i32,
                gl::UNSIGNED_INT,
                0 as *const gl::types::GLvoid
            );

        }
        self.vao.unbind();
    }
}

// todo: reduce duplication https://users.rust-lang.org/t/how-to-implement-inheritance-like-feature-for-rust/31159
pub struct Quadrangle<'a, T> where T: VertexDataSetter {
    pub program: &'a render_gl::Program,
    pub vbo: ArrayBuffer,
    pub vao: VertexArray,
    pub ebo: ElementArrayBuffer,
    vertices: Vec<T>,
    indices: [i32; 6],
}

impl<'a, T: VertexDataSetter> Quadrangle<'a, T> {
    pub fn new(v1: T, v2: T, v3: T, v4: T, program: &render_gl::Program) -> Quadrangle<T> {
        let vertices = vec![v1, v2, v3, v4];
        // todo indices should be function param
        let indices =  [0, 1, 3, 1, 2, 3];

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
        Quadrangle {
            program,
            vbo,
            vao,
            ebo,
            vertices,
            indices,
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

impl<'a, T: VertexDataSetter> Drawable for Quadrangle<'a, T> {
    fn render(&self) {
        self.program.set_used();
        self.vao.bind();
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices.len() as i32,
                gl::UNSIGNED_INT,
                0 as *const gl::types::GLvoid
            );
        }
        self.vao.unbind();
    }
}

pub trait Drawable {
    fn render(&self);
}
