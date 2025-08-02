use glam::Vec3Swizzles;

pub mod perlin;
pub mod worley;

//  https://www.shadertoy.com/view/3dVXDc

pub fn hash33(mut p: glam::Vec3) -> glam::Vec3 {
    p = glam::Vec3::fract(p * glam::vec3(0.1031f32, 0.1030f32, 0.0973f32));
    p += glam::Vec3::dot(p, p.yxz() + 33.33f32);
    glam::Vec3::fract((p.xxy() + p.yxx()) * p.zyx())
}

pub fn hash13(p: f32) -> glam::Vec3 {
    hash33(glam::Vec3::splat(p))
}
