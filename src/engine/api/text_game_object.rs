use std::rc::Rc;

use glam::Vec3;

use crate::{create_colored_rect_coords, create_rect_coords};
use crate::engine::api::colour::Colour;
use crate::engine::api::drawable::Drawable;
use crate::engine::api::engine_utilities::EngineUtilities;
use crate::engine::resources::fonts::SizedFont;
use crate::engine::api::maths::quadrangle::Quadrangle;
use crate::engine::api::maths::vertex::TexturedVertexData;
use crate::engine::api::render_util::RenderUtil;
use crate::engine::render_gl::ShaderProgram;

pub struct TextGameObject {
    sized_font: Rc<SizedFont>,
    text: String,
    quads: Vec<Quadrangle<TexturedVertexData>>,
    shader_program: Rc<ShaderProgram>,
    position: Vec3,
    colour: Colour,
}

// resources:
// https://www.freetype.org/freetype2/docs/glyphs/glyphs-3.html
// https://learnopengl.com/In-Practice/Text-Rendering
// https://github.com/jhasse/rust-opengl-test
impl TextGameObject {
    pub fn new(sized_font: Rc<SizedFont>, text: &str, position: Vec3, shader_program: Rc<ShaderProgram>, colour: Colour) -> TextGameObject {
        TextGameObject {
            sized_font: Rc::clone(&sized_font),
            text: String::from(text),
            quads: TextGameObject::init_quads(sized_font, text, position, Rc::clone(&shader_program), &colour),
            shader_program,
            position,
            colour,
        }
    }

    fn init_quads(sized_font: Rc<SizedFont>, text: &str, position: Vec3, shader_program: Rc<ShaderProgram>, colour: &Colour) -> Vec<Quadrangle<TexturedVertexData>> {
        let mut shift = 0.0;
        let scale = 0.01;
        let mut quads = vec!();
        for ch in text.chars() {
            let font_character = sized_font.get_char(ch);

            let scaled_bearing = font_character.get_bearing().x * scale;
            let x_pos = position.x + shift + scaled_bearing;

            // for characters below baseline like p or g
            let y_pos = position.y - (font_character.get_size().y - font_character.get_bearing().y) * scale;

            let w = font_character.get_size().x * scale;
            let h = font_character.get_size().y * scale;

            // >> 6 (/ 64) because advance is expressed in unit of 1/64 of pixel
            shift += (*font_character.get_advance() >> 6) as f32 * scale;

            // for debugging when debuger breaks :)
            // println!("ch {:?} xpos {:?} w {:?} h {:?} scaled shift {:?} advance {:?} advance shifted {:?} bearing {:?} size {:?}",
            //          ch as i32, x_pos, font_character.get_size().x, font_character.get_size().y,
            //          shift, font_character.get_advance(), font_character.get_advance() >> 6, font_character.get_bearing(), font_character.get_size());

            let quad = Quadrangle::new(
                create_colored_rect_coords(
                    &glam::vec3(x_pos, y_pos, position.z),
                    &glam::vec2(w, h),
                    &font_character.get_texture().topology.get_sprite_coords(0, 0).unwrap(),
                    colour,
                ),
                [0, 1, 3, 1, 2, 3],
                Rc::clone(&shader_program),
                Some(font_character.get_texture()),
            );
            quads.push(quad);
        }
        quads
    }

    pub fn set_text(&mut self, new_text: String) {
        if self.text != new_text {
            let new_quads = TextGameObject::init_quads(
                Rc::clone(&self.sized_font),
                new_text.as_str(),
                self.position,
                Rc::clone(&self.shader_program),
                &self.colour,
            );
            self.quads = new_quads;
        }
    }
}

impl Drawable for TextGameObject {
    fn render(&self, render_util: &RenderUtil) {
        self.quads.iter().for_each(|q| q.render(render_util))
    }
}