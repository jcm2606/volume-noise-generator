use bon::Builder;
use glam::Vec4Swizzles;

use crate::hash::{hash_ivec3, pcg_1d, rand_unit_vector_3d, rand_vector_3d};
use crate::random::hash::{pcg_11, pcg_44};
use crate::random::unit::unit_vector_23;
use crate::samplers::{NoiseSampler, NoiseSamplerState, Smoothing};
use crate::util::{clamp, clamp_vec3, mix, mix_vec3, SmoothingFn};

#[derive(Debug, Builder)]
pub struct VectorFieldSampler {
    pub frequency: f32,
    #[builder(default = 0)]
    pub seed: u32,
    #[builder(default = Smoothing::None)]
    pub smoothing: Smoothing,
    pub bias: Option<glam::Vec3>
}
impl VectorFieldSampler {
    fn get_random_direction_3d(
        &self,
        pi: glam::IVec3,
        pf: glam::Vec3,
        offset: glam::IVec3,
    ) -> glam::Vec3 {
        let cell = (pi + offset).rem_euclid(glam::IVec3::splat(self.frequency as i32));
        let hash = pcg_44(glam::uvec4(
            cell.x.cast_unsigned(),
            cell.y.cast_unsigned(),
            cell.z.cast_unsigned(),
            self.seed,
        ));

        let unit_vector = unit_vector_23(hash.xy());
        match &self.bias {
            Some(bias) => (unit_vector + bias).normalize(),
            None => unit_vector,
        }
    }
}
impl NoiseSampler<glam::Vec3> for VectorFieldSampler {
    fn sample_2d(&mut self, _uv: glam::Vec2) -> glam::Vec3 {
        panic!("2D vector fields are currently not supported")
    }

    fn sample_3d(&mut self, mut uvw: glam::Vec3) -> glam::Vec3 {
        uvw *= self.frequency;

        let pi = uvw.floor().as_ivec3();
        let pf = uvw.fract();

        let f = pf.quintic_smooth();

        let d000 = self.get_random_direction_3d(pi, pf, glam::ivec3(0, 0, 0));
        let d001 = self.get_random_direction_3d(pi, pf, glam::ivec3(0, 0, 1));
        let d010 = self.get_random_direction_3d(pi, pf, glam::ivec3(0, 1, 0));
        let d011 = self.get_random_direction_3d(pi, pf, glam::ivec3(0, 1, 1));
        let d100 = self.get_random_direction_3d(pi, pf, glam::ivec3(1, 0, 0));
        let d101 = self.get_random_direction_3d(pi, pf, glam::ivec3(1, 0, 1));
        let d110 = self.get_random_direction_3d(pi, pf, glam::ivec3(1, 1, 0));
        let d111 = self.get_random_direction_3d(pi, pf, glam::ivec3(1, 1, 1));

        let x00 = mix_vec3(d000, d100, f.x);
        let x01 = mix_vec3(d001, d101, f.x);
        let x10 = mix_vec3(d010, d110, f.x);
        let x11 = mix_vec3(d011, d111, f.x);

        let y0 = mix_vec3(x00, x10, f.y);
        let y1 = mix_vec3(x01, x11, f.y);

        mix_vec3(y0, y1, f.z)
    }
}
impl NoiseSamplerState for VectorFieldSampler {
    fn get_frequency(&self) -> f32 {
        self.frequency
    }

    fn get_seed(&self) -> u32 {
        self.seed
    }

    fn set_frequency(&mut self, new_frequency: f32) {
        self.frequency = new_frequency;
    }

    fn set_seed(&mut self, new_seed: u32) {
        self.seed = new_seed;
    }
}

