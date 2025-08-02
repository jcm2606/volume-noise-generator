use crate::noisetex::{Noisetex, NoisetexRgba8};
use crate::samplers::perlin::sample_perlin_fbm;
use crate::samplers::worley::sample_worley_fbm3;
use crate::util::{remap, remap_clamp};

mod noisetex;
mod samplers;
mod util;

fn main() {
    let mut noisetex = NoisetexRgba8::new(128, 128, 128);

    noisetex.fill(|uvw, pixel| {
        let mut seed = 4f32;

        let perlin_worley = {
            let perlin_fbm = sample_perlin_fbm(uvw, 8f32, 3, seed);

            seed *= 2f32;
            let worley_fbm = sample_worley_fbm3(uvw, 8f32, seed);

            remap(perlin_fbm, 0f32, 1f32, worley_fbm, 1f32)
            // remap(perlin_fbm, -worley_fbm, 1f32, 0f32, 1f32)
        };

        seed *= 2f32;
        let worley_fbm_0 = sample_worley_fbm3(uvw, 12f32, seed);

        seed *= 2f32;
        let worley_fbm_1 = sample_worley_fbm3(uvw, 16f32, seed);

        seed *= 2f32;
        let worley_fbm_2 = sample_worley_fbm3(uvw, 20f32, seed);

        pixel.r = (perlin_worley * 255f32) as u8;
        pixel.g = (worley_fbm_0 * 255f32) as u8;
        pixel.b = (worley_fbm_1 * 255f32) as u8;
        pixel.a = (worley_fbm_2 * 255f32) as u8;
    });

    // noisetex.save_as_image("output/lfCloudNoiseTex.png");
    noisetex
        .save_as_binary("output/lfCloudNoiseTex.bin")
        .unwrap();

    println!("Fin!");
}
