use crate::noisetex::{Noisetex, NoisetexRgba8};
use crate::samplers::perlin::sample_perlin_fbm;
use crate::samplers::worley::sample_worley_fbm3;
use crate::util::{remap, remap_clamp};

mod noisetex;
mod samplers;
mod util;

fn main() {
    let mut noisetex = NoisetexRgba8::new(128, 128, 1);

    noisetex.fill(|uvw, pixel| {
        let mut seed = 4f32;

        let perlin_worley = {
            let perlin_fbm = sample_perlin_fbm(uvw, 8f32, 3, seed);

            seed *= 2f32;
            let worley_fbm = sample_worley_fbm3(uvw, 6f32, seed);

            remap_clamp(perlin_fbm, 0f32, 1f32, worley_fbm, 1f32)
        };
        let perlin_worley = (perlin_worley * 255f32) as u8;

        pixel.r = perlin_worley;
        pixel.g = perlin_worley;
        pixel.b = perlin_worley;
        pixel.a = 255;
    });

    noisetex.save_as_image("output/lfCloudNoiseTex.png");

    println!("Fin!");
}
