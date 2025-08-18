use crate::noisetex::{NoisetexRgb8, NoisetexRgba8};
use crate::samplers::perlin::{sample_perlin_fbm, sample_perlin_noise};
use crate::samplers::worley::{sample_worley_fbm, sample_worley_noise};
use crate::util::remap;

mod hash;
mod noisetex;
mod samplers;
mod util;

fn generate_lf_cloud_noisetex(index: u32) {
    let mut noisetex = NoisetexRgba8::new(128, 128, 1);

    let seed = index;
    // let seed: u32 = rand::random();
    // let seed = 0u32;

    noisetex.fill(|info, pixel, pos| {
        let mut seed = seed;
        let uvw = pos.as_vec3() / info.size().as_vec3();

        let perlin_worley = {
            let perlin: f32 = sample_perlin_fbm(uvw, 5, 12f32, 2f32, &mut seed);
            let worley: f32 = 1f32 - sample_worley_fbm(uvw, 5, 4f32, 2f32, &mut seed).0;

            remap(perlin, 0.0f32, 1f32, worley, 1f32)
        };

        let worley_fbm_1 = 1f32 - sample_worley_fbm(uvw, 3, 4f32, 2f32, &mut seed).0;
        let worley_fbm_2 = 1f32 - sample_worley_fbm(uvw, 3, 8f32, 2f32, &mut seed).0;
        let worley_fbm_3 = 1f32 - sample_worley_fbm(uvw, 2, 16f32, 2f32, &mut seed).0;

        *pixel = (perlin_worley, worley_fbm_1, worley_fbm_2, worley_fbm_3).into();
    });

    noisetex.save_as_image(format!("output/lfCloudNoiseTex_{index}.png"));
    // noisetex.save_as_binary(format!("output/lfCloudNoiseTex_{index}.bin"));
}

fn generate_hf_cloud_noisetex(index: u32) {
    let mut noisetex = NoisetexRgb8::new(32, 32, 32);

    let seed = index;
    // let seed: u32 = rand::random();
    // let seed = 0u32;

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

fn generate_weathermap_perlin_noisetex(index: u32) {
    let mut noisetex = NoisetexRgba8::new(512, 512, 1);

    let seed = index;
    // let seed: u32 = rand::random();
    // let seed = 0u32;

    noisetex.fill(|info, pixel, pos| {
        let mut seed = seed;
        let uvw = pos.as_vec3() / info.size().as_vec3();

        let noise_0 = sample_perlin_noise(uvw, 4f32, &mut seed) * 0.5f32 + 0.5f32;
        let noise_1 = sample_perlin_fbm(uvw, 2, 8f32, 2f32, &mut seed) * 0.5f32 + 0.5f32;
        let noise_2 = sample_perlin_fbm(uvw, 2, 16f32, 2f32, &mut seed) * 0.5f32 + 0.5f32;
        let noise_3 = sample_perlin_fbm(uvw, 2, 32f32, 2f32, &mut seed) * 0.5f32 + 0.5f32;

        *pixel = (noise_0, noise_1, noise_2, noise_3).into();
    });

    noisetex.save_as_image(format!("output/perlinNoiseTex_{index}.png"));
}

fn generate_weathermap_worley_noisetex(index: u32) {
    let mut noisetex = NoisetexRgba8::new(512, 512, 1);

    let seed = index;
    // let seed: u32 = rand::random();
    // let seed = 0u32;

    noisetex.fill(|info, pixel, pos| {
        let mut seed = seed;
        let uvw = pos.as_vec3() / info.size().as_vec3();

        let noise_0 = 1f32 - sample_worley_noise(uvw, 4f32, &mut seed).0;
        let noise_1 = 1f32 - sample_worley_fbm(uvw, 2, 8f32, 2f32, &mut seed).0;
        let noise_2 = 1f32 - sample_worley_fbm(uvw, 2, 16f32, 2f32, &mut seed).0;
        let noise_3 = 1f32 - sample_worley_fbm(uvw, 2, 32f32, 2f32, &mut seed).0;

        *pixel = (noise_0, noise_1, noise_2, noise_3).into();
    });

    noisetex.save_as_image(format!("output/worleyNoiseTex_{index}.png"));
}

fn main() {
    for i in 0..1 {
        generate_lf_cloud_noisetex(i);
        // generate_hf_cloud_noisetex(i);
    }

    // for i in 0..1 {
    //     generate_weathermap_perlin_noisetex(i);
    //     generate_weathermap_worley_noisetex(i);
    // }

    println!("Fin!");
}
