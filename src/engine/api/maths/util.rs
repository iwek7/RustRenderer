use glam::Vec3;

fn lerp(a: glam::Vec3, b: glam::Vec3, t: f32) -> Vec3{
   a + (b - a) * t
}

// interpolation = A + (B - A) * t