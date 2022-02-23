use crate::engine::api::game_object::GameObject;
use crate::engine::api::game_api::CameraConfig;
use crate::engine::api::render_util::RenderUtil;
use crate::engine::opengl_context::OpenglContext;
use crate::engine::rendering;

pub struct Renderer {
    viewport: rendering::Viewport,
}

impl Renderer {
    pub fn new(context: &OpenglContext) -> Renderer {
        let viewport = rendering::Viewport::for_window(context.window.size().0 as i32, context.window.size().1 as i32);
        viewport.set_used();
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        }

        Renderer {
            viewport,
        }
    }

    pub fn render(&mut self, objects: &mut [&mut dyn GameObject], active_camera_config: &CameraConfig, context: &OpenglContext) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        objects.iter_mut()
            .for_each(
                |obj| {
                    obj.render(
                        &RenderUtil::new(active_camera_config.clone(), context))
                }
            );

        context.window.gl_swap_window();
    }

    pub fn resize_viewport(&mut self, w: i32, h: i32) {
        self.viewport.update_size(w, h);
        self.viewport.set_used();
    }
}
