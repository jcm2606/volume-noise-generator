use glam::Vec3Swizzles;

use crate::noisetex::{NoisetexRgb8, NoisetexRgba8};
use crate::samplers::perlin::{
    sample_perlin_fbm, sample_perlin_fbm_2d, sample_perlin_noise, sample_perlin_noise_2d,
};
use crate::samplers::vector_field::{sample_vector_field, sample_vector_field_fbm};
use crate::samplers::worley::{
    sample_worley_fbm, sample_worley_fbm_2d, sample_worley_noise, sample_worley_noise_2d,
};
use crate::util::remap;

mod hash;
mod noisetex;
mod samplers;
mod util;

fn generate_lf_cloud_noisetex(index: u32, seed: &u32) {
    let mut noisetex = NoisetexRgba8::new(128, 128, 128);

    let seed = seed.wrapping_add(index);

    noisetex.fill(|info, pixel, pos| {
        let mut seed = seed;
        let uvw = pos.as_vec3() / info.size().as_vec3();

        let perlin_worley = {
            let perlin: f32 = sample_perlin_fbm(uvw, 3, 4f32, 2f32, &mut seed) * 0.5f32 + 0.5f32;
            let worley: f32 = 1f32 - sample_worley_fbm(uvw, 3, 4f32, 2f32, &mut seed).0;

            remap(perlin, 0.3f32, 0.7f32, worley, 1f32)
            // remap(perlin, 0f32, 1f32, worley, 1f32)
        };

        let worley_fbm_1 = 1f32 - sample_worley_fbm(uvw, 3, 8f32, 2f32, &mut seed).0;
        let worley_fbm_2 = 1f32 - sample_worley_fbm(uvw, 3, 16f32, 2f32, &mut seed).0;
        let worley_fbm_3 = 1f32 - sample_worley_fbm(uvw, 2, 32f32, 2f32, &mut seed).0;

        *pixel = (perlin_worley, worley_fbm_1, worley_fbm_2, worley_fbm_3).into();
    });

    // noisetex.save_as_image(format!("output/lfCloudNoiseTex_{index}.png"));
    noisetex.save_as_binary(format!("output/lfCloudNoiseTex_{index}.bin"));
}

fn generate_hf_cloud_noisetex(index: u32, seed: &u32) {
    let mut noisetex = NoisetexRgb8::new(32, 32, 32);

    let seed = seed.wrapping_add(index);

    noisetex.fill(|info, pixel, pos| {
        let mut seed = seed;
        let uvw = pos.as_vec3() / info.size().as_vec3();

        let worley_fbm_1 = 1f32 - sample_worley_fbm(uvw, 3, 4f32, 2f32, &mut seed).0;
        let worley_fbm_2 = 1f32 - sample_worley_fbm(uvw, 3, 8f32, 2f32, &mut seed).0;
        let worley_fbm_3 = 1f32 - sample_worley_fbm(uvw, 2, 16f32, 2f32, &mut seed).0;

        *pixel = (worley_fbm_1, worley_fbm_2, worley_fbm_3).into();
    });

    // noisetex.save_as_image(format!("output/hfCloudNoiseTex_{index}.png"));
    noisetex.save_as_binary(format!("output/hfCloudNoiseTex_{index}.bin"));
}

fn generate_perlin_noisetex(index: u32, seed: &u32) {
    let mut noisetex = NoisetexRgb8::new(512, 512, 1);

    let seed = seed.wrapping_add(index);

    noisetex.fill(|info, pixel, pos| {
        let mut seed = seed;
        let uvw = pos.as_vec3() / info.size().as_vec3();

        let noise_0 = sample_perlin_noise_2d(uvw.xy(), 4f32, &mut seed) * 0.5f32 + 0.5f32;
        let noise_1 = sample_perlin_noise_2d(uvw.xy(), 8f32, &mut seed) * 0.5f32 + 0.5f32;
        let noise_2 = sample_perlin_noise_2d(uvw.xy(), 16f32, &mut seed) * 0.5f32 + 0.5f32;

        *pixel = (noise_0, noise_1, noise_2).into();
    });

    noisetex.save_as_image(format!("output/perlinNoiseTex_{index}.png"));
}

