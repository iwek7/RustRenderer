use std::fs::File;
use std::io;
use std::ops::Mul;
use std::rc::Rc;

use crate::engine::api::colour::WHITE;
use crate::engine::api::engine_utilities::EngineUtilities;
use crate::engine::api::game_object::{BaseGameObject, GameObject, GameObjectId, UpdateContext};
use crate::engine::api::maths::point::Point;
use crate::engine::api::maths::rectangle::Rectangle;
use crate::engine::api::maths::shapes_common::Area;
use crate::engine::api::maths::vertex::{ColoredVertexDataLayout, TexturedVertexDataLayout};
use crate::engine::api::render_util::RenderUtil;
use crate::engine::opengl_context::OpenglContext;

/**
this is simulation of submarine movement from Advent Of Code 2021/3
 */
pub struct SubmarineGame {
    base_game_object: BaseGameObject,
    engine_utilities: Rc<EngineUtilities>,
    lines: io::Lines<io::BufReader<File>>,
}

impl SubmarineGame {
    pub fn new(engine_utilities: Rc<EngineUtilities>) -> SubmarineGame {
        let material = engine_utilities.get_resource_manager().fetch_shader_material("submarine/shaders/texture");
        let submarine_tx = engine_utilities.get_resource_manager().fetch_sprite("submarine/textures/submarine.png");
        let submarine_sprite = Rectangle::new_textured(
            &glam::vec3(-10.0, 9.1099205, 0.0),
            &glam::vec2(1.0, 1.0),
            material.clone(),
            submarine_tx,
        );

        let lines = engine_utilities.get_resource_manager().read_file_lines("submarine/commands.txt");
        let mut base_game_object =  BaseGameObject::new();
        base_game_object.add_child(GameObjectId::new("Submarine"), Box::new(Submarine::new(submarine_sprite)));
        SubmarineGame {
            base_game_object,
            engine_utilities,
            lines,
        }
    }
}


impl GameObject for SubmarineGame {


    fn update(&mut self, update_context: &UpdateContext) {
        if let Some(line) = self.lines.next() {
            let unwrapped = line.unwrap();
            let split = unwrapped.split_whitespace();
            let parts: Vec<&str> = split.collect();

            let offset = parts[1].parse::<i32>().unwrap();

            return match parts[0] {
                "forward" => self.submarine.forward(offset),
                "down" => self.submarine.aim_down(offset),
                "up" => self.submarine.aim_up(offset),
                unknown_command => panic!("Cannot parse submarine movement command {}", unknown_command)
            };
        }
    }

    fn base_game_object(&mut self) -> &mut BaseGameObject {
        &mut self.base_game_object
    }
}


struct Submarine {
    base_game_object: BaseGameObject,
    submarine_sprite: Rectangle<TexturedVertexDataLayout>,
    aim: i32,
}

impl Submarine {
    pub fn new(submarine_sprite: Rectangle<TexturedVertexDataLayout>) -> Submarine {
        Submarine {
            base_game_object: BaseGameObject::new(),
            submarine_sprite,
            aim: 0,
        }
    }

    fn aim_up(&mut self, aim: i32) {
        self.aim = self.aim + aim;
    }

    fn aim_down(&mut self, aim: i32) {
        self.aim = self.aim - aim;
    }

    fn forward(&mut self, offset: i32) {
        let move_offset = glam::vec3(offset as f32, (offset * self.aim) as f32, 0.0).mul(glam::vec3(0.001, 0.000001, 1.0));
        self.submarine_sprite.move_by(move_offset);
    }
}

impl GameObject for Submarine {
    fn render(&mut self, render_util: &RenderUtil) {
        self.submarine_sprite.render(render_util)
    }

    fn base_game_object(&mut self) -> &mut BaseGameObject {
        &mut self.base_game_object
    }
}
