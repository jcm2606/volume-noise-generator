use core::f32;
use std::sync::atomic::Ordering;

use atomic_float::AtomicF32;
use glam::Vec3Swizzles;
use rayon::vec;

use crate::noisetex::{NoisetexR8, NoisetexRg8, NoisetexRgb16, NoisetexRgb8, NoisetexRgba8};
use crate::samplers::alligator::AlligatorSampler;
use crate::samplers::fbm::{FbmSampler, FbmSamplerBuilder};
use crate::samplers::perlin::{PerlinMode, PerlinSampler};
use crate::samplers::vector_field::{CurlSampler, VectorFieldFbmSampler, VectorFieldSampler};
use crate::samplers::worley::{WorleyMode, WorleySampler};
use crate::samplers::{NoiseSampler, Smoothing};
use crate::util::{
    clamped_map, map, mix, remap, remap_clamp, smoothstep, CoordWrapping, MappingFn, Normalized,
    SmoothMappingFn, SmoothingFn,
};

mod hash;
mod noisetex;
mod random;
mod samplers;
mod util;

fn generate_lf_cloudmap_noisetex(index: u32, seed: &u32) {
    let mut noisetex = NoisetexRgba8::new(512, 512, 1);

    let seed = seed.wrapping_add(index);

    let r_norm = Normalized::new();
    let g_norm = Normalized::new();
    let b_norm = Normalized::new();
    let a_norm = Normalized::new();

    noisetex.fill(|info, pixel, pos| {
        let uvw = pos.as_vec3() / info.size().as_vec3();

        pixel.r = r_norm.update_with(
            FbmSampler::builder()
                .sampler(PerlinSampler::builder().frequency(4.0).seed(seed).build())
                .octaves(4)
                .build()
                .sample_2d(uvw.xy()),
        );

        pixel.g = g_norm.update_with(
            FbmSampler::builder()
                .sampler(
                    WorleySampler::builder()
                        .frequency(4.0)
                        .seed(seed + 10)
                        .build(),
                )
                .octaves(4)
                .build()
                .sample_2d(uvw.xy()),
        );

        pixel.b = b_norm.update_with(
            FbmSampler::builder()
                .sampler(
                    PerlinSampler::builder()
                        .frequency(44.0)
                        .seed(seed)
                        .build(),
                )
                .octaves(3)
                .decay(2.25)
                .build()
                .sample_2d(uvw.xy()),
        );

        pixel.a = a_norm.update_with(
            FbmSampler::builder()
                .sampler(
                    WorleySampler::builder()
                        .frequency(16.0)
                        .seed(seed + 30)
                        .build(),
                )
                .octaves(3)
                .decay(2.75)
                .build()
                .sample_2d(uvw.xy()),
        );
    });

    noisetex.fill(|info, pixel, pos| {
        pixel.r = pixel.r.map(r_norm.min(), r_norm.max());
        pixel.g = pixel.g.map(g_norm.min(), g_norm.max());
        pixel.b = pixel.b.map(b_norm.min(), b_norm.max());
        pixel.a = pixel.a.map(a_norm.min(), a_norm.max());
    });

    noisetex.save_as_image(format!("output/cloudMap/cloudMapLfNoiseTex_{index}.png"));
}

fn generate_hf_cloudmap_noisetex(index: u32, seed: &u32) {
    let mut noisetex = NoisetexRgba8::new(512, 512, 1);

    let seed = seed.wrapping_add(index);

    let r_norm = Normalized::new();
    let g_norm = Normalized::new();
    let b_norm = Normalized::new();
    let a_norm = Normalized::new();

    noisetex.fill(|info, pixel, pos| {
        let uvw = pos.as_vec3() / (info.size().x as f32);

        pixel.r = r_norm.update_with(
            FbmSampler::builder()
                .sampler(
                    WorleySampler::builder()
                        .frequency(18.0)
                        .seed(seed + 20)
                        .smoothing(Smoothing::Cubic)
                        .build(),
                )
                .octaves(2)
                .decay(2.75)
                .build()
                .sample_2d(uvw.xy()),
        );

        pixel.g = g_norm.update_with({
            let hf_alligator_noise = FbmSampler::builder()
                .sampler(
                    AlligatorSampler::builder()
                        .frequency(26.0)
                        .seed(seed + 40)
                        .build(),
                )
                .octaves(3)
                .build()
                .sample_2d(uvw.xy())
                .remap(0.0, 1.0, 0.2, 0.0);

            1.0 - (1.0
                - FbmSampler::builder()
                    .sampler(
                        AlligatorSampler::builder()
                            .frequency(11.0)
                            .seed(seed + 20)
                            .build(),
                    )
                    .octaves(4)
                    .build()
                    .sample_2d(uvw.xy()))
            .powf(1.0)
            .max(hf_alligator_noise)
        });

        pixel.b = b_norm.update_with(0.0);

        pixel.a = a_norm.update_with(0.0);
    });

    noisetex.fill(|info, pixel, pos| {
        pixel.r = pixel.r.map(r_norm.min(), r_norm.max());
        pixel.g = pixel.g.map(g_norm.min(), g_norm.max());
        pixel.b = pixel.b.map(b_norm.min(), b_norm.max());
        pixel.a = pixel.a.map(a_norm.min(), a_norm.max());
    });

    noisetex.save_as_image(format!("output/cloudMap/cloudMapHfNoiseTex_{index}.png"));
}

