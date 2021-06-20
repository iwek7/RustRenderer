use std::ffi::CString;
use std::path::Path;

use render_gl::buffer;
use render_gl::data;

use crate::resources::Resources;

pub mod render_gl;
pub mod resources;
pub mod renderer;
pub mod vertex;
mod triangle;

fn main() {
    let mut renderer = renderer::Renderer::new();
    renderer.render();
}

