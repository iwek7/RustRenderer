use std::path::Path;

use crate::{render_gl, vertex};
use crate::opengl_context::OpenglContext;
use crate::render_gl::buffer;
use crate::resources::Resources;
use crate::triangle::{ObjectRender, Triangle};

pub struct Renderer<'a> {
    context: OpenglContext,
    viewport: render_gl::Viewport,
    objects: Vec<Box<dyn ObjectRender + 'a>>,
}

impl<'a> Renderer<'a> {
    pub fn new(context: OpenglContext) -> Renderer<'a> {
        let mut viewport = render_gl::Viewport::for_window(900, 700);
        viewport.set_used();
        unsafe {
            gl::Viewport(0, 0, 900, 700);
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        }

        Renderer {
            context,
            viewport,
            objects: vec![],
        }
    }

    pub fn render(&mut self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        for obj_render in self.objects.iter() {
            obj_render.render();
        }
        self.context.window.gl_swap_window();
    }


    pub fn add_object_render(&mut self, object_render: Box<dyn ObjectRender + 'a>) {
        self.objects.push(object_render);
    }

    pub fn resize_viewport(&mut self, w: i32, h: i32) {
        self.viewport.update_size(w, h);
        self.viewport.set_used();
    }

}