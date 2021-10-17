use glam::Vec4;

#[derive(Copy, Clone)]
pub struct Colour {
    clr: Vec4,
}

impl Colour {
    pub fn new_i(r: i32, g: i32, b: i32, a: f32) -> Colour {
        Colour::assert_color_i_value(r);
        Colour::assert_color_i_value(g);
        Colour::assert_color_i_value(b);
        Colour::assert_color_f_value(a);

        Colour {
            clr: glam::vec4(Colour::normalize_clr(r), Colour::normalize_clr(g), Colour::normalize_clr(b), a)
        }
    }

    pub fn new_f(r: f32, g: f32, b: f32, a: f32) -> Colour {
        Colour::assert_color_f_value(r);
        Colour::assert_color_f_value(g);
        Colour::assert_color_f_value(b);
        Colour::assert_color_f_value(a);

        Colour {
            clr: glam::vec4(r, g, b, a)
        }
    }


    pub fn get_raw(&self) -> &Vec4 {
        &self.clr
    }

    fn assert_color_i_value(c: i32) {
        assert!(c >= 0 && c < 256)
    }

    fn assert_color_f_value(c: f32) {
        assert!(c >= 0.0 && c <= 1.1)
    }

    fn normalize_clr(c: i32) -> f32 {
        c as f32 / 255.0
    }

    /**
    presets
   */

    pub fn WHITE() -> Colour {
        Colour::new_f(1.0, 1.0, 1.0, 1.0)
    }

    pub fn GREEN() -> Colour {
        Colour::new_f(0.0, 0.5, 0.0, 1.0)
    }
}
