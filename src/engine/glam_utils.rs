pub fn to_glam_vec(other: &(f32, f32, f32)) -> glam::Vec3 {
    glam::vec3(other.0, other.1, other.2)
}