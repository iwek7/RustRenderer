use crate::{render_gl, vertex};
use crate::render_gl::buffer;
use crate::resources::Resources;
use crate::vertex::Vertex;
use crate::render_gl::buffer::{ArrayBuffer, VertexArray};

pub struct Triangle<'a> {
    pub program: &'a render_gl::Program,
    pub vbo: ArrayBuffer,
    pub vao: VertexArray,
    vertices: Vec<Vertex>

}

impl<'a> Triangle<'a> {
    pub fn new(v1: Vertex, v2: Vertex, v3: Vertex, program: &render_gl::Program) -> Triangle {
        let vertices = vec![v1, v2, v3];

        let vbo = buffer::ArrayBuffer::new();
        vbo.bind();
        vbo.static_draw_data(&vertices);
        vbo.unbind();

        let vao = render_gl::buffer::VertexArray::new();
        vao.bind();
        vbo.bind();
        vertex::Vertex::vertex_attrib_pointers();
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

impl<'a> ObjectRender for Triangle<'a> {
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

pub trait ObjectRender {
    fn render(&self);
}
