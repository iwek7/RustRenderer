use std::collections::HashMap;
use std::iter::Map;
use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::engine::rendering::ShaderProgram;

type UniformName = String;

#[derive(Clone)]
pub struct Material {
    shader_program: Rc<ShaderProgram>,
    uniforms: HashMap<UniformName, Uniform>,
    activation_time: Option<Instant>,
}

impl Material {
    pub fn new(shader_program: Rc<ShaderProgram>) -> Material {
        Material {
            shader_program,
            uniforms: HashMap::new(),
            activation_time: None,
        }
    }

    pub fn set_variable(&mut self, name: &str, kind: UniformKind) {
        let uniform_name = String::from(name);
        let uniform = Uniform::new(uniform_name.clone(), kind, &*self.shader_program);
        self.uniforms.insert(uniform_name, uniform);
    }

    pub fn activate(&mut self) {
        if let None = self.activation_time {
            self.activation_time = Some(Instant::now());
        }
        self.shader_program.set_used();
        self.uniforms.values().for_each(|uniform| { uniform.activate() })
    }

    pub fn get_active_duration(&self) -> Duration {
        match self.activation_time {
            None => Duration::new(0, 0),
            Some(time) => time.elapsed()
        }
    }
}

#[derive(Clone)]
struct Uniform {
    name: UniformName,
    kind: UniformKind,
    location: gl::types::GLint,
}

impl Uniform {
    fn new(name: UniformName, kind: UniformKind, program: &ShaderProgram) -> Uniform {
        let cname = std::ffi::CString::new(name.clone()).expect("CString::new failed");
        unsafe {
            let location = gl::GetUniformLocation(program.id(), cname.as_ptr()) as gl::types::GLint;
            Uniform {
                name,
                kind,
                location,
            }
        }
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn activate(&self) {
        self.kind.activate(self.location);
    }
}

#[derive(Clone)]
pub enum UniformKind {
    MAT_4 { value: glam::Mat4 },
    VEC_4 { value: glam::Vec4 },
    VEC_2 { value: glam::Vec2 },
    FLOAT { value: f32 },
}

impl UniformKind {
    fn activate(&self, location: gl::types::GLint) {
        unsafe {
            match &self {
                UniformKind::MAT_4 { value } => {
                    gl::UniformMatrix4fv(
                        location,
                        1,
                        gl::FALSE,
                        &value.as_ref()[0],
                    );
                }
                UniformKind::VEC_2 { value } => {
                    gl::Uniform2fv(
                        location,
                        1,
                        &value.as_ref()[0],
                    );
                }
                UniformKind::VEC_4 { value } => {
                    gl::Uniform4fv(
                        location,
                        1,
                        &value.as_ref()[0],
                    );
                }
                UniformKind::FLOAT { value } => {
                    gl::Uniform1f(location, *value);
                }
            }
        }
    }
}


