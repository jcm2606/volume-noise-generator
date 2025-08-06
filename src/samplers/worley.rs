use crate::hash::{hash_ivec3, rand_vector_3d};
use crate::util::clamp;

pub fn sample_worley_noise(mut uvw: glam::Vec3, frequency: f32, seed: u32) -> f32 {
    let splatted_frequency = glam::Vec3::splat(frequency);
    uvw *= splatted_frequency;

    let p = uvw.floor().as_ivec3();
    let f = uvw.fract();

    let mut min_dist = 1f32;

    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                let offset = glam::ivec3(x, y, z);

                let hash =
                    hash_ivec3((p + offset).rem_euclid(splatted_frequency.as_ivec3())) + seed;
                let point = rand_vector_3d(hash) + offset.as_vec3();

                let delta = f - point;
                min_dist = f32::min(min_dist, delta.length());
            }
        }
    }

    clamp(1f32 - min_dist, -1f32, 1f32)
}

pub fn sample_worley_fbm(
    uvw: glam::Vec3,
    num_octaves: u32,
    frequency: f32,
    lacunarity: f32,
    seed: u32,
) -> f32 {
    let mut sum = 0f32;
    let mut amplitude_sum = 0f32;

    for octave in 0..num_octaves {
        let attenuation = f32::powf(lacunarity, octave as f32);
        let amplitude = 1f32 / attenuation;

        let sample = sample_worley_noise(uvw, frequency * attenuation, seed + octave);

        sum += sample * amplitude;
        amplitude_sum += amplitude;
    }

    clamp(sum / amplitude_sum, -1f32, 1f32)
}
