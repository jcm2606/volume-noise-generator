use std::fs::File;
use std::io::Write;
use std::os::windows::prelude::FileExt;

use image::{GenericImage, GenericImageView, Rgba, RgbaImage};

pub trait Pixel: Send + Sync + Clone + Default {
    fn write_to_buffer(&self, buffer: &mut Vec<u8>);

    fn write_to_rgba8_png(&self, x: u32, y: u32, png: &mut RgbaImage);
}

#[derive(Debug, Clone, Default)]
pub struct PixelRgba8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl Pixel for PixelRgba8 {
    fn write_to_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(&self.r.to_ne_bytes());
        buffer.extend_from_slice(&self.g.to_ne_bytes());
        buffer.extend_from_slice(&self.b.to_ne_bytes());
        buffer.extend_from_slice(&self.a.to_ne_bytes());
    }

    fn write_to_rgba8_png(&self, x: u32, y: u32, png: &mut RgbaImage) {
        png.put_pixel(x, y, Rgba([self.r, self.g, self.b, self.a]));
    }
}
