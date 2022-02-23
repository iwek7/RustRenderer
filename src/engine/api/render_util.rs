use glam::{Vec3};
use crate::engine::api::game_api::CameraConfig;
use crate::engine::opengl_context::OpenglContext;

pub struct RenderUtil<'a> {
    camera_config: CameraConfig,
    opengl_context: &'a OpenglContext,
}

impl<'a> RenderUtil<'a> {
    pub fn new(camera_config: CameraConfig, opengl_context: &'a OpenglContext) -> RenderUtil<'a> {
        RenderUtil {
            camera_config,
            opengl_context,
        }
    }

    pub fn calculate_camera_MVP(&self, position: glam::Vec3, scale: glam::Vec3, scale_point_offset: Vec3) -> glam::Mat4 {
        let projection = self.camera_config.get_projection_matrix(self.opengl_context.get_aspect_ratio());
        let view = self.camera_config.get_view_matrix();
        let scaling_conjugate = glam::Mat4::from_translation(scale_point_offset);
        let scaling_conjugate_inverse = scaling_conjugate.clone().inverse();
        let model = glam::Mat4::from_translation(position) * scaling_conjugate_inverse * glam::Mat4::from_scale(scale) * scaling_conjugate;
        // let model = glam::Mat4::from_scale_rotation_translation(scale, glam::quat(0.0, 0.0, 0.0, 0.0), position);
        return projection * view * model;
    }

    pub fn get_window_size(&self) -> glam::Vec2 {
        let win_size = self.opengl_context.window.size();
        glam::Vec2::new(win_size.0 as f32, win_size.1 as f32)
    }
}
