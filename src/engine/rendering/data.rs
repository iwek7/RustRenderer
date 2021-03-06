use glam::Vec4;

use crate::engine::api::colour::Colour;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f32_f32_f32 {
    pub d0: f32,
    pub d1: f32,
    pub d2: f32,
}

impl f32_f32_f32 {
    pub fn new(d0: f32, d1: f32, d2: f32) -> f32_f32_f32 {
        f32_f32_f32 {
            d0, d1, d2
        }
    }

    //todo: move it from here, why is it event here
    pub unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize) {
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
        gl::VertexAttribPointer(
            location as gl::types::GLuint,
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid
        );
    }

}

impl From<(f32, f32, f32)> for f32_f32_f32 {
    fn from(other: (f32, f32, f32)) -> Self {
        f32_f32_f32::new(other.0, other.1, other.2)
    }
}

impl From<(i32, i32, i32)> for f32_f32_f32 {
    fn from(other: (i32, i32, i32)) -> Self {
        f32_f32_f32::new(other.0 as f32, other.1 as f32, other.2 as f32)
    }
}

impl From<glam::Vec3> for f32_f32_f32 {
    fn from(other: glam::Vec3) -> Self {
        f32_f32_f32::new(other.x, other.y, other.z)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f32_f32 {
    pub d0: f32,
    pub d1: f32,
}

// todo: deduplication via inheritance
impl f32_f32 {
    pub fn new(d0: f32, d1: f32) -> f32_f32 {
        f32_f32 {
            d0, d1
        }
    }

    //todo: move it from here, why is it even here
    pub unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize) {
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
        gl::VertexAttribPointer(
            location as gl::types::GLuint,
            2, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid
        );
    }

}

impl From<(f32, f32)> for f32_f32 {
    fn from(other: (f32, f32)) -> Self {
        f32_f32::new(other.0, other.1)
    }
}



#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f32_f32_f32_f32 {
    pub d0: f32,
    pub d1: f32,
    pub d2: f32,
    pub d3: f32
}

impl f32_f32_f32_f32 {
    pub fn new(d0: f32, d1: f32, d2: f32, d3: f32) -> f32_f32_f32_f32 {
        f32_f32_f32_f32 {
            d0, d1, d2, d3
        }
    }

    //todo: move it from here, why is it event here
    pub unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize) {
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
        gl::VertexAttribPointer(
            location as gl::types::GLuint,
            4, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid
        );
    }

}

impl From<(f32, f32, f32, f32)> for f32_f32_f32_f32 {
    fn from(other: (f32, f32, f32, f32)) -> Self {
        f32_f32_f32_f32::new(other.0, other.1, other.2, other.3)
    }
}

impl From<glam::Vec4> for f32_f32_f32_f32 {
    fn from(other: Vec4) -> Self {
        f32_f32_f32_f32::new(other.x, other.y, other.z, other.w)
    }
}

impl From<Colour> for f32_f32_f32_f32 {
    fn from(other: Colour) -> Self {
        let raw: Vec4 = other.into();
        f32_f32_f32_f32::new(raw.x, raw.y, raw.z, raw.w)
    }
}