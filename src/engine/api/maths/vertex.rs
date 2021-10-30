use glam::Vec3;

use crate::engine::rendering::data;

// todo: split geometry and opengl logic
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct ColoredVertexData {
    pub pos: data::f32_f32_f32,
    pub clr: data::f32_f32_f32_f32,
}

impl VertexShaderDataConfigurer for ColoredVertexData {
    fn configure_vertex_shader_data() {
        let stride = std::mem::size_of::<Self>(); // byte offset between consecutive attributes

        let location = 0; // layout (location = 0)
        let offset = 0; // offset of the first component
        unsafe {
            data::f32_f32_f32::vertex_attrib_pointer(stride, location, offset);
        }

        let location = 1; // layout (location = 1)
        let offset = offset + std::mem::size_of::<data::f32_f32_f32>(); // offset of the first component
        unsafe {
            data::f32_f32_f32_f32::vertex_attrib_pointer(stride, location, offset);
        }
    }

    // todo: should be immutable?
    fn transpose_deprecated(&mut self, x: f32, y: f32, z: f32) {
        self.pos = (self.pos.d0 + x, self.pos.d1 + y, self.pos.d2 + z).into()
    }

    fn transpose(&mut self, offset: &Vec3) {
        self.pos = (self.pos.d0 + offset.x, self.pos.d1 + offset.y, self.pos.d2 + offset.z).into()
    }

    fn get_pos_deprecated(&self) -> (f32, f32, f32) {
        // im am afraid to return f32_f32_f32 as it is packed
        // so I return tuple but this is stack allocation...
        return match self.pos {
            val => (val.d0, val.d1, val.d2)
        };
    }

    fn get_pos(&self) -> Vec3 {
        glam::vec3(self.pos.d0, self.pos.d1, self.pos.d2)
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)] // todo: why packed?
pub struct TexturedVertexData {
    pub pos: data::f32_f32_f32,
    pub clr: data::f32_f32_f32_f32,
    pub tx_coords: data::f32_f32,
}

impl VertexShaderDataConfigurer for TexturedVertexData {
    fn configure_vertex_shader_data() {
        let stride = std::mem::size_of::<Self>(); // byte offset between consecutive attributes

        let location = 0;
        let offset = 0; // offset of the first component
        unsafe {
            data::f32_f32_f32::vertex_attrib_pointer(stride, location, offset);
        }

        let location = 1;
        let offset = offset + std::mem::size_of::<data::f32_f32_f32>();
        unsafe {
            data::f32_f32_f32_f32::vertex_attrib_pointer(stride, location, offset);
        }

        let location = 2;
        let offset = offset + std::mem::size_of::<data::f32_f32_f32_f32>();
        unsafe {
            data::f32_f32::vertex_attrib_pointer(stride, location, offset);
        }
    }

    fn transpose_deprecated(&mut self, x: f32, y: f32, z: f32) {
        self.pos = (self.pos.d0 + x, self.pos.d1 + y, self.pos.d2 + z).into()
    }

    fn transpose(&mut self, offset: &Vec3) {
        self.pos = (self.pos.d0 + offset.x, self.pos.d1 + offset.y, self.pos.d2 + offset.z).into()
    }

    fn get_pos_deprecated(&self) -> (f32, f32, f32) {
        // im am afraid to return f32_f32_f32 as it is packed and references work weird with it
        // this https://github.com/rust-lang/rust/issues/27060
        // so I return tuple but this is stack allocation... and all those clones
        return (self.pos.d0.clone(), self.pos.d1.clone(), self.pos.d2.clone());
    }

    fn get_pos(&self) -> Vec3 {
        glam::vec3(self.pos.d0, self.pos.d1, self.pos.d2)
    }
}

pub trait VertexShaderDataConfigurer {
    fn configure_vertex_shader_data();

    // todo: reduce code duplication between implementations
    fn transpose_deprecated(&mut self, x: f32, y: f32, z: f32);
    fn transpose(&mut self, offset: &glam::Vec3);

    fn get_pos_deprecated(&self) -> (f32, f32, f32);
    fn get_pos(&self) -> glam::Vec3;

}