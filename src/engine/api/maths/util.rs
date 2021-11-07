pub fn lerp_v3(a: glam::Vec3, b: glam::Vec3, t: f32) -> glam::Vec3 {
    a + (b - a) * t
}

pub fn lerp_v4(a: glam::Vec4, b: glam::Vec4, t: f32) -> glam::Vec4 {
    a + (b - a) * t
}
