//  https://www.shadertoy.com/view/3dVXDc

use glam::Vec3Swizzles;

pub fn hash33(mut p: glam::Vec3) -> glam::Vec3 {
    p = glam::Vec3::fract(p * glam::vec3(0.1031f32, 0.1030f32, 0.0973f32));
    p += glam::Vec3::dot(p, p.yxz() + 33.33f32);
    glam::Vec3::fract((p.xxy() + p.yxx()) * p.zyx())
}

pub fn hash22(p: glam::Vec2) -> glam::Vec2 {
    let mut p3 =
        glam::Vec3::fract(glam::vec3(p.x, p.y, p.x) * glam::vec3(0.1031f32, 0.1030f32, 0.0973f32));
    p3 += glam::Vec3::dot(p3, p3.yzx() + 33.33f32);
    glam::Vec2::fract((p3.xx() + p3.yz()) * p3.zy())
}
