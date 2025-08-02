use glam::Vec3;

use crate::noise::hash33;
use crate::util::clamp;

fn project(p: glam::Vec3, f: glam::Vec3, offset: glam::Vec3, frequency: f32) -> f32 {
    let gradient = hash33((p + offset).rem_euclid(Vec3::splat(frequency))) * 2.0f32 - 1.0f32;
    Vec3::dot(gradient, f - offset)
}

/// outputs [-1, 1)
pub fn sample_perlin_noise(uvw: glam::Vec3, frequency: f32) -> f32 {
    let p = uvw.floor();
    let f = uvw.fract();

    let u = f * f * f * (f * (f * 6f32 - 15f32) + 10f32);
    // let u = f * f * (3f32 - 2f32 * f);

    let projection_a = project(p, f, glam::vec3(0f32, 0f32, 0f32), frequency);
    let projection_b = project(p, f, glam::vec3(1f32, 0f32, 0f32), frequency);
    let projection_c = project(p, f, glam::vec3(0f32, 1f32, 0f32), frequency);
    let projection_d = project(p, f, glam::vec3(1f32, 1f32, 0f32), frequency);
    let projection_e = project(p, f, glam::vec3(0f32, 0f32, 1f32), frequency);
    let projection_f = project(p, f, glam::vec3(1f32, 0f32, 1f32), frequency);
    let projection_g = project(p, f, glam::vec3(0f32, 1f32, 1f32), frequency);
    let projection_h = project(p, f, glam::vec3(1f32, 1f32, 1f32), frequency);

    projection_a
        + u.x * (projection_b - projection_a)
        + u.y * (projection_c - projection_a)
        + u.z * (projection_e - projection_a)
        + u.x * u.y * (projection_a - projection_b - projection_c + projection_d)
        + u.y * u.z * (projection_a - projection_c - projection_e + projection_g)
        + u.z * u.x * (projection_a - projection_b - projection_e + projection_f)
        + u.x
            * u.y
            * u.z
            * (-projection_a + projection_b + projection_c - projection_d + projection_e
                - projection_f
                - projection_g
                + projection_h)
}

pub fn sample_perlin_fbm(uvw: glam::Vec3, mut frequency: f32, num_octaves: usize) -> f32 {
    let mut weight = 1.0f32;

    let mut sum = 0f32;
    let mut weight_sum = 0f32;

    for octave in 0..num_octaves {
        sum += (sample_perlin_noise(uvw * frequency, frequency)) * weight;
        weight_sum += weight;

        weight *= 0.5;
        frequency *= 2.0;
    }

    clamp((sum / weight_sum) * 0.5f32 + 0.5f32, 0f32, 1f32)
}