fn generate_perlin_fbm_noisetex(index: u32, seed: &u32) {
    let mut noisetex = NoisetexRgba8::new(512, 512, 1);

    let seed = seed.wrapping_add(index);

    noisetex.fill(|info, pixel, pos| {
        let mut seed = seed;
        let uvw = pos.as_vec3() / info.size().as_vec3();

        let noise_0 = sample_perlin_fbm_2d(uvw.xy(), 3, 4f32, 2f32, &mut seed) * 0.5f32 + 0.5f32;
        let noise_1 = sample_perlin_fbm_2d(uvw.xy(), 3, 8f32, 2f32, &mut seed) * 0.5f32 + 0.5f32;
        let noise_2 = sample_perlin_fbm_2d(uvw.xy(), 3, 16f32, 2f32, &mut seed) * 0.5f32 + 0.5f32;
        let noise_3 = sample_perlin_fbm_2d(uvw.xy(), 3, 32f32, 2f32, &mut seed) * 0.5f32 + 0.5f32;

        *pixel = (noise_0, noise_1, noise_2, noise_3).into();
    });

    noisetex.save_as_image(format!("output/perlinFbmTex_{index}.png"));
}

fn generate_worley_noisetex(index: u32, seed: &u32) {
    let mut noisetex = NoisetexRgba8::new(512, 512, 1);

    let seed = seed.wrapping_add(index);

    noisetex.fill(|info, pixel, pos| {
        let mut seed = seed;
        let uvw = pos.as_vec3() / info.size().as_vec3();

        let noise_0 = 1f32 - sample_worley_noise_2d(uvw.xy(), 4f32, &mut seed).0;
        let noise_1 = 1f32 - sample_worley_fbm_2d(uvw.xy(), 3, 8f32, 2f32, &mut seed).0;
        let noise_2 = 1f32 - sample_worley_fbm_2d(uvw.xy(), 3, 16f32, 2f32, &mut seed).0;
        let noise_3 = 1f32 - sample_worley_fbm_2d(uvw.xy(), 3, 32f32, 2f32, &mut seed).0;

        *pixel = (noise_0, noise_1, noise_2, noise_3).into();
    });

    noisetex.save_as_image(format!("output/worleyNoiseTex_{index}.png"));
}

fn generate_3d_curl_noise(index: u32, seed: &u32) {
    const FREQUENCY: f32 = 4f32;
    const OCTAVES: u32 = 3;
    const LACUNARITY: f32 = 2f32;

    let mut noisetex = NoisetexRgb8::new(32, 32, 32);

    let seed = seed.wrapping_add(index);

    noisetex.fill(|info, pixel, pos| {
        let mut seed = seed;
        let uvw = pos.as_vec3() / info.size().as_vec3();

        let delta = 1f32 / info.size().as_vec3();
        let span = 2f32 * delta;

        let delta_x = glam::Vec3::X * delta;
        let delta_y = glam::Vec3::Y * delta;
        let delta_z = glam::Vec3::Z * delta;

        let dx =
            (sample_vector_field_fbm(uvw + delta_x, OCTAVES, FREQUENCY, LACUNARITY, &mut seed)
                - sample_vector_field_fbm(
                    uvw - delta_x,
                    OCTAVES,
                    FREQUENCY,
                    LACUNARITY,
                    &mut seed,
                ))
                / span;

        let dy =
            (sample_vector_field_fbm(uvw + delta_y, OCTAVES, FREQUENCY, LACUNARITY, &mut seed)
                - sample_vector_field_fbm(
                    uvw - delta_y,
                    OCTAVES,
                    FREQUENCY,
                    LACUNARITY,
                    &mut seed,
                ))
                / span;

        let dz =
            (sample_vector_field_fbm(uvw + delta_z, OCTAVES, FREQUENCY, LACUNARITY, &mut seed)
                - sample_vector_field_fbm(
                    uvw - delta_z,
                    OCTAVES,
                    FREQUENCY,
                    LACUNARITY,
                    &mut seed,
                ))
                / span;

        let curl = ((dy.z - dz.z) * glam::Vec3::X
            + (dz.x - dx.z) * glam::Vec3::Y
            + (dx.y - dy.x) * glam::Vec3::Z)
            * 0.5f32
            + 0.5f32;

        *pixel = (curl.x, curl.y, curl.z).into();
    });

    noisetex.save_as_binary(format!("output/curl3dNoiseTex_{index}.bin"));
    // noisetex.save_as_image(format!("output/curl3dNoiseTex_{index}.png"));
}

fn main() {
    // let seed = 0u32;
    let seed = rand::random::<u32>();

    for i in 0..1 {
        generate_lf_cloud_noisetex(i, &seed);
        // generate_hf_cloud_noisetex(i, &seed);
    }

    // for i in 0..10 {
    //     generate_perlin_fbm_noisetex(i, &seed);
    //     generate_perlin_noisetex(i, &seed);
    //     generate_worley_noisetex(i, &seed);
    // }

    // for i in 0..1 {
    //     generate_3d_curl_noise(i, &seed);
    // }

    println!("Fin!");
}
