use std::path::Path;

use rayon::prelude::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

pub type NoisetexRgba8 = Noisetex<Rgba8>;
pub type NoisetexRgb8 = Noisetex<Rgb8>;
pub type NoisetexRg8 = Noisetex<Rg8>;
pub type NoisetexR8 = Noisetex<R8>;

#[derive(Debug, Clone)]
pub struct NoisetexInfo {
    width: u32,
    height: u32,
    depth: u32,
}
impl NoisetexInfo {
    pub fn size(&self) -> glam::UVec3 {
        glam::uvec3(self.width, self.height, self.depth)
    }
}

pub struct Noisetex<P>
where
    P: PixelType,
{
    info: NoisetexInfo,
    pixels: Vec<P>,
}
impl<P> Noisetex<P>
where
    P: PixelType,
{
    pub fn new(width: u32, height: u32, depth: u32) -> Self {
        let info = NoisetexInfo {
            width,
            height,
            depth,
        };

        Self {
            info,
            pixels: vec![P::default(); (width * height * depth) as usize],
        }
    }

    pub fn fill<F>(&mut self, function: F)
    where
        F: Fn(&NoisetexInfo, &mut P, glam::UVec3) + Send + Sync,
    {
        self.pixels
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, pixel)| {
                let xyz = Self::index_to_coord(index as u32, self.info.width, self.info.height);
                let xyz = glam::uvec3(xyz.0, xyz.1, xyz.2);

                function(&self.info, pixel, xyz);
            });
    }

    pub fn save_as_binary<Pt>(&self, path: Pt)
    where
        Pt: AsRef<Path>,
    {
        let mut buffer = Vec::<u8>::with_capacity(self.pixels.len() * std::mem::size_of::<P>());

        for pixel in self.pixels.iter() {
            pixel.write_to_buffer(&mut buffer);
        }

        std::fs::write(path, &buffer).unwrap();
    }

    pub fn save_as_image<Pt>(&self, path: Pt)
    where
        Pt: AsRef<Path>,
    {
        let mut img: P::ImageType = P::create_image(self.info.width, self.info.height);

        for (index, pixel) in self.pixels.iter().enumerate() {
            let (x, y, _) = Self::index_to_coord(index as u32, self.info.width, self.info.height);

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
#[derive(Debug, Clone)]
pub struct Rgba8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl Default for Rgba8 {
    fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }
}
impl From<(f32, f32, f32, f32)> for Rgba8 {
    fn from(value: (f32, f32, f32, f32)) -> Self {
        Self {
            r: (value.0 * 255f32) as u8,
            g: (value.1 * 255f32) as u8,
            b: (value.2 * 255f32) as u8,
            a: (value.3 * 255f32) as u8,
        }
    }
}
impl From<f32> for Rgba8 {
    fn from(value: f32) -> Self {
        Self {
            r: (value * 255f32) as u8,
            g: (value * 255f32) as u8,
            b: (value * 255f32) as u8,
            a: 255u8,
        }
    }
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
impl From<(f32, f32, f32)> for Rgb8 {
    fn from(value: (f32, f32, f32)) -> Self {
        Self {
            r: (value.0 * 255f32) as u8,
            g: (value.1 * 255f32) as u8,
            b: (value.2 * 255f32) as u8,
        }
    }
}
impl From<f32> for Rgb8 {
    fn from(value: f32) -> Self {
        Self {
            r: (value * 255f32) as u8,
            g: (value * 255f32) as u8,
            b: (value * 255f32) as u8,
        }
    }
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

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Rg8 {
    pub r: u8,
    pub g: u8,
}
impl From<f32> for Rg8 {
    fn from(value: f32) -> Self {
        Self {
            r: (value * 255f32) as u8,
            g: (value * 255f32) as u8,
        }
    }
}
impl PixelType for Rg8 {
    type ImageType = image::RgbImage;
    type ImagePixelType = image::Rgb<u8>;

    fn create_image(width: u32, height: u32) -> Self::ImageType {
        println!("warning: encoded images for RG8 noise textures are actually saved as RGB8");
        image::RgbImage::new(width, height)
    }

    fn save_image<P>(path: P, img: Self::ImageType)
    where
        P: AsRef<Path>,
    {
        img.save(path).unwrap();
    }

    fn write_to_image(&self, x: u32, y: u32, img: &mut Self::ImageType) {
        img.put_pixel(x, y, image::Rgb([self.r, self.g, 0u8]));
    }

    fn write_to_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.r);
    }
}

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct R8 {
    pub r: u8,
}
impl From<f32> for R8 {
    fn from(value: f32) -> Self {
        Self {
            r: (value * 255f32) as u8,
        }
    }
}
impl PixelType for R8 {
    type ImageType = image::RgbImage;
    type ImagePixelType = image::Rgb<u8>;

    fn create_image(width: u32, height: u32) -> Self::ImageType {
        println!("warning: encoded images for R8 noise textures are actually saved as RGB8");
        image::RgbImage::new(width, height)
    }

    fn save_image<P>(path: P, img: Self::ImageType)
    where
        P: AsRef<Path>,
    {
        img.save(path).unwrap();
    }

    fn write_to_image(&self, x: u32, y: u32, img: &mut Self::ImageType) {
        img.put_pixel(x, y, image::Rgb([self.r, self.r, self.r]));
    }

    fn write_to_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.r);
    }
}
