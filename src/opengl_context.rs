use std::ops::{Add, Mul};

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
    pub fn sdl_space_to_world_space_at_z0(&self, pos: &(i32, i32), camera_config: &CameraConfig) -> Option<glam::Vec3> {
        // println!("sdl pos {:?}", pos);
        // println!("camera pos {:?}", camera_config.get_eye_position());

        // 1. screen mouse coords to normalized space
        let opengl_mouse_pos = self.sdl_window_to_opengl_space(pos);
        let clip_coords = glam::Vec4::new(opengl_mouse_pos.0 as f32, opengl_mouse_pos.1 as f32, -1.0, 1.0);
        // println!("opengl mouse pos {:?}", clip_coords);

        // 2. Calculate ray
        // todo projection is duplicated, how to solve this?
        let eye_coords = OpenglContext::get_eye_coords(clip_coords, camera_config);
        let world_coords = OpenglContext::get_world_coords(eye_coords, camera_config);
        let ray_direction = world_coords.normalize();
        // println!("ray direction {:?}", ray_direction);

        // 3. Ray plane intersection - for simplicity we only consider plane located at z == 0
        let plane_normal = glam::Vec3::new(0., 0., 1.);

        let bot = ray_direction.clone().dot(plane_normal.clone());
        if bot.abs() < 0.01 {
            // println!("bot too small: {:?}, does not cross!", bot);
            return None;
        }

        let top = -(camera_config.get_eye_position().clone().dot(plane_normal.clone()));
        let t = top / bot;
        // println!("top {:?} bot {:?} t {:?} ", top, bot, t);

        let z_zero_plane_intersection = camera_config.get_eye_position().clone().add(ray_direction.clone().mul(t));

        return Some(z_zero_plane_intersection);
    }

    fn get_eye_coords(clip_coords: glam::Vec4, camera_config: &CameraConfig) -> glam::Vec4 {
        let inverted_projection = camera_config.get_projection_matrix().inverse();
        let eye_coords = inverted_projection * clip_coords;
        return glam::Vec4::new(eye_coords.x, eye_coords.y, -1.0, 0.0);
    }

    fn get_world_coords(eye_coords: glam::Vec4, camera_config: &CameraConfig) -> glam::Vec3 {
        let inverted_view = camera_config.get_view_matrix().inverse();
        let ray_world = inverted_view * eye_coords;
        return glam::Vec3::new(ray_world.x, ray_world.y, ray_world.z);
    }
}
