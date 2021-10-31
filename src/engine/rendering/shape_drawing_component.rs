use std::marker::PhantomData;
use std::rc::Rc;

use crate::engine::api::maths::vertex::VertexShaderDataConfigurer;
use crate::engine::api::render_util::RenderUtil;
use crate::engine::api::texture::Texture;
use crate::engine::rendering;
use crate::engine::rendering::buffer::{ArrayBuffer, ElementArrayBuffer, VertexArray};
use crate::engine::rendering::buffer;
use crate::engine::rendering::material::{Material, UniformKind};

// todo: this class should be probably on engine side
pub struct ShapeDrawingComponent<T> where T: VertexShaderDataConfigurer {
    vbo: ArrayBuffer,
    vao: VertexArray,
    ebo: ElementArrayBuffer,
    texture: Option<Rc<Texture>>,
    _marker: PhantomData<T>,
}

impl<'a, T: VertexShaderDataConfigurer> ShapeDrawingComponent<T> {
    pub fn new(vertices: &[T], indices: &[i32],
               texture: Option<Rc<Texture>>) -> ShapeDrawingComponent<T> {
        let vbo = buffer::ArrayBuffer::new();
        let vao = rendering::buffer::VertexArray::new();
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
            _marker: ::std::marker::PhantomData,
        }
    }

    pub fn bind_data(&self, vertices: &[T]) {
        self.vbo.bind();
        self.vbo.bind_buffer_data(vertices);
        self.vbo.unbind();
    }


    pub fn render(
        &mut self,
        num_indices: i32,
        mode: gl::types::GLenum,
        world_coords_position: glam::Vec3,     //todo: world_coords_position are incorrect and not even used
        render_util: &RenderUtil,
        material: &mut Material
    ) {
        // todo wtf this position
        // it does not work!
        let mvp = render_util.calculate_camera_MVP(glam::vec3(0.0, 0.0, 0.0));

        // set shader uniforms
        material.set_variable("mvp", UniformKind::MAT_4 { value: mvp });
        material.set_variable("resolution", UniformKind::VEC_2 { value: render_util.get_window_size() });
        // care - u64 to f32
        material.set_variable("timeMillis", UniformKind::FLOAT {value: material.get_active_duration().as_millis() as f32});

        material.activate();

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
            gl::UseProgram(0);
        }
        self.vao.unbind();
        self.ebo.unbind();
    }
}