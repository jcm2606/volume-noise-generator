use crate::hash::{pcg_3d, per_pixel_seed, rand_vector_3d};
use crate::noisetex::{NoisetexRgb8, NoisetexRgba8};
use crate::samplers::perlin::{sample_perlin_fbm, sample_perlin_noise};
use crate::samplers::worley::{sample_worley_fbm, sample_worley_noise};
use crate::util::remap;

mod hash;
mod noisetex;
mod samplers;
mod util;

fn generate_lf_cloud_noisetex(seed: u32) {
    let mut noisetex = NoisetexRgba8::new(128, 128, 128);

    noisetex.fill(|info, pixel, pos| {
        let uvw = pos.as_vec3() / info.size().as_vec3();

        let perlin_worley = {
            let perlin: f32 = sample_perlin_fbm(uvw, 4, 12f32, 2f32, seed) * 0.5f32 + 0.5f32;
            let worley: f32 = sample_worley_fbm(uvw, 4, 4f32, 2f32, seed + 1);

            //  Closely matches the figures in the Nubis slides.
            remap(perlin, 0f32, 1f32, 0f32, worley * 2f32)
        };

        let worley_fbm_1 = sample_worley_fbm(uvw, 3, 8f32, 2f32, seed + 2);
        let worley_fbm_2 = sample_worley_fbm(uvw, 3, 16f32, 2f32, seed + 3);
        let worley_fbm_3 = sample_worley_fbm(uvw, 3, 32f32, 2f32, seed + 4);

        pixel.r = (perlin_worley * 255f32) as u8;
        pixel.g = (worley_fbm_1 * 255f32) as u8;
        pixel.b = (worley_fbm_2 * 255f32) as u8;
        pixel.a = (worley_fbm_3 * 255f32) as u8;
    });

    // noisetex.save_as_image("output/test.png");
    noisetex.save_as_binary("output/lfCloudNoiseTex.bin");
}

fn generate_cloud_map_noisetex(seed: u32) {
    let mut noisetex = NoisetexRgba8::new(256, 256, 1);

    noisetex.fill(|info, pixel, pos| {
        let uvw = pos.as_vec3() / info.size().as_vec3();

        let perlin_worley_0 = {
            let perlin: f32 = sample_perlin_fbm(uvw, 5, 4f32, 2f32, seed + 1) * 0.5f32 + 0.5f32;
            let worley: f32 = sample_worley_fbm(uvw, 5, 1f32, 2f32, seed + 2);

            //  Closely matches the figures in the Nubis slides.
            remap(perlin, 0f32, 1f32, 0f32, worley * 2f32)
        };

        let perlin_worley_1 = {
            let perlin: f32 = sample_perlin_fbm(uvw, 5, 8f32, 2f32, seed + 1) * 0.5f32 + 0.5f32;
            let worley: f32 = sample_worley_fbm(uvw, 5, 2f32, 2f32, seed + 2);

            //  Closely matches the figures in the Nubis slides.
            remap(perlin, 0f32, 1f32, 0f32, worley * 2f32)
        };

        let perlin_fbm_1 = sample_perlin_fbm(uvw, 3, 12f32, 2f32, seed + 3) * 0.5f32 + 0.5f32;
        let perlin_fbm_2 = sample_perlin_fbm(uvw, 3, 18f32, 2f32, seed + 4) * 0.5f32 + 0.5f32;

        pixel.r = (perlin_worley_0 * 255f32) as u8;
        pixel.g = (perlin_worley_1 * 255f32) as u8;
        pixel.b = (perlin_fbm_1 * 255f32) as u8;
        pixel.a = (perlin_fbm_2 * 255f32) as u8;
    });

    noisetex.save_as_image(format!("output/cloudMapNoiseTex_{seed}.png"));
}

fn main() {
    // generate_lf_cloud_noisetex(0);
    for i in 10..20 {
        generate_cloud_map_noisetex(i);
    }

    println!("Fin!");
}
