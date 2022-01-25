use std::fs::File;
use std::io;
use std::rc::Rc;

use crate::engine::api::colour::WHITE;
use crate::engine::api::drawable::{Drawable, UpdateContext};
use crate::engine::api::engine_utilities::EngineUtilities;
use crate::engine::api::maths::point::Point;
use crate::engine::api::maths::rectangle::Rectangle;
use crate::engine::api::render_util::RenderUtil;
use crate::engine::opengl_context::OpenglContext;
use crate::vertex::{ColoredVertexDataLayout, TexturedVertexDataLayout};

pub struct SubmarineGame {
    submarine: Submarine,
    engine_utilities: Rc<EngineUtilities>,
    lines: io::Lines<io::BufReader<File>>
}

impl SubmarineGame {
    pub fn new(engine_utilities: Rc<EngineUtilities>) -> SubmarineGame {
        let material = engine_utilities.get_resource_manager().fetch_shader_material("submarine/shaders/texture");
        let submarine_tx = engine_utilities.get_resource_manager().fetch_sprite("submarine/textures/submarine.jpg");
        let submarine_sprite = Rectangle::new_textured(
            &glam::vec3(-10.0, 9.1099205, 0.0),
            &glam::vec2(1.0, 1.0),
            material.clone(),
            submarine_tx,
        );

        let submarine = Submarine::new(submarine_sprite);
        let lines = engine_utilities.get_resource_manager().read_file_lines("submarine/commands.txt");
        SubmarineGame { submarine, engine_utilities, lines}
    }
}


impl Drawable for SubmarineGame {
    fn render(&mut self, render_util: &RenderUtil) {
        self.submarine.render(render_util)
    }

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
}


struct Submarine {
    submarine_sprite: Rectangle<TexturedVertexDataLayout>,
}

impl Submarine {
    pub fn new(submarine_sprite: Rectangle<TexturedVertexDataLayout>) -> Submarine {
        Submarine { submarine_sprite }
    }

    fn aim_up(&mut self, aim: i32) {
        println!("aiming up by {}", aim)
    }

    fn aim_down(&mut self, aim: i32) {
        println!("aiming down by {}", aim)
    }

    fn forward(&mut self, offset: i32) {
        println!("moving forward by {}", offset)
    }
}

impl Drawable for Submarine {
    fn render(&mut self, render_util: &RenderUtil) {
        self.submarine_sprite.render(render_util)
    }
}
