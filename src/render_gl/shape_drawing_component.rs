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

    pub fn render(&self, num_indices: i32, mode: gl::types::GLenum, world_coords_position: glam::Vec3) {
        self.program.set_used();

        // todo: move this away from here, camera should do this
        // let projection = glam::Mat4::orthographic_rh_gl(
        //     -900.0 / 2.0,
        //     // todo get window sizes from opengl context
        //     900.0 / 2.0,
        //     700.0 / 2.0,
        //     -700.0 / 2.0,
        //     0.1,
        //     100.0,
        // );
        let projection = glam::Mat4::perspective_rh_gl(45.0, 3.0 / 3.0, 0.1, 100.0);
        self.program.set_mat4("projection", projection);

        let mut view = glam::Mat4::look_at_rh(
            glam::vec3(0.0, 0.0, 2.0), // todo is this correct?
            glam::vec3(0.0, 0.0, 0.0),
            glam::vec3(0.0, 1.0, 0.0),
        );
        self.program.set_mat4("view", view);

        // let mut model = glam::Mat4::IDENTITY.mul_vec4(glam::vec4(0.0,0.0,-1.0, 1.0));

        // let mut model = glam::Mat4::from_translation(glam::vec3(1.0,1.0,3.0));
        self.program.set_mat4("model",  glam::Mat4::IDENTITY);
        // todo:  end of camera specific stuff to be moved away

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