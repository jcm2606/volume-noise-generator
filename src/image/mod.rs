use std::path::Path;

use image::RgbaImage;
use rayon::prelude::{
    IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator, ParallelSlice,
};

use crate::image::pixel::Pixel;

pub mod pixel;

pub struct Image<T>
where
    T: Pixel,
{
    width: usize,
    height: usize,
    depth: usize,
    pixels: Vec<T>,
}
impl<T> Image<T>
where
    T: Pixel,
{
    pub fn new(width: usize, height: usize, depth: usize) -> Self {
        Self {
            width,
            height,
            depth,
            pixels: vec![T::default(); width * height * depth],
        }
    }

    pub fn fill<F>(&mut self, kernel: F)
    where
        F: Fn(glam::Vec3, &mut T) + Send + Sync,
    {
        self.pixels
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, pixel)| {
                let uvw = index_to_uvw(index, self.width, self.height, self.depth);

                kernel(uvw, pixel)
            });
    }

    pub fn save_as_binary<P>(&self, path: P)
    where
        P: AsRef<Path>,
    {
        let mut buffer = Vec::<u8>::with_capacity(self.pixels.len() * size_of::<T>());

        for pixel in self.pixels.iter() {
            pixel.write_to_buffer(&mut buffer);
        }

        std::fs::write(path, &buffer).unwrap();
    }

    pub fn save_as_rgba_png<P>(&self, path: P)
    where
        P: AsRef<Path>,
    {
        let mut png = RgbaImage::new(self.width as u32, self.height as u32);

        for (index, pixel) in self.pixels.iter().enumerate() {
            let (x, y, _) = index_to_xyz(index, self.width, self.height);

            pixel.write_to_rgba8_png(x as u32, y as u32, &mut png);
        }

        png.save(path).unwrap();
    }

    pub fn pixels(&self) -> &[T] {
        &self.pixels
    }
}

pub fn index_to_uvw(index: usize, width: usize, height: usize, depth: usize) -> glam::Vec3 {
    let slice_size = width * height;
    let remainder = index % slice_size;

    let x = remainder % width;
    let y = remainder / width;
    let z = index / slice_size;

    glam::vec3(x as f32, y as f32, z as f32) / glam::vec3(width as f32, height as f32, depth as f32)
}

pub fn index_to_xyz(index: usize, width: usize, height: usize) -> (usize, usize, usize) {
    let slice_size = width * height;
    let remainder = index % slice_size;

    let x = remainder % width;
    let y = remainder / width;
    let z = index / slice_size;

    (x, y, z)
}
