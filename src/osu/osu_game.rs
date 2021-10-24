use std::rc::Rc;

use sdl2::event::Event;
use soloud::*;

use crate::create_rect_coords_colored_deprecated;
use crate::engine::api::colour::Colour;
use crate::engine::api::drawable::{Drawable, UpdateContext};
use crate::engine::api::engine_utilities::EngineUtilities;
use crate::engine::api::maths::circle::Circle;
use crate::engine::api::maths::quadrangle::Quadrangle;
use crate::engine::api::maths::shapes_common::Area;
use crate::engine::api::maths::vertex::ColoredVertexData;
use crate::engine::api::render_util::RenderUtil;
use crate::engine::api::resource_manager::ResourceManager;
use crate::engine::api::text_game_object::TextGameObject;
use crate::engine::engine::Engine;
use crate::engine::opengl_context::OpenglContext;
use crate::osu::playing_field::PlayingField;

pub struct OsuGame {
    playing_field: PlayingField,
    score_label: TextGameObject,
    score_text: TextGameObject
}

const SCOREBOARD_PADDING: f32 = 4.0;
const TEXT_TOP_PADDING: f32 = 1.0;
const TEXT_LEFT_PADDING: f32 = 1.32;
const TEXT_Y_OFFSET: f32 = 1.0;

impl OsuGame {
    pub fn new(engine_utilities: Rc<EngineUtilities>) -> OsuGame {
        let playing_field_top_edge = -1.7;
        let playing_field_position = glam::vec3(-25.6, -18.2, 0.0);
        let playing_field_size = glam::vec2(4.9304495 + 25.6 - SCOREBOARD_PADDING, 18.2 + playing_field_top_edge);

        let playing_field = PlayingField::new(
            &playing_field_position,
            &playing_field_size,
            engine_utilities.get_resource_manager());

        let wav = engine_utilities.get_resource_manager().fetch_audio("osu/audio/a_cruel_angel_thesis.ogg");
        engine_utilities.get_audio_manager().play(wav);

        let text_shader = engine_utilities.get_resource_manager().fetch_shader_program("osu/shaders/character");
        let sized_font = engine_utilities.get_resource_manager().fetch_font("osu/fonts/go3v2.ttf");
        let score_label = TextGameObject::new(Rc::clone(&sized_font), "SCORE",
                                              glam::vec3(playing_field_position.x + playing_field_size.x + TEXT_LEFT_PADDING,
                                                   playing_field_top_edge - TEXT_TOP_PADDING,
                                                   0.0,
                                        ),
                                              Rc::clone(&text_shader),
                                              Colour::WHITE()
        );


        let score =  TextGameObject::new(sized_font, "0",
                                         glam::vec3(playing_field_position.x + playing_field_size.x + TEXT_LEFT_PADDING,
                                                    playing_field_top_edge - TEXT_TOP_PADDING - TEXT_Y_OFFSET,
                                                    0.0,
                                         ),
                                         text_shader,
                                         Colour::WHITE()
        );


        OsuGame {
            playing_field,
            score_label,
            score_text: score
        }
    }

}

impl<'a> Drawable for OsuGame {
    fn render(&self, render_util: &RenderUtil) {
        self.playing_field.render(render_util);
        self.score_label.render(render_util);
        self.score_text.render(render_util);
    }

    fn update(&mut self, update_context: &UpdateContext) {
        self.playing_field.update(update_context);
        self.score_text.set_text(self.playing_field.get_total_score().to_string());
    }

    fn handle_event(&mut self, event: &Event, context: &OpenglContext, update_context: &UpdateContext) {
        self.playing_field.handle_event(event, context, update_context)
    }
}