#[derive(Debug, Builder)]
pub struct VectorFieldFbmSampler<S: NoiseSampler<glam::Vec3>> {
    sampler: S,
    octaves: u32,
    #[builder(default = 2.0)]
    persistence: f32,
    #[builder(default = 2.0)]
    lacunarity: f32,
    #[builder(default = Smoothing::None)]
    smoothing: Smoothing,
    #[builder(default = false)]
    restore_original_state: bool,
}
impl<S: NoiseSampler<glam::Vec3>> NoiseSampler<glam::Vec3> for VectorFieldFbmSampler<S> {
    fn sample_2d(&mut self, uv: glam::Vec2) -> glam::Vec3 {
        panic!("2D vector fields are currently not supported")
    }

    fn sample_3d(&mut self, uvw: glam::Vec3) -> glam::Vec3 {
        let backup_frequency = self.sampler.get_frequency();
        let backup_seed = self.sampler.get_seed();

        let mut noise_sum: glam::Vec3 = glam::Vec3::ZERO;
        let mut amplitude_sum: glam::Vec3 = glam::Vec3::ZERO;

        for octave in 0..self.octaves {
            let sample = self.sampler.sample_3d(uvw);
            let amplitude = (1.0 / self.persistence).powf(octave as f32);

            noise_sum += sample * amplitude;
            amplitude_sum += amplitude;

            self.sampler.set_frequency(self.sampler.get_frequency() * self.lacunarity);
            self.sampler.set_seed(pcg_11(self.sampler.get_seed()));
        }

        if self.restore_original_state {
            self.sampler.set_frequency(backup_frequency);
            self.sampler.set_seed(backup_seed);
        }

        self.smoothing.smooth(noise_sum)
    }
}
impl<S: NoiseSampler<glam::Vec3>> NoiseSamplerState for VectorFieldFbmSampler<S> {
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

#[derive(Debug, Builder)]
pub struct CurlSampler<S: NoiseSampler<glam::Vec3>> {
    sampler: S,
    size: glam::Vec3,
}
impl<S: NoiseSampler<glam::Vec3>> CurlSampler<S> {
    fn wrap_uvw_3d(&self, uvw: glam::Vec3) -> glam::Vec3 {
        ((uvw - 0.5) * self.size).rem_euclid(self.size) / self.size
    }
}
impl<S: NoiseSampler<glam::Vec3>> NoiseSampler<glam::Vec3> for CurlSampler<S> {
    fn sample_2d(&mut self, uv: glam::Vec2) -> glam::Vec3 {
        todo!()
    }

    fn sample_3d(&mut self, uvw: glam::Vec3) -> glam::Vec3 {
        let size = self.size * self.sampler.get_frequency();
        let delta = 1.0 / size;
        let span = delta * size * 2.0;

        let delta_x = glam::Vec3::X * delta;
        let delta_y = glam::Vec3::Y * delta;
        let delta_z = glam::Vec3::Z * delta;

        let dx = (self.sampler.sample_3d(self.wrap_uvw_3d(uvw + delta_x))
            - self.sampler.sample_3d(self.wrap_uvw_3d(uvw - delta_x)))
            / span;

        let dy = (self.sampler.sample_3d(self.wrap_uvw_3d(uvw + delta_y))
            - self.sampler.sample_3d(self.wrap_uvw_3d(uvw - delta_y)))
            / span;

        let dz = (self.sampler.sample_3d(self.wrap_uvw_3d(uvw + delta_z))
            - self.sampler.sample_3d(self.wrap_uvw_3d(uvw - delta_z)))
            / span;

        (dy.z - dz.y) * glam::Vec3::X
            + (dz.x - dx.z) * glam::Vec3::Y
            + (dx.y - dy.x) * glam::Vec3::Z
    }
}
impl<S: NoiseSampler<glam::Vec3>> NoiseSamplerState for CurlSampler<S> {
    fn get_frequency(&self) -> f32 {
        self.sampler.get_frequency()
    }

    fn get_seed(&self) -> u32 {
        self.sampler.get_seed()
    }

    fn set_frequency(&mut self, new_frequency: f32) {
        self.sampler.set_frequency(new_frequency);
    }

    fn set_seed(&mut self, new_seed: u32) {
        self.sampler.set_seed(new_seed);
    }
}
