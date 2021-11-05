use std::rc::Rc;

use glam::Vec3;

use crate::engine::api::colour::{Colour, WHITE};
use crate::engine::api::drawable::Drawable;
use crate::engine::api::maths::rectangle::Rectangle;
use crate::engine::api::maths::shapes_common::Area;
use crate::engine::api::maths::vertex::TexturedVertexDataLayout;
use crate::engine::api::render_util::RenderUtil;
use crate::engine::rendering::material::{Material, UniformKind};
use crate::engine::resources::fonts::SizedFont;

pub struct TextGameObject {
    sized_font: Rc<SizedFont>,
    text: String,
    rects: Vec<Rectangle<TexturedVertexDataLayout>>,
    material: Material,
    position: Vec3,
    colour: Colour,
}

// resources:
// https://www.freetype.org/freetype2/docs/glyphs/glyphs-3.html
// https://learnopengl.com/In-Practice/Text-Rendering
// https://github.com/jhasse/rust-opengl-test
impl TextGameObject {
    pub fn new(sized_font: Rc<SizedFont>, text: &str, position: Vec3, material: Material, colour: Colour) -> TextGameObject {
        TextGameObject {
            sized_font: Rc::clone(&sized_font),
            text: String::from(text),
            rects: TextGameObject::init_rects(sized_font, text, position, material.clone(), &colour),
            material: material.clone(),
            position,
            colour,
        }
    }

    fn init_rects(sized_font: Rc<SizedFont>, text: &str, position: Vec3, mut material: Material, colour: &Colour) -> Vec<Rectangle<TexturedVertexDataLayout>> {
        let mut shift = 0.0;
        let scale = 0.01;
        let mut rects = vec!();
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

            material.set_variable("color", UniformKind::VEC_4 { value: colour.clone().into() });

            let rect = Rectangle::new_textured(
                &glam::vec3(x_pos, y_pos, position.z),
                &glam::vec2(w, h),
                material.clone(),
                font_character.get_sprite().clone(),
            );

            rects.push(rect);
        }
        rects
    }

    pub fn set_text(&mut self, new_text: String) {
        if self.text != new_text {
            let new_quads = TextGameObject::init_rects(
                Rc::clone(&self.sized_font),
                new_text.as_str(),
                self.position,
                self.material.clone(),
                &self.colour,
            );
            self.rects = new_quads;
        }
    }
}

impl Drawable for TextGameObject {
    fn render(&mut self, render_util: &RenderUtil) {
        self.rects.iter_mut().for_each(|q| q.render(render_util))
    }
}

impl Area for TextGameObject {
    fn contains_point(&self, point: &(f32, f32)) -> bool {
        self.rects.iter().any(|rect| rect.contains_point(point))
    }

    fn area(&self) -> f32 {
        todo!()
    }

    fn num_vertices(&self) -> usize {
        todo!()
    }

    fn get_pos(&self) -> &Vec3 {
        todo!()
    }

    fn move_to(&mut self, final_position: Vec3) {
        todo!()
    }

    fn move_by(&mut self, offset: Vec3) {
        todo!()
    }

    fn get_scale(&self) -> &Vec3 {
        todo!()
    }

    fn set_scale(&mut self, new_scale: Vec3) {
        todo!()
    }
}