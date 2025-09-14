use crate::hash::{hash_ivec2, hash_ivec3, pcg_1d, rand_vector_2d, rand_vector_3d};
use crate::util::{clamp, mix};

pub fn sample_perlin_noise(mut uvw: glam::Vec3, frequency: f32, seed: &mut u32) -> f32 {
    uvw *= frequency;

    let pi = uvw.floor().as_ivec3();
    let pf = uvw.fract();

    let f = pf * pf * pf * (pf * (pf * 6f32 - 15f32) + 10f32);

    let p000 = project(pi, pf, glam::ivec3(0, 0, 0), frequency, seed);
    let p001 = project(pi, pf, glam::ivec3(0, 0, 1), frequency, seed);
    let p010 = project(pi, pf, glam::ivec3(0, 1, 0), frequency, seed);
    let p011 = project(pi, pf, glam::ivec3(0, 1, 1), frequency, seed);
    let p100 = project(pi, pf, glam::ivec3(1, 0, 0), frequency, seed);
    let p101 = project(pi, pf, glam::ivec3(1, 0, 1), frequency, seed);
    let p110 = project(pi, pf, glam::ivec3(1, 1, 0), frequency, seed);
    let p111 = project(pi, pf, glam::ivec3(1, 1, 1), frequency, seed);

    let x00 = mix(p000, p100, f.x);
    let x01 = mix(p001, p101, f.x);
    let x10 = mix(p010, p110, f.x);
    let x11 = mix(p011, p111, f.x);

    let y0 = mix(x00, x10, f.y);
    let y1 = mix(x01, x11, f.y);

    *seed = pcg_1d(*seed);
    mix(y0, y1, f.z)
}

pub fn sample_perlin_fbm(
    uvw: glam::Vec3,
    num_octaves: u32,
    frequency: f32,
    lacunarity: f32,
    seed: &mut u32,
) -> f32 {
    let mut sum = 0f32;
    let mut amplitude_sum = 0f32;

    for octave in 0..num_octaves {
        let attenuation = f32::powf(lacunarity, octave as f32);
        let amplitude = 1f32 / attenuation;

        let sample = sample_perlin_noise(uvw, frequency * attenuation, seed);

        sum += sample * amplitude;
        amplitude_sum += amplitude;
    }

    clamp(sum / amplitude_sum, -1f32, 1f32)
}

fn project(
    pi: glam::IVec3,
    pf: glam::Vec3,
    offset: glam::IVec3,
    frequency: f32,
    seed: &u32,
) -> f32 {
    let splatted_frequency = glam::IVec3::splat(frequency as i32);
    let point = (rand_vector_3d(
        (hash_ivec3((pi + offset).rem_euclid(splatted_frequency)) & 15u32).wrapping_add(*seed),
    ) * 2f32
        - 1f32);

    glam::Vec3::dot(pf - offset.as_vec3(), point)
}

pub fn sample_perlin_noise_2d(mut uvw: glam::Vec2, frequency: f32, seed: &mut u32) -> f32 {
    uvw *= frequency;

    let pi = uvw.floor().as_ivec2();
    let pf = uvw.fract();

    let f = pf * pf * pf * (pf * (pf * 6f32 - 15f32) + 10f32);

    let p00 = project_2d(pi, pf, glam::ivec2(0, 0), frequency, seed);
    let p10 = project_2d(pi, pf, glam::ivec2(1, 0), frequency, seed);
    let p01 = project_2d(pi, pf, glam::ivec2(0, 1), frequency, seed);
    let p11 = project_2d(pi, pf, glam::ivec2(1, 1), frequency, seed);

    *seed = pcg_1d(*seed);
    mix(mix(p00, p10, f.x), mix(p01, p11, f.x), f.y)
}

pub fn sample_perlin_fbm_2d(
    uvw: glam::Vec2,
    num_octaves: u32,
    frequency: f32,
    lacunarity: f32,
    seed: &mut u32,
) -> f32 {
    let mut sum = 0f32;
    let mut amplitude_sum = 0f32;

    for octave in 0..num_octaves {
        let attenuation = f32::powf(lacunarity, octave as f32);
        let amplitude = 1f32 / attenuation;

        let sample = sample_perlin_noise_2d(uvw, frequency * attenuation, seed);

        sum += sample * amplitude;
        amplitude_sum += amplitude;
    }

    clamp(sum / amplitude_sum, -1f32, 1f32)
}

fn project_2d(
    pi: glam::IVec2,
    pf: glam::Vec2,
    offset: glam::IVec2,
    frequency: f32,
    seed: &u32,
) -> f32 {
    let splatted_frequency = glam::IVec2::splat(frequency as i32);
    let point = (rand_vector_2d(
        (hash_ivec2((pi + offset).rem_euclid(splatted_frequency)) & 15u32).wrapping_add(*seed),
    ) * 2f32
        - 1f32);

    glam::Vec2::dot(pf - offset.as_vec2(), point)
}
