use std::rc::Rc;

use sdl2::EventPump;
use sdl2::mouse::MouseButton;

use crate::engine::api::audio::AudioManager;
use crate::engine::api::coordinate_system::CoordinateSystem;
use crate::engine::api::drawable::{Drawable, UpdateContext};
use crate::engine::api::engine_utilities::EngineUtilities;
use crate::engine::api::game_api::GameController;
use crate::engine::api::maths::point::Point;
use crate::engine::api::maths::vertex::ColoredVertexDataLayout;
use crate::engine::opengl_context::OpenglContext;
use crate::engine::renderer;
use crate::engine::renderer::Renderer;
use crate::engine::resources::catching_resource_manager::CachingResourceManager;
use crate::games_root::GamesRoot;

const ENGINE_FEATURES_ON: bool = true;

pub struct Engine {
    // todo: this should not be concrete implementation
    // todo: but Box<dyn Drawable + GameController> does not work
    game: Option<GamesRoot>,
    coordinate_system: Box<dyn Drawable>,
    // one day generalize to engine overlay
    event_pump: EventPump,
    renderer: Renderer,
    opengl_context: OpenglContext,
    engine_utilities: Rc<EngineUtilities>,

}

impl Engine {
    pub fn new() -> Engine {
        let opengl_context = OpenglContext::init();

        let mut resource_manager = Rc::new(CachingResourceManager::new());
        let audio_manager = Rc::new(AudioManager::new());
        let engine_utilities = Rc::new(EngineUtilities::new(resource_manager, audio_manager));

        let mut event_pump = opengl_context.sdl.event_pump().unwrap();
        let mut renderer = renderer::Renderer::new(&opengl_context);

        let shader_material = engine_utilities.get_resource_manager().fetch_shader_material("chess/shaders/triangle");

        let mut coordinate_system = Box::new(CoordinateSystem::new(shader_material));

        Engine {
            game: None,
            coordinate_system,
            opengl_context,
            event_pump,
            renderer,
            engine_utilities,
        }
    }

    pub fn set_game(&mut self, games_root: GamesRoot) {
        self.game = Some(games_root)
    }

    pub fn get_engine_utilities(&self) -> Rc<EngineUtilities> {
        Rc::clone(&self.engine_utilities)
    }

    pub fn start(&mut self) {
        match &mut self.game {
            None => { panic!("Attempting to start game in engine, but no game was provided") }
            Some(game) => {
                let material = self.engine_utilities.get_resource_manager().fetch_shader_material("chess/shaders/triangle");
                let point = Point::new(
                    [ColoredVertexDataLayout { pos: (-2.0, -2.0, 0.0).into(), clr: (0.0, 0.0, 0.0, 1.0).into() }, ],
                    material,
                );
                'main: loop {
                    let mouse_state = self.event_pump.mouse_state();
                    let sdl_pos = glam::vec2(mouse_state.x().clone() as f32, mouse_state.y().clone() as f32);

                    let camera_config = game.get_camera_config();
                    let update_context = UpdateContext::new(sdl_pos, camera_config, Rc::clone(&self.engine_utilities));

                    game.update(&update_context);

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

                        game.handle_event(&event, &self.opengl_context, &update_context)
                    }
                    let camera_config = game.get_camera_config();
                    self.renderer.render(&mut [game, /*self.coordinate_system.borrow(),&point */],&camera_config , &self.opengl_context)
                }
            }
        }
    }
}
