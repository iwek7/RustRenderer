use std::marker::PhantomData;

use crate::maths::vertex::VertexShaderDataSetter;
use crate::render_gl;
use crate::render_gl::buffer::{ArrayBuffer, ElementArrayBuffer, VertexArray};
use crate::render_gl::buffer;
use crate::texture::Texture;

// todo: this should be moved away from maths package to opengl package
pub struct ShapeDrawingComponent<'a, T> where T: VertexShaderDataSetter {
    vbo: ArrayBuffer,
    vao: VertexArray,
    ebo: ElementArrayBuffer,
    texture: Option<&'a Texture>,
    program: &'a render_gl::Program,
    _marker: PhantomData<T>,
}

impl<'a, T: VertexShaderDataSetter> ShapeDrawingComponent<'a, T> {
    pub fn new(vertices: &[T], indices: &[i32],
               texture: Option<&'a Texture>, program: &'a render_gl::Program) -> ShapeDrawingComponent<'a, T> {
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