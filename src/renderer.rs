use crate::maths::triangle::Drawable;
use crate::opengl_context::OpenglContext;
use crate::render_gl;

pub struct Renderer<'a> {
    context: &'a OpenglContext,
    viewport: render_gl::Viewport,
}

impl<'a> Renderer<'a> {
    pub fn new(context: &OpenglContext) -> Renderer {
        let viewport = render_gl::Viewport::for_window(900, 700);
        viewport.set_used();
        unsafe {
            gl::Viewport(0, 0, 900, 700);
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        }

        Renderer {
            context,
            viewport,
        }
    }

    pub fn render(&mut self, objects: &[&dyn Drawable]) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        for obj_render in objects.iter() {
            obj_render.render();
        }
        self.context.window.gl_swap_window();
    }

    pub fn resize_viewport(&mut self, w: i32, h: i32) {
        self.viewport.update_size(w, h);
        self.viewport.set_used();
    }
}