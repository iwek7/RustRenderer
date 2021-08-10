use crate::maths::triangle::Drawable;
use crate::opengl_context::OpenglContext;
use crate::render_gl;
use crate::engine::game_controller::CameraConfig;

pub struct Renderer<'a> {
    context: &'a OpenglContext,
    viewport: render_gl::Viewport,
}

impl<'a> Renderer<'a> {
    pub fn new(context: &OpenglContext) -> Renderer {
        let viewport = render_gl::Viewport::for_window(900, 700);
        viewport.set_used();
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        }

        Renderer {
            context,
            viewport,
        }
    }

    pub fn render(&mut self, objects: &[&dyn Drawable], active_camera_config: &CameraConfig) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        for obj_render in objects.iter() {
            obj_render.render(&RenderUtil::new());
        }
        self.context.window.gl_swap_window();
    }

    pub fn resize_viewport(&mut self, w: i32, h: i32) {
        self.viewport.update_size(w, h);
        self.viewport.set_used();
    }
}

pub struct RenderUtil {}

impl RenderUtil {
    fn new() -> RenderUtil {
        RenderUtil {}
    }
}
