use noise::core::perlin;

use crate::noisetex::NoisetexRgba8;
use crate::samplers::perlin::PerlinNoiseSampler;
use crate::samplers::worley::WorleyNoiseSampler;
use crate::samplers::{FbmSampler, SamplerState};
use crate::util::remap;

mod noisetex;
mod samplers;
mod util;

fn main() {
    let mut noisetex = NoisetexRgba8::new(128, 128, 1);

    noisetex.fill(|uvw, pixel| {
        let mut state = SamplerState::default();

        let perlin_worley = {
            let perlin_fbm = FbmSampler::builder()
                .num_octaves(3)
                .frequency(8f32)
                .build()
                .sample::<PerlinNoiseSampler>(uvw, &mut state);

            let worley_fbm = FbmSampler::builder()
                .num_octaves(3)
                .frequency(8f32)
                .build()
                .sample::<WorleyNoiseSampler>(uvw, &mut state);

            remap(perlin_fbm, 0f32, 1f32, worley_fbm, 1f32)
            // remap(perlin_fbm, -worley_fbm, 1f32, 0f32, 1f32)
        };

        let worley_fbm_0 = FbmSampler::builder()
            .num_octaves(3)
            .frequency(16f32)
            .build()
            .sample::<WorleyNoiseSampler>(uvw, &mut state);

        let worley_fbm_1 = FbmSampler::builder()
            .num_octaves(3)
            .frequency(22f32)
            .build()
            .sample::<WorleyNoiseSampler>(uvw, &mut state);

        let worley_fbm_2 = FbmSampler::builder()
            .num_octaves(3)
            .frequency(28f32)
            .build()
            .sample::<WorleyNoiseSampler>(uvw, &mut state);

        pixel.r = (perlin_worley * 255f32) as u8;
        pixel.g = (worley_fbm_0 * 255f32) as u8;
        pixel.b = (worley_fbm_1 * 255f32) as u8;
        pixel.a = (worley_fbm_2 * 255f32) as u8;
    });

    noisetex.save_as_image("output/lfCloudNoiseTex.png");
    // noisetex
    //     .save_as_binary("output/lfCloudNoiseTex.bin")
    //     .unwrap();

    println!("Fin!");
}
