use std::path::Path;

use rayon::prelude::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

pub type NoisetexRgba8 = Noisetex<Rgba8>;
pub type NoisetexRgb8 = Noisetex<Rgb8>;

pub struct Noisetex<P>
where
    P: PixelType,
{
    width: u32,
    height: u32,
    depth: u32,
    pixels: Vec<P>,
}
impl<P> Noisetex<P>
where
    P: PixelType,
{
    pub fn new(width: u32, height: u32, depth: u32) -> Self {
        Self {
            width,
            height,
            depth,
            pixels: vec![P::default(); (width * height * depth) as usize],
        }
    }

    pub fn fill<F>(&mut self, function: F)
    where
        F: Fn(glam::Vec3, &mut P) + Send + Sync,
    {
        self.pixels
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, pixel)| {
                let (x, y, z) = Self::index_to_coord(index as u32, self.width, self.height);
                let uvw = glam::vec3(x as f32, y as f32, z as f32)
                    / glam::vec3(self.width as f32, self.height as f32, self.depth as f32);

                function(uvw, pixel);
            });
    }

    pub fn save_as_binary<Pt>(&self, path: Pt) -> std::io::Result<()>
    where
        Pt: AsRef<Path>,
    {
        let mut buffer = Vec::<u8>::with_capacity(self.pixels.len() * std::mem::size_of::<P>());

        for pixel in self.pixels.iter() {
            pixel.write_to_buffer(&mut buffer);
        }

        std::fs::write(path, &buffer)
    }

    pub fn save_as_image<Pt>(&self, path: Pt)
    where
        Pt: AsRef<Path>,
    {
        let mut img: P::ImageType = P::create_image(self.width, self.height);

        for (index, pixel) in self.pixels.iter().enumerate() {
            let (x, y, _) = Self::index_to_coord(index as u32, self.width, self.height);

            pixel.write_to_image(x, y, &mut img);
        }

        P::save_image(path, img);
    }

    fn index_to_coord(index: u32, width: u32, height: u32) -> (u32, u32, u32) {
        let slice_index = width * height;
        let remainder = index % slice_index;

        let x = remainder % width;
        let y = remainder / width;
        let z = index / slice_index;

        (x, y, z)
    }
}

pub trait PixelType: Sized + Send + Sync + Clone + Default {
    type ImageType: image::GenericImage + image::GenericImageView;
    type ImagePixelType: image::Pixel;

    fn create_image(width: u32, height: u32) -> Self::ImageType;

    fn save_image<P>(path: P, img: Self::ImageType)
    where
        P: AsRef<Path>;

    fn write_to_image(&self, x: u32, y: u32, img: &mut Self::ImageType);

    fn write_to_buffer(&self, buffer: &mut Vec<u8>);
}

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Rgba8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl PixelType for Rgba8 {
    type ImageType = image::RgbaImage;
    type ImagePixelType = image::Rgba<u8>;

    fn create_image(width: u32, height: u32) -> Self::ImageType {
        image::RgbaImage::new(width, height)
    }

    fn save_image<P>(path: P, img: Self::ImageType)
    where
        P: AsRef<Path>,
    {
        img.save(path).unwrap();
    }

    fn write_to_image(&self, x: u32, y: u32, img: &mut Self::ImageType) {
        img.put_pixel(x, y, image::Rgba([self.r, self.g, self.b, self.a]));
    }

    fn write_to_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.r);
        buffer.push(self.g);
        buffer.push(self.b);
        buffer.push(self.a);
    }
}

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Rgb8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
impl PixelType for Rgb8 {
    type ImageType = image::RgbImage;
    type ImagePixelType = image::Rgb<u8>;

    fn create_image(width: u32, height: u32) -> Self::ImageType {
        image::RgbImage::new(width, height)
    }

    fn save_image<P>(path: P, img: Self::ImageType)
    where
        P: AsRef<Path>,
    {
        img.save(path).unwrap();
    }

    fn write_to_image(&self, x: u32, y: u32, img: &mut Self::ImageType) {
        img.put_pixel(x, y, image::Rgb([self.r, self.g, self.b]));
    }

    fn write_to_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.r);
        buffer.push(self.g);
        buffer.push(self.b);
    }
}
