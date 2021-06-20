use crate::resources::Resources;
use crate::render_gl;
use crate::render_gl::buffer;

pub struct Triangle {
    program: render_gl::Program,
    _vbo: buffer::ArrayBuffer, // _ to disable warning about not used vbo
    vao: buffer::VertexArray,
}