fn generate_new_noise_composite_texture(index: u32, seed: &u32) {
    let mut noisetex = NoisetexRgba8::new(128, 128, 128);

    let r_norm = Normalized::new();
    let g_norm = Normalized::new();
    let b_norm = Normalized::new();
    let a_norm = Normalized::new();

    noisetex.fill(|info, pixel, pos| {
        let size = glam::Vec3::splat(info.size().x as f32);
        let uvw = pos.as_vec3() / size;

        pixel.r = r_norm.update_with({
            let curl = {
                let vector_field_fbm_sampler = VectorFieldFbmSampler::builder()
                    .sampler(
                        VectorFieldSampler::builder()
                            .frequency(3.0)
                            .seed(*seed)
                            .build(),
                    )
                    .octaves(3)
                    .restore_original_state(true)
                    .build();

                CurlSampler::builder()
                    .sampler(vector_field_fbm_sampler)
                    .size(size)
                    .build()
                    .sample_3d(uvw)
            };

            (1.0 - FbmSampler::builder()
                .sampler(
                    AlligatorSampler::builder()
                        .frequency(3.0)
                        .seed(seed + 10)
                        .build(),
                )
                .octaves(4)
                .build()
                .sample_3d((uvw + curl * 1.0).wrap_coord(size)))
            .powf(1.0)
            .quintic_smooth()
        });

        pixel.g = g_norm.update_with({
            let curl = {
                let vector_field_fbm_sampler = VectorFieldFbmSampler::builder()
                    .sampler(
                        VectorFieldSampler::builder()
                            .frequency(5.0)
                            .seed(seed + 20)
                            .build(),
                    )
                    .octaves(3)
                    .restore_original_state(true)
                    .build();

                CurlSampler::builder()
                    .sampler(vector_field_fbm_sampler)
                    .size(size)
                    .build()
                    .sample_3d(uvw)
            };

            (1.0 - FbmSampler::builder()
                .sampler(
                    AlligatorSampler::builder()
                        .frequency(7.0)
                        .seed(seed + 30)
                        .build(),
                )
                .octaves(3)
                .decay(2.25)
                .build()
                .sample_3d((uvw + curl * 1.0).wrap_coord(size)))
            .powf(2.0)
            .quintic_smooth()
        });

        pixel.b = b_norm.update_with({
            (1.0 - (1.0
                - FbmSampler::builder()
                    .sampler(
                        AlligatorSampler::builder()
                            .frequency(4.0)
                            .seed(seed + 50)
                            .build(),
                    )
                    .octaves(4)
                    .decay(2.5)
                    .build()
                    .sample_3d(uvw))
            .powf(4.0))
            .cubic_smooth()
        });

        pixel.a = a_norm.update_with({
            (1.0 - (1.0
                - FbmSampler::builder()
                    .sampler(
                        AlligatorSampler::builder()
                            .frequency(11.0)
                            .seed(seed + 70)
                            .build(),
                    )
                    .octaves(3)
                    .decay(2.25)
                    .build()
                    .sample_3d(uvw))
            .powf(4.0))
            .cubic_smooth()
        });
    });

    noisetex.fill(|_info, pixel, _pos| {
        pixel.r = pixel.r.map(r_norm.min(), r_norm.max());
        pixel.g = pixel.g.map(g_norm.min(), g_norm.max());
        pixel.b = pixel.b.map(b_norm.min(), b_norm.max());
        pixel.a = pixel.a.map(a_norm.min(), a_norm.max());
    });

    // noisetex.save_as_image(format!(
    //     "output/noiseComposite/volCloudNoiseTex_{index}.png"
    // ));
    noisetex.save_as_binary(format!(
        "output/noiseComposite/volCloudNoiseTex_{index}.bin"
    ));
}

fn main() {
    let seed = 0u32;
    // let seed = rand::random::<u32>();

    for i in 0..1 {
        // generate_lf_cloud_noisetex(i, &seed);
        // generate_hf_cloud_noisetex(i, &seed);
        generate_new_noise_composite_texture(i, &seed);
    }

    for i in 0..10 {
        generate_lf_cloudmap_noisetex(i, &seed);
        generate_hf_cloudmap_noisetex(i, &seed);
    }

    for i in 0..1 {
        // generate_3d_curl_noise(i, &seed);
    }

    println!("Fin!");
}
