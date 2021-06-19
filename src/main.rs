use std::ffi::{CString};
use std::path::Path;
use crate::resources::Resources;
use render_gl::data;
use render_gl::buffer;

pub mod render_gl;
pub mod resources;

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    pos: data::f32_f32_f32,
    clr: data::f32_f32_f32,
}

impl Vertex {
    fn vertex_attrib_pointers() {
        let stride = std::mem::size_of::<Self>(); // byte offset between consecutive attributes

        let location = 0; // layout (location = 0)
        let offset = 0; // offset of the first component
        unsafe {
            data::f32_f32_f32::vertex_attrib_pointer(stride, location, offset);
        }

        let location = 1; // layout (location = 1)
        let offset = offset + std::mem::size_of::<data::f32_f32_f32>(); // offset of the first component
        unsafe {
            data::f32_f32_f32::vertex_attrib_pointer(stride, location, offset);
        }
    }
}

fn main() {

    let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();

    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let mut event_pump = sdl.event_pump().unwrap();

    let gl_context = window.gl_create_context().unwrap();
    let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let shader_program = render_gl::Program::from_res(&res, "shaders/triangle").unwrap();
    shader_program.set_used();

    let vertices: Vec<Vertex> = vec![
        Vertex { pos: (0.5, -0.5, 0.0).into(),  clr: (1.0, 0.0, 0.0).into() }, // bottom right
        Vertex { pos: (-0.5, -0.5, 0.0).into(), clr: (0.0, 1.0, 0.0).into() }, // bottom left
        Vertex { pos: (0.0,  0.5, 0.0).into(),  clr: (0.0, 0.0, 1.0).into() }  // top
    ];

    let vbo = buffer::ArrayBuffer::new();
    vbo.bind();
    vbo.static_draw_data(&vertices);
    vbo.unbind();

    let vao = render_gl::buffer::VertexArray::new();
    vao.bind();
    vbo.bind();
    Vertex::vertex_attrib_pointers();
    vbo.unbind();
    vao.unbind();

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }

            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            shader_program.set_used();
            vao.bind();
            unsafe {
                gl::DrawArrays(
                    gl::TRIANGLES, // mode
                    0,             // starting index in the enabled arrays
                    3,             // number of indices to be rendered
                );
            }


            window.gl_swap_window();
        }
    }
}

