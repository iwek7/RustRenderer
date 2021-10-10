use std::rc::Rc;
use crate::api::drawable::Drawable;
use crate::api::resource_manager::ResourceManager;
use crate::{create_rect_coords_deprecated, create_rect_coords_colored, create_rect_coords_colored_deprecated, create_rect_coords};
use crate::api::colour::Colour;
use crate::maths::quadrangle::Quadrangle;
use crate::maths::vertex::{ColoredVertexData, TexturedVertexData};
use crate::renderer::RenderUtil;

pub struct PlayingField {
    background: Quadrangle<TexturedVertexData>
}

impl PlayingField {
    pub fn new(position: &glam::Vec3, size: &glam::Vec2, resource_manager: Rc<ResourceManager>) -> PlayingField {
        let shader = resource_manager.fetch_shader_program("osu/shaders/texture");
        let sanungoku = resource_manager.fetch_texture("osu/textures/EVANGELION_BG.jpg");
        let background = Quadrangle::new(
            create_rect_coords(position, size, &sanungoku.topology.get_sprite_coords(0, 0).unwrap()),
            [0, 1, 3, 1, 2, 3],
            shader,
            Some(sanungoku),
        );

        PlayingField {
            background
        }
    }
}

impl Drawable for PlayingField {
    fn render(&self, render_util: &RenderUtil) {
        self.background.render(render_util);
    }
}
