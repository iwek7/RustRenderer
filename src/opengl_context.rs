pub struct OpenglContext {
    pub sdl: sdl2::Sdl,
    pub window: sdl2::video::Window,
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

        OpenglContext {
            sdl,
            window,
            gl_context,
        }
    }
}
