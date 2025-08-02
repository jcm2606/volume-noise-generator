//  1. seed must be global
//  2. samplers must be stateless
//  3. fbm sampler must be composable

use bon::Builder;
use glam::Vec3Swizzles;

pub mod perlin;
pub mod worley;

//  https://www.shadertoy.com/view/3dVXDc

pub fn hash33(mut p: glam::Vec3) -> glam::Vec3 {
    p = glam::Vec3::fract(p * glam::vec3(0.1031f32, 0.1030f32, 0.0973f32));
    p += glam::Vec3::dot(p, p.yxz() + 33.33f32);
    glam::Vec3::fract((p.xxy() + p.yxx()) * p.zyx())
}

pub fn hash13(p: f32) -> glam::Vec3 {
    hash33(glam::Vec3::splat(p))
}

#[derive(Debug, Clone)]
pub struct SamplerState {
    pub seed: f32,
    pub frequency: f32,
}
impl SamplerState {
    pub fn next_seed(&mut self) {
        self.seed += 1f32;
    }
}
impl Default for SamplerState {
    fn default() -> Self {
        Self {
            seed: 0f32,
            frequency: 1f32,
        }
    }
}

pub trait NoiseSampler {
    fn sample(uvw: glam::Vec3, state: &SamplerState) -> f32;
}

#[derive(Debug, Clone, Builder)]
pub struct FbmSampler {
    num_octaves: u32,
    frequency: f32,
    #[builder(default = 1f32)]
    amplitude: f32,
    #[builder(default = 2f32)]
    frequency_mult: f32,
    #[builder(default = 0.5f32)]
    amplitude_mult: f32,
}
impl FbmSampler {
    pub fn sample<T>(&mut self, uvw: glam::Vec3, state: &mut SamplerState) -> f32
    where
        T: NoiseSampler,
    {
        let previous_frequency = state.frequency;
        state.frequency = self.frequency;
        state.next_seed();

        let mut sum = 0f32;
        let mut amplitude_sum = 0f32;

        for _octave in 0..self.num_octaves {
            let noise = T::sample(uvw, state);

            sum += noise * self.amplitude;
            amplitude_sum += self.amplitude;

            state.frequency *= self.frequency_mult;
            self.amplitude *= self.amplitude_mult;
            state.next_seed();
        }

        state.frequency = previous_frequency;
        sum / amplitude_sum
    }
}
