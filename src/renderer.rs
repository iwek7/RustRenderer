use std::path::Path;

use crate::{render_gl, vertex};
use crate::render_gl::buffer;
use crate::resources::Resources;
use crate::triangle::{Triangle, ObjectRender};
use crate::opengl_context::OpenglContext;

pub struct Renderer<'a> {
    context: OpenglContext,
    viewport: render_gl::Viewport,
    event_pump: sdl2::EventPump,
    objects: Vec<Box<dyn ObjectRender + 'a>>
}

impl<'a> Renderer<'a> {
    pub fn new(context: OpenglContext) -> Renderer<'a> {
        let mut viewport = render_gl::Viewport::for_window(900, 700);
        let mut event_pump = context.sdl.event_pump().unwrap();
        Renderer {
            context,
            viewport,
            event_pump,
            objects: vec![]
        }
    }

    pub fn render(&mut self) {
        self.viewport.set_used();
        unsafe {
            gl::Viewport(0, 0, 900, 700);
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        }

        'main: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => break 'main,
                    sdl2::event::Event::Window {
                        win_event: sdl2::event::WindowEvent::Resized(w, h),
                        ..
                    } => {
                        self.viewport.update_size(w, h);
                        self.viewport.set_used();
                    }
                    _ => {}
                }

                unsafe {
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }

                for obj_render in self.objects.iter() {
                   obj_render.render();
                }
                self.context.window.gl_swap_window();
            }
        }
    }

    pub fn add_object_render(&mut self, object_render: Box<dyn ObjectRender + 'a>) {
        self.objects.push(object_render);
    }


}