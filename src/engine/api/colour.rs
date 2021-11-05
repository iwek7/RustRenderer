use glam::Vec4;

#[derive(Copy, Clone)]
pub struct Colour {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

pub const WHITE: Colour = Colour { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
pub const GREEN: Colour = Colour { r: 0.0, g: 0.5, b: 0.0, a: 1.0 };
pub const RED: Colour = Colour { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };

impl Colour {
    pub fn new_i(r: i32, g: i32, b: i32, a: f32) -> Colour {
        Colour::assert_color_i_value(r);
        Colour::assert_color_i_value(g);
        Colour::assert_color_i_value(b);
        Colour::assert_color_f_value(a);

        Colour {
            r: Colour::normalize_clr(r),
            g: Colour::normalize_clr(g),
            b: Colour::normalize_clr(b),
            a,
        }
    }

    pub fn new_f(r: f32, g: f32, b: f32, a: f32) -> Colour {
        Colour::assert_color_f_value(r);
        Colour::assert_color_f_value(g);
        Colour::assert_color_f_value(b);
        Colour::assert_color_f_value(a);

        Colour {
            r,
            g,
            b,
            a,
        }
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
}

impl From<glam::Vec4> for Colour {
    fn from(other: glam::Vec4) -> Self {
        Colour::new_f(other.x, other.y, other.z, other.w)
    }
}

impl Into<glam::Vec4> for Colour {
    fn into(self) -> Vec4 {
        glam::vec4(self.r, self.g, self.b, self.a)
    }
}

impl From<(f32, f32, f32, f32)> for Colour {
    fn from(other: (f32, f32, f32, f32)) -> Self {
        Colour::new_f(other.0, other.1, other.2, other.3)
    }
}