use crate::renderer::RenderUtil;
use crate::opengl_context::OpenglContext;
use sdl2::event::Event;

// todo rename
pub trait Drawable {
    fn render(&self, render_util: &RenderUtil);
    fn handle_event(&mut self, event: &Event, context: &OpenglContext) {}
}
