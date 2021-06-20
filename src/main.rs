use std::ffi::CString;
use std::path::Path;

use render_gl::buffer;
use render_gl::data;

use crate::resources::Resources;
use crate::triangle::Triangle;
use crate::opengl_context::OpenglContext;

pub mod render_gl;
pub mod resources;
pub mod renderer;
pub mod vertex;
pub mod triangle;
pub mod opengl_context;

fn main() {

    let context = OpenglContext::init();

    let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();
    let shader_program = render_gl::Program::from_res(&res, "shaders/triangle").unwrap();
    shader_program.set_used();

    let triangle = Triangle::new(
        vertex::Vertex { pos: (0.5, -0.5, 0.0).into(), clr: (1.0, 0.0, 0.0).into() }, // bottom right
        vertex::Vertex { pos: (-0.5, -0.5, 0.0).into(), clr: (0.0, 1.0, 0.0).into() }, // bottom left
        vertex::Vertex { pos: (0.0, 0.5, 0.0).into(), clr: (0.0, 0.0, 1.0).into() },
        &shader_program);

    let triangle2 = Triangle::new(
        vertex::Vertex { pos: (-1.0, -0.9, 0.0).into(), clr: (1.0, 0.0, 0.0).into() }, // bottom right
        vertex::Vertex { pos: (-0.7, -0.9, 0.0).into(), clr: (0.0, 1.0, 0.0).into() }, // bottom left
        vertex::Vertex { pos: (-0.85, -0.5, 0.0).into(), clr: (0.0, 0.0, 1.0).into() },
        &shader_program);

    let mut renderer = renderer::Renderer::new(context);
    renderer.add_object_render(Box::new(triangle));
    renderer.add_object_render(Box::new(triangle2));

    renderer.render();

}

