use crate::noisetex::{Noisetex, NoisetexRgba8};
use crate::samplers::perlin::sample_perlin_fbm;

mod noisetex;
mod samplers;
mod util;

fn main() {
    let mut noisetex = NoisetexRgba8::new(64, 64, 1);

    noisetex.fill(|uvw, pixel| {
        let perlin = sample_perlin_fbm(uvw, 8f32, 3);
        let perlin = (perlin * 255f32) as u8;

        pixel.r = perlin;
        pixel.g = perlin;
        pixel.b = perlin;
        pixel.a = 255;
    });

    noisetex.save_as_image("output/test.png");

    println!("Fin!");
}
