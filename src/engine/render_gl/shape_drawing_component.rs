use std::marker::PhantomData;
use std::rc::Rc;

use crate::engine::api::maths::vertex::VertexShaderDataConfigurer;
use crate::engine::api::render_util::RenderUtil;
use crate::engine::api::texture::Texture;
use crate::engine::render_gl;
use crate::engine::render_gl::buffer::{ArrayBuffer, ElementArrayBuffer, VertexArray};
use crate::engine::render_gl::buffer;

// todo: this class should be probably on engine side
pub struct ShapeDrawingComponent<T> where T: VertexShaderDataConfigurer {
    vbo: ArrayBuffer,
    vao: VertexArray,
    ebo: ElementArrayBuffer,
    texture: Option<Rc<Texture>>,
    program: Rc<render_gl::ShaderProgram>,
    _marker: PhantomData<T>,
}

impl<'a, T: VertexShaderDataConfigurer> ShapeDrawingComponent<T> {
    pub fn new(vertices: &[T], indices: &[i32],
               texture: Option<Rc<Texture>>, program: Rc<render_gl::ShaderProgram>) -> ShapeDrawingComponent<T> {
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

        // todo wtf this position
        // it does not work!
        let mvp = render_util.calculate_camera_MVP(glam::vec3(0.0, 0.0, 0.0));
        self.program.set_mat4("mvp", mvp);

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