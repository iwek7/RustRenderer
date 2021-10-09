use std::rc::Rc;
use sdl2::event::Event;

use crate::api::drawable::{Drawable, UpdateContext};
use crate::api::resource_manager::ResourceManager;
use crate::create_rect_coords_colored;
use crate::maths::circle::Circle;
use crate::maths::quadrangle::Quadrangle;
use crate::maths::shapes_common::Area;
use crate::maths::vertex::ColoredVertexData;
use crate::opengl_context::OpenglContext;
use crate::renderer::RenderUtil;

pub struct OsuGame {
    test_circle: Circle
}

impl OsuGame {
    pub fn new(resource_manager: Rc<ResourceManager>) -> OsuGame {
        let shader = resource_manager.fetch_shader_program("shaders/triangle");

        let test_circle = Circle::new_colored(
            glam::vec3(-10.0, -10.0, 0.0),
            glam::vec4(0.5, 0.5, 0.5, 1.0),
            1.0,
            Rc::clone(&shader)
        );

        OsuGame {
            test_circle,
        }
    }
}

impl<'a> Drawable for OsuGame {
    fn render(&self, render_util: &RenderUtil)
    {
        self.test_circle.render(render_util);
    }

    fn handle_event(&mut self, event: &Event, context: &OpenglContext, update_context: &UpdateContext) {
        match context.sdl_space_to_world_space_at_z0(update_context.get_sdl_mouse_position(), &update_context.get_camera_config()) {
            None => {}
            Some(world_mouse_position) => {
                match event {
                    sdl2::event::Event::MouseButtonDown { .. } => {
                        if self.test_circle.contains_point(&(world_mouse_position.x, world_mouse_position.y)) {
                            println!("INSIDE")
                        } else {
                            println!("OUTSIDE")
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}