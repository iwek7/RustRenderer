use std;
use std::ffi;
use std::ffi::{CStr, CString};

use gl;

pub struct ShaderProgram {
    id: gl::types::GLuint,
    name: String,
}

impl ShaderProgram {
    pub fn from_shaders(shaders: &[Shader], name: &str) -> Result<ShaderProgram, ShaderError> {
        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe { gl::AttachShader(program_id, shader.id()); }
        }

        unsafe { gl::LinkProgram(program_id); }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(ShaderError::CanNotLinkProgram { message: error.to_string_lossy().into_owned() });
        }

        // continue with error handling here

        for shader in shaders {
            unsafe { gl::DetachShader(program_id, shader.id()); }
        }

        println!("Created shader program {:?}, it has id {:?}", name, program_id);

        Ok(ShaderProgram { id: program_id, name: name.parse().unwrap() })
    }

    pub fn new(vertex_shader_raw: &ffi::CString, frag_shader_raw: &ffi::CString, name: &str) -> Result<ShaderProgram, ShaderError> {
        let shaders = vec!(
            Shader::from_c_ctr(vertex_shader_raw, ShaderType::VERTEX).unwrap(),
            Shader::from_c_ctr(frag_shader_raw, ShaderType::FRAG).unwrap()
        );
        ShaderProgram::from_shaders(&shaders[..], name)
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn set_mat4(&self, name: &str, mat4: glam::Mat4) {
        unsafe {
            let cname = std::ffi::CString::new(name).expect("CString::new failed");
            let loc = gl::GetUniformLocation(self.id, cname.as_ptr());
            gl::UniformMatrix4fv(
                loc as gl::types::GLint,
                1,
                gl::FALSE,
                &mat4.as_ref()[0],
            );
        }
    }

    pub fn set_vec2(&self, name: &str, vec2: glam::Vec2) {
        unsafe {
            let cname = std::ffi::CString::new(name).expect("CString::new failed");
            let loc = gl::GetUniformLocation(self.id, cname.as_ptr());
            gl::Uniform2fv(
                loc as gl::types::GLint,
                1,
                &vec2.as_ref()[0],
            );
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

pub enum ShaderType {
    VERTEX,
    FRAG,
}

impl ShaderType {
   pub fn file_extension(&self) -> &str {
        match &self {
            ShaderType::VERTEX => { ".vert" }
            ShaderType::FRAG => { ".frag" }
        }
    }

    pub fn gl_type(&self) -> gl::types::GLenum {
        match &self {
            ShaderType::VERTEX => { gl::VERTEX_SHADER }
            ShaderType::FRAG => { gl::FRAGMENT_SHADER }
        }
    }
}

#[derive(Clone)]
pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    pub fn from_c_ctr(str: &ffi::CString, shader_type: ShaderType) -> Result<Shader, ShaderError> {


        Shader::from_source(&str, shader_type)
    }

    fn from_source(
        source: &CStr,
        shader_type: ShaderType,
    ) -> Result<Shader, ShaderError> {
        let id = shader_from_source(source, shader_type)?;
        Ok(Shader { id })
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

fn shader_from_source(
    source: &CStr,
    shader_type: ShaderType,
) -> Result<gl::types::GLuint, ShaderError> {
    let id = unsafe { gl::CreateShader(shader_type.gl_type()) };
    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    };

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }
        let error: CString = create_whitespace_cstring_with_len(len as usize);
        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
            return Err(ShaderError::CanNotCompileShader { message: error.to_string_lossy().into_owned() });
        }
    }
    Ok(id)
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}

#[derive(Debug)]
pub enum ShaderError {
    CanNotDetermineShaderTypeForResource { name: String },
    CanNotLinkProgram { message: String },
    CanNotCompileShader { message: String },
    CanNotLoadShader { message: String /* inner: resource_loader::Error */ },
}
