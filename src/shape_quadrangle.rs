use crate::{Draggable, render_gl};
use crate::render_gl::buffer::{ArrayBuffer, ElementArrayBuffer, VertexArray};
use crate::render_gl::buffer;
use crate::shape_triangle::{Area, Drawable, is_point_within_convex_polygon};
use crate::texture::Texture;
use crate::vertex::VertexShaderDataSetter;

// todo: reduce duplication https://users.rust-lang.org/t/how-to-implement-inheritance-like-feature-for-rust/31159
pub struct Quadrangle<'a, T> where T: VertexShaderDataSetter {
    pub program: &'a render_gl::Program,
    pub vbo: ArrayBuffer,
    pub vao: VertexArray,
    pub ebo: ElementArrayBuffer,
    vertices: [T; 4],
    indices: [i32; 6],
    texture: Option<Texture>,
    is_dragged: bool, // todo: it should not be here
}

impl<'a, T: VertexShaderDataSetter> Quadrangle<'a, T> {
    pub fn new(vertices: [T; 4], indices: [i32; 6], program: &render_gl::Program, texture: Option<Texture>) -> Quadrangle<T> {
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
            texture,
            is_dragged: false,
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

impl<'a, T: VertexShaderDataSetter> Drawable for Quadrangle<'a, T> {
    fn render(&self) {
        self.program.set_used();
        self.vao.bind();
        self.ebo.bind();
        unsafe {
            if self.texture.is_some() {
                self.texture.as_ref().unwrap().bind();
            }
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices.len() as i32,
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

impl<'a, T: VertexShaderDataSetter> Area for Quadrangle<'a, T> {
    fn contains_point(&self, point: &(f32, f32)) -> bool {
        return is_point_within_convex_polygon(point,
                                              &self.vertices.iter()
                                                  .map(|v| -> (f32, f32){ v.get_pos() })
                                                  .collect(), )
        ;
    }

    fn area(&self) -> f32 {
        return 1.0;
    }

    fn num_vertices(&self) -> usize {
        return self.vertices.len();
    }
}

impl<'a, T: VertexShaderDataSetter> Draggable for Quadrangle<'a, T> {
    fn is_mouse_over(&self, mouse_pos: &(f32, f32)) -> bool {
        self.contains_point(mouse_pos)
    }

    fn handle_start_drag(&mut self) {
        self.is_dragged = true
    }

    fn handle_drop(&mut self) {
        self.is_dragged = true
    }

    fn handle_drag_pointer_move(&mut self, offset: &(f32, f32)) {
        if self.is_dragged {
            self.move_by(offset.0, offset.1, 0.0)
        }
    }
}