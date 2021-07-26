use crate::render_gl::data;

// todo: split geometry and opengl logic
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct VertexColored {
    pub pos: data::f32_f32_f32,
    pub clr: data::f32_f32_f32_f32,
}

impl VertexShaderDataSetter for VertexColored {
    fn set_vertex_shader_data() {
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
    fn transpose(&mut self, x: f32, y: f32, z: f32) {
        self.pos = (self.pos.d0 + x, self.pos.d1 + y, self.pos.d2 + z).into()
    }

    fn get_pos(&self) -> (f32, f32, f32) {
        // im am afraid to return f32_f32_f32 as it is packed
        // so I return tuple but this is stack allocation...
        return match self.pos {
            val => (val.d0, val.d1, val.d2)
        };
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)] // todo: why packed?
pub struct VertexTextured {
    pub pos: data::f32_f32_f32,
    pub clr: data::f32_f32_f32_f32,
    pub tx_coords: data::f32_f32,
}

impl VertexShaderDataSetter for VertexTextured {
    fn set_vertex_shader_data() {
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

    fn transpose(&mut self, x: f32, y: f32, z: f32) {
        self.pos = (self.pos.d0 + x, self.pos.d1 + y, self.pos.d2 + z).into()
    }

    fn get_pos(&self) -> (f32, f32, f32) {
        // im am afraid to return f32_f32_f32 as it is packed and references work weird with it
        // this https://github.com/rust-lang/rust/issues/27060
        // so I return tuple but this is stack allocation... and all those clones
        return (self.pos.d0.clone(), self.pos.d1.clone(), self.pos.d2.clone());
    }
}

pub trait VertexShaderDataSetter {
    fn set_vertex_shader_data();
    fn transpose(&mut self, x: f32, y: f32, z: f32);
    fn get_pos(&self) -> (f32, f32, f32);
}