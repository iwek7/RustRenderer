pub struct OpenglContext {
    pub sdl: sdl2::Sdl,
    pub window: sdl2::video::Window,
    // todo: make those private
    gl_context: sdl2::video::GLContext,
}


impl OpenglContext {
    pub fn init() -> OpenglContext {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 5);

        // todo: can window be moved to renderer?
        let window = video_subsystem
            .window("Game", 900, 700)
            .opengl()
            .resizable()
            .build()
            .unwrap();

        let gl_context = window.gl_create_context().unwrap();
        let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

        unsafe {
            gl::Enable(gl::TEXTURE_2D);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        OpenglContext {
            sdl,
            window,
            gl_context,
        }
    }

    pub fn sdl_window_to_opengl_space(&self, pos: &(i32, i32)) -> (f32, f32) {
        let win_size = self.window.size();
        return (
            2.0 * pos.0 as f32 / win_size.0 as f32 - 1.0,
            -(2.0 * pos.1 as f32 / win_size.1 as f32 - 1.0),
        );
    }

    pub fn engine_to_opengl_space(&self, pos: &(i32, i32, i32)) -> (f32, f32, f32) {
        let win_size = self.window.size();
        return (
            2.0 * pos.0 as f32 / win_size.0 as f32 - 1.0,
            2.0 * pos.1 as f32 / win_size.1 as f32 - 1.0,
            pos.2 as f32 // todo fix this when creating camera
        );
    }
}
