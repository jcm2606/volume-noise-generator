use std::f32;

pub fn pcg_1d(v: u32) -> u32 {
    let state = v.wrapping_mul(747796405u32).wrapping_add(2891336453u32);
    let word =
        ((state >> ((state >> 28u32).wrapping_add(4u32))) ^ state).wrapping_mul(277803737u32);
    (word >> 22u32) ^ word
}

pub fn pcg_2d(mut v: glam::UVec2) -> glam::UVec2 {
    v = v * 1664525u32 + 1013904223u32;

    v.x += v.y * 1664525u32;
    v.y += v.x * 1664525u32;

    v = v ^ (v >> 16u32);

    v.x += v.y * 1664525u32;
    v.y += v.x * 1664525u32;

    v = v ^ (v >> 16u32);

    v
}

pub fn pcg_3d(mut v: glam::UVec3) -> glam::UVec3 {
    v = v * 1664525u32 + 1013904223u32;

    v.x += v.y * v.z;
    v.y += v.z * v.x;
    v.z += v.x * v.y;

    v ^= v >> 16u32;

    v.x += v.y * v.z;
    v.y += v.z * v.x;
    v.z += v.x * v.y;

    v
}

pub fn rand_vector_2d(v: u32) -> glam::Vec2 {
    let x = pcg_1d(v ^ 0xA53C9A1Fu32);
    let y = pcg_1d(v ^ 0xC2B2AE35u32);

    (glam::vec2(x as f32, y as f32) / u32::MAX as f32) * 2.0 - 1.0
}

pub fn rand_vector_3d(v: u32) -> glam::Vec3 {
    let x = pcg_1d(v ^ 0xA53C9A1Fu32);
    let y = pcg_1d(v ^ 0xC2B2AE35u32);
    let z = pcg_1d(v ^ 0x27D4EB2Fu32);

    (glam::vec3(x as f32, y as f32, z as f32) / u32::MAX as f32) * 2.0 - 1.0
}

pub fn rand_unit_vector_3d(v: u32) -> glam::Vec3 {
    let theta = ((pcg_1d(v ^ 0xA53C9A1Fu32) as f32) / (u32::MAX as f32)) * 2.0 * f32::consts::PI;

    let z = (pcg_1d(v ^ 0xC2B2AE35u32) as f32) / (u32::MAX as f32) * 2.0 - 1.0;
    let radius = (1.0 - z * z).sqrt();

    let x = radius * theta.cos();
    let y = radius * theta.sin();

    glam::Vec3::new(x, y, z)
}

pub fn rand_unit_vector_2d(v: u32) -> glam::Vec2 {
    let theta = ((pcg_1d(v ^ 0xA53C9A1Fu32) as f32) / (u32::MAX as f32)) * 2.0 * f32::consts::PI;

    let x = theta.cos();
    let y = theta.sin();

    glam::Vec2::new(x, y)
}

pub fn per_pixel_seed(xyz: glam::UVec3, size: glam::UVec3) -> u32 {
    xyz.x + xyz.y * size.x + xyz.z * size.x * size.y
}

pub fn hash_ivec3(v: glam::IVec3) -> u32 {
    let x = i32::cast_unsigned(v.x);
    let y = i32::cast_unsigned(v.y);
    let z = i32::cast_unsigned(v.z);

    let mut h = 0xDEADBEEFu32;
    h ^= x
        .wrapping_add(0x9E3779B9u32)
        .wrapping_add(h << 6u32)
        .wrapping_add(h >> 2u32);
    h ^= y
        .wrapping_add(0x9E3779B9u32)
        .wrapping_add(h << 6u32)
        .wrapping_add(h >> 2u32);
    h ^= z
        .wrapping_add(0x9E3779B9u32)
        .wrapping_add(h << 6u32)
        .wrapping_add(h >> 2u32);

    h ^= h >> 16;
    h = h.wrapping_mul(0x85EBCA6Bu32);
    h ^= h >> 13;
    h = h.wrapping_mul(0xC2B2AE35u32);
    h ^= h >> 16;

    h
}

pub fn hash_ivec2(v: glam::IVec2) -> u32 {
    let x = i32::cast_unsigned(v.x);
    let y = i32::cast_unsigned(v.y);

    let mut h = 0xDEADBEEFu32;
    h ^= x
        .wrapping_add(0x9E3779B9u32)
        .wrapping_add(h << 6u32)
        .wrapping_add(h >> 2u32);
    h ^= y
        .wrapping_add(0x9E3779B9u32)
        .wrapping_add(h << 6u32)
        .wrapping_add(h >> 2u32);

    h ^= h >> 16;
    h = h.wrapping_mul(0x85EBCA6Bu32);
    h ^= h >> 13;
    h = h.wrapping_mul(0xC2B2AE35u32);
    h ^= h >> 16;

    h
}
