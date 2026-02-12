use std::f32;

use bon::Builder;

use crate::random::hash::pcg_11;
use crate::samplers::{NoiseSampler, NoiseSamplerState, Smoothing};

#[derive(Debug, Builder)]
pub struct FbmSampler<S: NoiseSampler<f32>> {
    sampler: S,
    octaves: u32,
    #[builder(default = 2.0)]
    decay: f32,
    #[builder(default = 2.0)]
    lacunarity: f32,
    #[builder(default = Smoothing::None)]
    smoothing: Smoothing
}
impl<S: NoiseSampler<f32>> NoiseSampler<f32> for FbmSampler<S> {
    fn sample_2d(&mut self, uv: glam::Vec2) -> f32 {
        let mut noise_sum: f32 = 0.0;
        let mut amplitude_sum: f32 = 0.0;

        for octave in 0..self.octaves {
            let sample = self.sampler.sample_2d(uv);
            let amplitude = (1.0 / self.decay).powf(octave as f32);

            noise_sum += sample * amplitude;
            amplitude_sum += amplitude;

            self.sampler.set_frequency(self.sampler.get_frequency() * self.lacunarity);
            self.sampler.set_seed(pcg_11(self.sampler.get_seed()));
        }

        self.smoothing.smooth((noise_sum / amplitude_sum.max(f32::EPSILON)).clamp(0.0, 1.0))
    }

    fn sample_3d(&mut self, uvw: glam::Vec3) -> f32 {
        let mut noise_sum: f32 = 0.0;
        let mut amplitude_sum: f32 = 0.0;

        for octave in 0..self.octaves {
            let sample = self.sampler.sample_3d(uvw);
            let amplitude = (1.0 / self.decay).powf(octave as f32);

            noise_sum += sample * amplitude;
            amplitude_sum += amplitude;

            self.sampler.set_frequency(self.sampler.get_frequency() * self.lacunarity);
            self.sampler.set_seed(pcg_11(self.sampler.get_seed()));
        }

        self.smoothing.smooth((noise_sum / amplitude_sum.max(f32::EPSILON)).clamp(0.0, 1.0))
    }
}
impl<S: NoiseSampler<f32>> NoiseSamplerState for FbmSampler<S> {
    fn get_frequency(&self) -> f32 {
        self.sampler.get_frequency()
    }

    fn get_seed(&self) -> u32 {
        self.sampler.get_seed()
    }

    fn set_frequency(&mut self, new_frequency: f32) {
        self.sampler.set_frequency(new_frequency)
    }

    fn set_seed(&mut self, new_seed: u32) {
        self.sampler.set_seed(new_seed)
    }
}