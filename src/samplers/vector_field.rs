use crate::hash::{hash_ivec3, pcg_1d, rand_vector_3d};
use crate::samplers::perlin::sample_perlin_noise;
use crate::util::{clamp, clamp_vec3, mix, mix_vec3};

pub fn sample_vector_field(mut uvw: glam::Vec3, frequency: f32, seed: &mut u32) -> glam::Vec3 {
    uvw *= frequency;

    let pi = uvw.floor().as_ivec3();
    let pf = uvw.fract();

    let f = pf * pf * pf * (pf * (pf * 6f32 - 15f32) + 10f32);

    let d000 = random_direction(pi, pf, glam::ivec3(0, 0, 0), frequency, seed);
    let d001 = random_direction(pi, pf, glam::ivec3(0, 0, 1), frequency, seed);
    let d010 = random_direction(pi, pf, glam::ivec3(0, 1, 0), frequency, seed);
    let d011 = random_direction(pi, pf, glam::ivec3(0, 1, 1), frequency, seed);
    let d100 = random_direction(pi, pf, glam::ivec3(1, 0, 0), frequency, seed);
    let d101 = random_direction(pi, pf, glam::ivec3(1, 0, 1), frequency, seed);
    let d110 = random_direction(pi, pf, glam::ivec3(1, 1, 0), frequency, seed);
    let d111 = random_direction(pi, pf, glam::ivec3(1, 1, 1), frequency, seed);

    let x00 = mix_vec3(d000, d100, f.x);
    let x01 = mix_vec3(d001, d101, f.x);
    let x10 = mix_vec3(d010, d110, f.x);
    let x11 = mix_vec3(d011, d111, f.x);

    let y0 = mix_vec3(x00, x10, f.y);
    let y1 = mix_vec3(x01, x11, f.y);

    *seed = pcg_1d(*seed);
    mix_vec3(y0, y1, f.z)
}

pub fn sample_vector_field_fbm(
    uvw: glam::Vec3,
    num_octaves: u32,
    frequency: f32,
    lacunarity: f32,
    seed: &mut u32,
) -> glam::Vec3 {
    let mut sum = glam::Vec3::ZERO;
    let mut amplitude_sum = 0f32;

    for octave in 0..num_octaves {
        let attenuation = f32::powf(lacunarity, octave as f32);
        let amplitude = 1f32 / attenuation;

        let sample = sample_vector_field(uvw, frequency * attenuation, seed);

        sum += sample * amplitude;
        amplitude_sum += amplitude;
    }

    sum / amplitude_sum
}

fn random_direction(
    pi: glam::IVec3,
    pf: glam::Vec3,
    offset: glam::IVec3,
    frequency: f32,
    seed: &u32,
) -> glam::Vec3 {
    let splatted_frequency = glam::IVec3::splat(frequency as i32);
    (rand_vector_3d(
        (hash_ivec3((pi + offset).rem_euclid(splatted_frequency)) & 15u32).wrapping_add(*seed),
    ) * 2f32
        - 1f32)
}
