use ::image::{Rgb, RgbImage};

use crate::image::pixel::PixelRgba8;
use crate::image::Image;
use crate::noise::perlin::sample_perlin_noise;

mod image;
mod noise;
mod util;

fn main() {
    let mut img: Image<PixelRgba8> = Image::new(256, 256, 1);

    img.fill(|uvw, pixel| {
        let perlin = sample_perlin_noise(uvw * 10f32, 10f32) * 0.5f32 + 0.5f32;
        let perlin = (perlin * 255f32) as u8;

        pixel.r = perlin;
        pixel.g = perlin;
        pixel.b = perlin;
        pixel.a = 255;
    });

    img.save_as_rgba_png("output/test.png");

    println!("Fin!");
}
