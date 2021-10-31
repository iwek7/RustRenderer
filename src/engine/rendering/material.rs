use std::collections::HashMap;
use std::iter::Map;
use std::rc::Rc;

use crate::engine::rendering::ShaderProgram;

type UniformName = String;

#[derive(Clone)]
pub struct Material {
    shader_program: Rc<ShaderProgram>,
    uniforms: HashMap<UniformName, Uniform>,
}

impl Material {
    pub fn new(shader_program: Rc<ShaderProgram>) -> Material {
        Material {
            shader_program,
            uniforms: HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, name: &str, kind: UniformKind) {
        let uniform_name = String::from(name);
        let uniform = Uniform::new(uniform_name.clone(), kind, &*self.shader_program);
        self.uniforms.insert(uniform_name, uniform);
    }

    pub fn activate(&self) {
        self.shader_program.set_used();
        self.uniforms.values().for_each(|uniform| { uniform.activate() })
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
}

impl UniformKind {
    fn activate(&self, location: gl::types::GLint) {
        match &self {
            UniformKind::MAT_4 { value } => {
                unsafe {
                    gl::UniformMatrix4fv(
                        location,
                        1,
                        gl::FALSE,
                        &value.as_ref()[0],
                    );
                }
            }
            UniformKind::VEC_2 { value } => {
                unsafe {
                    gl::Uniform2fv(
                        location,
                        1,
                        &value.as_ref()[0],
                    );
                }
            }
            UniformKind::VEC_4 { value } => {
                unsafe {
                    gl::Uniform4fv(
                        location,
                        1,
                        &value.as_ref()[0],
                    );
                }
            }
        }
    }
}


