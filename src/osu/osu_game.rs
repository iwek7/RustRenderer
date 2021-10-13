use std::rc::Rc;

use sdl2::event::Event;
use soloud::*;

use crate::api::drawable::{Drawable, UpdateContext};
use crate::api::resource_manager::ResourceManager;
use crate::create_rect_coords_colored_deprecated;
use crate::maths::circle::Circle;
use crate::maths::quadrangle::Quadrangle;
use crate::maths::shapes_common::Area;
use crate::maths::vertex::ColoredVertexData;
use crate::opengl_context::OpenglContext;
use crate::osu::playing_field::PlayingField;
use crate::renderer::RenderUtil;

pub struct OsuGame {
    playing_field: PlayingField,
}

impl OsuGame {
    pub fn new(resource_manager: Rc<ResourceManager>) -> OsuGame {
        let playing_field = PlayingField::new(
            &glam::vec3(-24.930449, -18.174343, 0.0),
            &glam::vec2(4.9304495 + 24.930449, 18.174343 - 1.8412428), //4.9304495, -1.8412428
            Rc::clone(&resource_manager));


        let mut sl = Soloud::default().unwrap();
        let wav = resource_manager.fetch_audio("osu/audio/a_cruel_angel_thesis.ogg");
        sl.play(&wav);
        while sl.voice_count() > 0 {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        OsuGame {
            playing_field,
        }
    }
}

impl<'a> Drawable for OsuGame {
    fn render(&self, render_util: &RenderUtil) {
        self.playing_field.render(render_util);
    }

    fn update(&mut self, update_context: &UpdateContext) {
        self.playing_field.update(update_context);
    }

    fn handle_event(&mut self, event: &Event, context: &OpenglContext, update_context: &UpdateContext) {
        self.playing_field.handle_event(event, context, update_context)
    }
}