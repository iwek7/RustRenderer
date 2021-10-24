use crate::engine::api::drawable::Drawable;
use crate::engine::api::game_api::CameraConfig;
use crate::engine::render_gl;
use crate::engine::api::render_util::RenderUtil;
use crate::engine::opengl_context::OpenglContext;

pub struct Renderer {
    viewport: render_gl::Viewport,
}

impl Renderer {
    pub fn new(context: &OpenglContext) -> Renderer {
        let viewport = render_gl::Viewport::for_window(context.window.size().0 as i32, context.window.size().1 as i32);
        viewport.set_used();
        unsafe {
            gl::ClearColor(0.0,0.0,0.0, 1.0);
        }

        Renderer {
            viewport,
        }
    }

    pub fn render(&mut self, objects: &[&dyn Drawable], active_camera_config: &CameraConfig, context: &OpenglContext) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        for obj_render in objects.iter() {
            obj_render.render(&RenderUtil::new(active_camera_config.clone(), context));
        }
        context.window.gl_swap_window();
    }

    pub fn resize_viewport(&mut self, w: i32, h: i32) {
        self.viewport.update_size(w, h);
        self.viewport.set_used();
    }
}
