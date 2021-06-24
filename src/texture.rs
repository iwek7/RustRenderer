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

            // todo: why gl:: can't be here? it should be used as everywhere...
            // gl::TexParameteri(gl::GL_TEXTURE_2D, gl::GL_TEXTURE_WRAP_S, gl::GL_REPEAT);
            // gl::TexParameteri(gl::GL_TEXTURE_2D, gl::GL_TEXTURE_WRAP_T, gl::GL_REPEAT);
            // gl::TexParameteri(gl::GL_TEXTURE_2D, gl::GL_TEXTURE_MIN_FILTER, gl::GL_LINEAR);
            // gl::TexParameteri(gl::GL_TEXTURE_2D, gl::GL_TEXTURE_MAG_FILTER, gl::GL_LINEAR);
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
                           img_ptr);
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        return Texture {
            texture_id
        };
    }

    // todo drop
}