use std::ffi::c_void;

use crate::resources::ImageData;

pub struct Texture {
    texture_id: gl::types::GLuint,
}

impl Texture {
    pub fn from_image(img_data: ImageData) -> Texture {
        let mut texture_id: gl::types::GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }
        let img_raw = img_data.image.as_raw();
        let img_ptr: *const c_void = img_raw.as_ptr() as *const _ as *const c_void;
        // todo: of course not always RGBA
        unsafe {
            gl::TexImage2D(gl::TEXTURE_2D,
                           0,
                           gl::RGBA as i32,
                           img_data.width as i32,
                           img_data.height as i32,
                           0,
                           gl::RGBA,
                           gl::UNSIGNED_BYTE,
                           img_ptr,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        return Texture {
            texture_id
        };
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
    // todo drop
}