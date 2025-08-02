use crate::samplers::{hash13, hash33};
use crate::util::clamp;

pub fn sample_worley_noise(mut uvw: glam::Vec3, frequency: f32, seed: f32) -> f32 {
    uvw *= frequency;

    let p = glam::Vec3::floor(uvw);
    let f = glam::Vec3::fract(uvw);

    let seed_offset = hash13(seed);

    let mut min_dist = 1.0;

    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                let offset = glam::vec3(x as f32, y as f32, z as f32);
                let point =
                    hash33((p + offset + seed_offset).rem_euclid(glam::Vec3::splat(frequency)))
                        + offset;

                let delta = f - point;
                min_dist = f32::min(min_dist, glam::Vec3::dot(delta, delta));
            }
        }
    }

    1f32 - clamp(f32::sqrt(min_dist), 0f32, 1f32)
}

pub fn sample_worley_fbm3(uvw: glam::Vec3, frequency: f32, seed: f32) -> f32 {
    let worley0 = sample_worley_noise(uvw, frequency, seed);
    let worley1 = sample_worley_noise(uvw, frequency * 2.0, seed + 1f32);
    let worley2 = sample_worley_noise(uvw, frequency * 4.0, seed + 2f32);

    worley0 * 0.625f32 + worley1 * 0.25f32 + worley2 * 0.125f32
}
