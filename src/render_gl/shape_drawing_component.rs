use std::marker::PhantomData;

use crate::maths::vertex::VertexShaderDataConfigurer;
use crate::render_gl;
use crate::render_gl::buffer::{ArrayBuffer, ElementArrayBuffer, VertexArray};
use crate::render_gl::buffer;
use crate::texture::Texture;
use crate::renderer::RenderUtil;

// todo: this class should be probably on engine side
pub struct ShapeDrawingComponent<'a, T> where T: VertexShaderDataConfigurer {
    vbo: ArrayBuffer,
    vao: VertexArray,
    ebo: ElementArrayBuffer,
    texture: Option<&'a Texture>,
    program: &'a render_gl::Program,
    _marker: PhantomData<T>,
}

impl<'a, T: VertexShaderDataConfigurer> ShapeDrawingComponent<'a, T> {
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
        T::configure_vertex_shader_data();

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

    //todo: world_coords_position are incorrect
    pub fn render(&self, num_indices: i32, mode: gl::types::GLenum, world_coords_position: glam::Vec3, render_util: &RenderUtil) {
        self.program.set_used();

        let mvp = render_util.calculate_camera_MVP(glam::vec3(0.5, 1.0, -1.0));
        self.program.set_mat4("mvp", mvp);

        self.program.set_vec2("win_size", render_util.get_window_size());

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