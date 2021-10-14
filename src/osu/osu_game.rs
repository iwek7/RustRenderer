use std::rc::Rc;

use sdl2::event::Event;
use soloud::*;

use crate::engine::api::drawable::{Drawable, UpdateContext};
use crate::engine::api::resource_manager::ResourceManager;
use crate::create_rect_coords_colored_deprecated;
use crate::engine::api::engine_utilities::EngineUtilities;
use crate::engine::api::maths::circle::Circle;
use crate::engine::api::maths::quadrangle::Quadrangle;
use crate::engine::api::maths::shapes_common::Area;
use crate::engine::api::maths::vertex::ColoredVertexData;
use crate::engine::opengl_context::OpenglContext;
use crate::osu::playing_field::PlayingField;
use crate::engine::api::render_util::RenderUtil;
use crate::engine::engine::Engine;

pub struct OsuGame {
    playing_field: PlayingField,
}

impl OsuGame {
    pub fn new(engine_utilities: Rc<EngineUtilities>) -> OsuGame {
        let playing_field = PlayingField::new(
            &glam::vec3(-24.930449, -18.174343, 0.0),
            &glam::vec2(4.9304495 + 24.930449, 18.174343 - 1.8412428), //4.9304495, -1.8412428
            engine_utilities.get_resource_manager());

        let wav = engine_utilities.get_resource_manager().fetch_audio("osu/audio/a_cruel_angel_thesis.ogg");
        engine_utilities.get_audio_manager().play(wav);

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