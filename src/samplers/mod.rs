use crate::util::SmoothingFn;

pub mod perlin;
pub mod vector_field;
pub mod worley;
pub mod alligator;
pub mod fbm;

#[derive(Debug)]
pub enum Smoothing {
    None,
    Cubic,
    Quintic
}
impl Smoothing {
    pub fn smooth<T: SmoothingFn>(&self, value: T) -> T {
        match self {
            Smoothing::None => value,
            Smoothing::Cubic => value.cubic_smooth(),
            Smoothing::Quintic => value.quintic_smooth(),
        }
    }
}

pub trait NoiseSamplerState {
    fn get_frequency(&self) -> f32;

    fn get_seed(&self) -> u32;

    fn set_frequency(&mut self, new_frequency: f32);

    fn set_seed(&mut self, new_seed: u32);
}

pub trait NoiseSampler<T: Sized>: NoiseSamplerState {
    fn sample_2d(&mut self, uv: glam::Vec2) -> T;

    fn sample_3d(&mut self, uvw: glam::Vec3) -> T;
}


