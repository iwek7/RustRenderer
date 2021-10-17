use std::rc::Rc;

use sdl2::EventPump;
use sdl2::mouse::MouseButton;

use crate::engine::api::audio::AudioManager;
use crate::engine::api::coordinate_system::CoordinateSystem;
use crate::engine::api::drawable::{Drawable, UpdateContext};
use crate::engine::api::engine_utilities::EngineUtilities;
use crate::engine::api::game_api::GameController;
use crate::engine::api::maths::point::Point;
use crate::engine::api::maths::vertex::ColoredVertexData;
use crate::engine::api::resource_manager::ResourceManager;
use crate::engine::opengl_context::OpenglContext;
use crate::engine::renderer;
use crate::engine::renderer::Renderer;
use crate::games_root::GamesRoot;

const ENGINE_FEATURES_ON: bool = true;

pub struct Engine {
    // todo: this should not be concrete implementation
    // todo: but Box<dyn Drawable + GameController> does not work
    game: GamesRoot,
    coordinate_system: Box<dyn Drawable>,
    // one day generalize to engine overlay
    event_pump: EventPump,
    renderer: Renderer,
    opengl_context: OpenglContext,
    engine_utilities: Rc<EngineUtilities>,

}

impl Engine {
    pub fn new(game: GamesRoot, engine_utilities: Rc<EngineUtilities>, opengl_context: OpenglContext) -> Engine {
        let mut event_pump = opengl_context.sdl.event_pump().unwrap();
        let mut renderer = renderer::Renderer::new(&opengl_context);

        let shader_program = engine_utilities.get_resource_manager().fetch_shader_program("chess/shaders/triangle");

        let mut coordinate_system = Box::new(CoordinateSystem::new(shader_program));

        Engine {
            game,
            coordinate_system,
            opengl_context,
            event_pump,
            renderer,
            engine_utilities,
        }
    }

    pub fn start(&mut self) {
        let shader_program = self.engine_utilities.get_resource_manager().fetch_shader_program("chess/shaders/triangle");
        let point = Point::new(
            [ColoredVertexData { pos: (-2.0, -2.0, 0.0).into(), clr: (0.0, 0.0, 0.0, 1.0).into() }, ],
            Rc::clone(&shader_program),
        );
        'main: loop {
            let mouse_state = self.event_pump.mouse_state();
            let sdl_pos = glam::vec2(mouse_state.x().clone() as f32, mouse_state.y().clone() as f32);
            let camera_config = self.game.get_camera_config();
            let update_context = UpdateContext::new(sdl_pos, camera_config, Rc::clone(&self.engine_utilities));

            self.game.update(&update_context);

            for event in self.event_pump.poll_iter() {
                if ENGINE_FEATURES_ON {
                    match event {
                        sdl2::event::Event::Quit { .. } => break 'main,
                        sdl2::event::Event::Window {
                            win_event: sdl2::event::WindowEvent::Resized(w, h),
                            ..
                        } => {
                            self.renderer.resize_viewport(w, h);
                        }
                        sdl2::event::Event::MouseButtonDown {
                            mouse_btn, ..
                        } => {
                            if mouse_btn == MouseButton::Left {
                                match self.opengl_context.sdl_space_to_world_space_at_z0(update_context.get_sdl_mouse_position(), update_context.get_camera_config()) {
                                    None => {
                                        println!("Could not find mouse and z0 crosspoint")
                                    }
                                    Some(pos) => {
                                        println!("Mouse position is {:?}", pos)
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }

                self.game.handle_event(&event, &self.opengl_context, &update_context)
            }

            self.renderer.render(&[&self.game, /*self.coordinate_system.borrow(),&point */ ], &self.game.get_camera_config(), &self.opengl_context)
        }
    }
}
