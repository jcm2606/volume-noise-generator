use noise::core::perlin;

use crate::noisetex::{NoisetexRgb8, NoisetexRgba8};
use crate::samplers::perlin::{Perlin2dNoiseSampler, Perlin3dNoiseSampler};
use crate::samplers::worley::WorleyNoiseSampler;
use crate::samplers::{FbmSampler, NoiseSampler, SamplerState};
use crate::util::remap;

mod noisetex;
mod samplers;
mod util;

fn generate_lf_cloud_noisetex() {
    let mut noisetex = NoisetexRgba8::new(128, 128, 128);

    noisetex.fill(|uvw, pixel| {
        let mut state = SamplerState::default();

        let perlin_worley = {
            let perlin_fbm = FbmSampler::builder()
                .num_octaves(3)
                .frequency(8f32)
                .build()
                .sample::<Perlin3dNoiseSampler>(uvw, &mut state);

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

    // noisetex.save_as_image("output/lfCloudNoiseTex.png");
    noisetex
        .save_as_binary("output/lfCloudNoiseTex.bin")
        .unwrap();
}

fn generate_2d_perlin_noisetex() {
    let mut noisetex = NoisetexRgba8::new(256, 256, 1);

    noisetex.fill(|uvw, pixel| {
        let mut state = SamplerState::default();
        state.frequency = 16f32;

        // let perlin = Perlin2dNoiseSampler::sample(uvw, &state);

        let perlin_fbm_0 = FbmSampler::builder()
            .num_octaves(3)
            .frequency(8f32)
            .build()
            .sample::<Perlin2dNoiseSampler>(uvw, &mut state);

        let perlin_fbm_1 = FbmSampler::builder()
            .num_octaves(3)
            .frequency(16f32)
            .build()
            .sample::<Perlin2dNoiseSampler>(uvw, &mut state);

        let perlin_fbm_2 = FbmSampler::builder()
            .num_octaves(3)
            .frequency(32f32)
            .build()
            .sample::<Perlin2dNoiseSampler>(uvw, &mut state);

        let perlin_fbm_3 = FbmSampler::builder()
            .num_octaves(3)
            .frequency(64f32)
            .build()
            .sample::<Perlin2dNoiseSampler>(uvw, &mut state);

        pixel.r = (perlin_fbm_0 * 255f32) as u8;
        pixel.g = (perlin_fbm_1 * 255f32) as u8;
        pixel.b = (perlin_fbm_2 * 255f32) as u8;
        pixel.a = (perlin_fbm_3 * 255f32) as u8;
    });

    noisetex.save_as_image("output/perlin2d.png");
}

fn main() {
    generate_lf_cloud_noisetex();
    generate_2d_perlin_noisetex();

    println!("Fin!");
}
