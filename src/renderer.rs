use crate::resources::Resources;
use crate::{render_gl, vertex};
use crate::render_gl::buffer;
use std::path::Path;

pub struct Renderer {
    sdl: sdl2::Sdl,
    window: sdl2::video::Window,
    event_pump: sdl2::EventPump,
    gl_context: sdl2::video::GLContext,

}

impl Renderer {
    pub fn new() -> Renderer {
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

        Renderer {
            sdl,
            window,
            event_pump,
            gl_context,
        }
    }

    pub fn render(&mut self) {
        unsafe {
            gl::Viewport(0, 0, 900, 700);
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        }

        let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();
        let shader_program = render_gl::Program::from_res(&res, "shaders/triangle").unwrap();
        shader_program.set_used();

        let vertices: Vec<vertex::Vertex> = vec![
            vertex::Vertex { pos: (0.5, -0.5, 0.0).into(), clr: (1.0, 0.0, 0.0).into() }, // bottom right
            vertex::Vertex { pos: (-0.5, -0.5, 0.0).into(), clr: (0.0, 1.0, 0.0).into() }, // bottom left
            vertex::Vertex { pos: (0.0, 0.5, 0.0).into(), clr: (0.0, 0.0, 1.0).into() }  // top
        ];

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

        'main: loop {
            for event in self.event_pump.poll_iter() {
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
                self.window.gl_swap_window();
            }
        }
    }
}