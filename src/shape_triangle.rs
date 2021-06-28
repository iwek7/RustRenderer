use crate::{render_gl, vertex};
use crate::render_gl::buffer;
use crate::render_gl::buffer::{ArrayBuffer, ElementArrayBuffer, VertexArray};
use crate::texture::Texture;
use crate::vertex::VertexDataSetter;

pub struct Triangle<'a, T: VertexDataSetter> {
    pub program: &'a render_gl::Program,
    pub vbo: ArrayBuffer,
    pub vao: VertexArray,
    pub ebo: ElementArrayBuffer,
    vertices: [T; 3],
    indices: [i32; 3],
    texture: Option<Texture>,
}

impl<'a, T: VertexDataSetter> Triangle<'a, T> {
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
            texture
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


pub trait Drawable {
    fn render(&self);
}
