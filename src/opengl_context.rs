use crate::engine::game_controller::CameraConfig;

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
            // todo is this needed?
            gl::Enable(gl::TEXTURE_2D);

            // todo: to enable this I would have to alternate z offset of pieces and chessboard
            // todo: to do so I would need to transfrom sdl point via camera transform to world space
            // gl::Enable(gl::DEPTH_TEST);

            // todo: this should be moved to texture rendering code and disabled afterwards
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
        self.engine_to_opengl_space_f(&(pos.0 as f32, pos.1 as f32, pos.2 as f32))
    }

    pub fn engine_to_opengl_space_f(&self, pos: &(f32, f32, f32)) -> (f32, f32, f32) {
        let win_size = self.window.size();
        return (
            2.0 * pos.0 / win_size.0 as f32 - 1.0,
            2.0 * pos.1 / win_size.1 as f32 - 1.0,
            0.0 // todo fix this when creating camera
        );
    }

    // based on https://stackoverflow.com/questions/7692988/opengl-math-projecting-screen-space-to-world-space-coords
    // todo: what about z (glReadPixels)
    pub fn sdl_space_to_world_space(&self, pos: &(i32, i32), camera_config: &CameraConfig) -> (f32, f32, f32) {

        println!("sdl pos {:?}", pos);

        // todo projection is duplicated, how to solve this?
        let projection = glam::Mat4::perspective_rh_gl(45.0, 3.0 / 3.0, 0.1, 100.0);
        let mut view = glam::Mat4::look_at_rh(
            camera_config.get_eye_position().clone(),
            camera_config.get_direction().clone(),
            camera_config.get_up_vector().clone(),
        );

        let pv = projection * view;
        println!("pv {:?}", pv);

        let reverse_vp = (pv).inverse();


        let opengl_mouse_pos = self.sdl_window_to_opengl_space(pos);
        let v = glam::Vec4::new(opengl_mouse_pos.0 as f32, opengl_mouse_pos.1 as f32, 1.0, 1.0);
        println!("pre raw {:?}", v);

        let res = reverse_vp * v;
        println!("raw {:?}", res);
        return (res.x / res.w, res.y / res.w, res.z);
    }
}
