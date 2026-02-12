use std::f32;

use crate::random::hash::pcg_11;

pub fn unit_vector_12(r: u32) -> glam::Vec2 {
    let theta = (pcg_11(r) as f32) / (u32::MAX as f32) * 2.0 * f32::consts::PI;

    glam::Vec2::new(theta.cos(), theta.sin())
}

pub fn unit_vector_23(r: glam::UVec2) -> glam::Vec3 {
    let theta = (r.x as f32) / (u32::MAX as f32) * 2.0 * f32::consts::PI;
    let z = (r.y as f32) / (u32::MAX as f32) * 2.0 - 1.0;

    let radius = (1.0 - z * z).sqrt();
    glam::Vec3::new(theta.cos() * radius, theta.sin() * radius, z)
}

pub fn unit_vector_13(r: u32) -> glam::Vec3 {
    unit_vector_23(glam::uvec2(r, pcg_11(r)))
}