use crate::hash::{hash_ivec2, hash_ivec3, pcg_1d, rand_vector_2d, rand_vector_3d};
use crate::util::clamp;

pub fn sample_worley_noise(mut uvw: glam::Vec3, frequency: f32, seed: &mut u32) -> (f32, f32) {
    let splatted_frequency = glam::Vec3::splat(frequency);
    uvw *= splatted_frequency;

    let p = uvw.floor().as_ivec3();
    let f = uvw.fract();

    let mut f1_dist = 1f32;
    let mut f2_dist = 1f32;

    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                let offset = glam::ivec3(x, y, z);

                let hash = hash_ivec3((p + offset).rem_euclid(splatted_frequency.as_ivec3()))
                    .wrapping_add(*seed);
                let feature = rand_vector_3d(hash) + offset.as_vec3();

                let dist = (f - feature).length_squared();
                if dist < f1_dist {
                    f2_dist = f1_dist;
                    f1_dist = dist;
                } else if dist < f2_dist {
                    f2_dist = dist;
                }
            }
        }
    }

    f1_dist = clamp(f1_dist.sqrt(), -1f32, 1f32);
    f2_dist = clamp(f2_dist.sqrt(), -1f32, 1f32);

    *seed = pcg_1d(*seed);
    (f1_dist, f2_dist)
}

pub fn sample_worley_fbm(
    uvw: glam::Vec3,
    num_octaves: u32,
    frequency: f32,
    lacunarity: f32,
    seed: &mut u32,
) -> (f32, f32) {
    let mut f1_sum = 0f32;
    let mut f2_sum = 0f32;
    let mut amplitude_sum = 0f32;

    for octave in 0..num_octaves {
        let attenuation = f32::powf(lacunarity, octave as f32);
        let amplitude = 1f32 / attenuation;

        let (f1_sample, f2_sample) = sample_worley_noise(uvw, frequency * attenuation, seed);

        f1_sum += f1_sample * amplitude;
        f2_sum += f2_sample * amplitude;
        amplitude_sum += amplitude;
    }

    f1_sum = clamp(f1_sum / amplitude_sum, -1f32, 1f32);
    f2_sum = clamp(f2_sum / amplitude_sum, -1f32, 1f32);

    (f1_sum, f2_sum)
}

pub fn sample_worley_noise_2d(mut uvw: glam::Vec2, frequency: f32, seed: &mut u32) -> (f32, f32) {
    let splatted_frequency = glam::Vec2::splat(frequency);
    uvw *= splatted_frequency;

    let p = uvw.floor().as_ivec2();
    let f = uvw.fract();

    let mut f1_dist = 1f32;
    let mut f2_dist = 1f32;

    for x in -1..=1 {
        for y in -1..=1 {
            let offset = glam::ivec2(x, y);

            let hash = hash_ivec2((p + offset).rem_euclid(splatted_frequency.as_ivec2()))
                .wrapping_add(*seed);
            let feature = rand_vector_2d(hash) + offset.as_vec2();

            let dist = (f - feature).length_squared();
            if dist < f1_dist {
                f2_dist = f1_dist;
                f1_dist = dist;
            } else if dist < f2_dist {
                f2_dist = dist;
            }
        }
    }

    f1_dist = clamp(f1_dist.sqrt(), -1f32, 1f32);
    f2_dist = clamp(f2_dist.sqrt(), -1f32, 1f32);

    *seed = pcg_1d(*seed);
    (f1_dist, f2_dist)
}

pub fn sample_worley_fbm_2d(
    uvw: glam::Vec2,
    num_octaves: u32,
    frequency: f32,
    lacunarity: f32,
    seed: &mut u32,
) -> (f32, f32) {
    let mut f1_sum = 0f32;
    let mut f2_sum = 0f32;
    let mut amplitude_sum = 0f32;

    for octave in 0..num_octaves {
        let attenuation = f32::powf(lacunarity, octave as f32);
        let amplitude = 1f32 / attenuation;

        let (f1_sample, f2_sample) = sample_worley_noise_2d(uvw, frequency * attenuation, seed);

        f1_sum += f1_sample * amplitude;
        f2_sum += f2_sample * amplitude;
        amplitude_sum += amplitude;
    }

    f1_sum = clamp(f1_sum / amplitude_sum, -1f32, 1f32);
    f2_sum = clamp(f2_sum / amplitude_sum, -1f32, 1f32);

    (f1_sum, f2_sum)
}
