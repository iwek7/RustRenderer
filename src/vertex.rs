use crate::render_gl::data;

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Vertex {
    pub pos: data::f32_f32_f32,
    pub clr: data::f32_f32_f32,
}

impl Vertex {
    pub fn vertex_attrib_pointers() {
        let stride = std::mem::size_of::<Self>(); // byte offset between consecutive attributes

        let location = 0; // layout (location = 0)
        let offset = 0; // offset of the first component
        unsafe {
            data::f32_f32_f32::vertex_attrib_pointer(stride, location, offset);
        }

        let location = 1; // layout (location = 1)
        let offset = offset + std::mem::size_of::<data::f32_f32_f32>(); // offset of the first component
        unsafe {
            data::f32_f32_f32::vertex_attrib_pointer(stride, location, offset);
        }
    }

    pub fn transpose(&mut self, x: f32, y: f32, z: f32) {
        self.pos = (self.pos.d0 + x, self.pos.d1 + y, self.pos.d2 + z).into()
    }

}
