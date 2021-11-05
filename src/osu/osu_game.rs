use std::rc::Rc;

use glam::Vec3;
use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use soloud::*;

use crate::engine::api::colour::{Colour, WHITE};
use crate::engine::api::drawable::{Drawable, UpdateContext};
use crate::engine::api::engine_utilities::EngineUtilities;
use crate::engine::api::maths::circle::Circle;
use crate::engine::api::maths::quadrangle::Quadrangle;
use crate::engine::api::maths::shapes_common::Area;
use crate::engine::api::maths::vertex::ColoredVertexDataLayout;
use crate::engine::api::render_util::RenderUtil;
use crate::engine::api::text_game_object::TextGameObject;
use crate::engine::engine::Engine;
use crate::engine::opengl_context::OpenglContext;
use crate::osu::playing_field::PlayingField;

pub struct OsuGame {
    playing_field: PlayingField,
    score_label: TextGameObject,
    score_text: TextGameObject,
    start_game_button: TextGameObject,
    stop_game_button: TextGameObject,
    paused: bool,
}

const SCOREBOARD_PADDING: f32 = 4.0;
const TEXT_TOP_PADDING: f32 = 1.0;
const UI_LEFT_PADDING: f32 = 1.45;
const TEXT_Y_OFFSET: f32 = 1.0;
const BUTTONS_Y_OFFSET: f32 = 15.0;
const BUTTONS_PADDING: f32 = 1.0;

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
        // engine_utilities.get_audio_manager().play(wav);

        let text_material = engine_utilities.get_resource_manager().fetch_shader_material("osu/shaders/character");
        let sized_font = engine_utilities.get_resource_manager().fetch_font("osu/fonts/go3v2.ttf");
        let score_label = TextGameObject::new(Rc::clone(&sized_font), "SCORE",
                                              glam::vec3(playing_field_position.x + playing_field_size.x + UI_LEFT_PADDING,
                                                         playing_field_top_edge - TEXT_TOP_PADDING,
                                                         0.0,
                                              ),
                                              text_material.clone(),
                                              WHITE,
        );

        let score_text = TextGameObject::new(Rc::clone(&sized_font), "0",
                                             glam::vec3(playing_field_position.x + playing_field_size.x + UI_LEFT_PADDING,
                                                        playing_field_top_edge - TEXT_TOP_PADDING - TEXT_Y_OFFSET,
                                                        0.0,
                                             ),
                                             text_material.clone(),
                                             WHITE,
        );

        let start_game_button = TextGameObject::new(Rc::clone(&sized_font), "START",
                                                    glam::vec3(playing_field_position.x + playing_field_size.x + UI_LEFT_PADDING,
                                                               playing_field_top_edge - BUTTONS_Y_OFFSET,
                                                               0.0,
                                                    ),
                                                    text_material.clone(),
                                                    WHITE,
        );

        let stop_game_button = TextGameObject::new(sized_font, "STOP",
                                                   glam::vec3(playing_field_position.x + playing_field_size.x + UI_LEFT_PADDING,
                                                              playing_field_top_edge - BUTTONS_Y_OFFSET - BUTTONS_PADDING,
                                                              0.0,
                                                   ),
                                                   text_material,
                                                   WHITE,
        );

        OsuGame {
            playing_field,
            score_label,
            score_text,
            start_game_button,
            stop_game_button,
            paused: false,
        }
    }
}

impl<'a> Drawable for OsuGame {
    fn render(&mut self, render_util: &RenderUtil) {
        self.playing_field.render(render_util);
        self.score_label.render(render_util);
        self.score_text.render(render_util);
        self.start_game_button.render(render_util);
        self.stop_game_button.render(render_util);
    }

    fn update(&mut self, update_context: &UpdateContext) {
        if !self.paused {
            self.playing_field.update(update_context);
            self.score_text.set_text(self.playing_field.get_total_score().to_string());
        }
    }

    fn handle_event(&mut self, event: &Event, context: &OpenglContext, update_context: &UpdateContext) {
        match context.sdl_space_to_world_space_at_z0(update_context.get_sdl_mouse_position(), &update_context.get_camera_config()) {
            None => {}
            Some(world_mouse_position) => {
                match event {
                    sdl2::event::Event::MouseButtonDown { mouse_btn, .. } => {
                        match mouse_btn {
                            MouseButton::Left => {
                                if self.start_game_button.contains_point(&(world_mouse_position.x, world_mouse_position.y)) {
                                    self.paused = false
                                }

                                if self.stop_game_button.contains_point(&(world_mouse_position.x, world_mouse_position.y)) {
                                    self.paused = true
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }

        if !self.paused {
            self.playing_field.handle_event(event, context, update_context)
        }
    }
}