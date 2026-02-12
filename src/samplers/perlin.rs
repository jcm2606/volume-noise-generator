use std::f32;

use bon::Builder;
use glam::{Vec3Swizzles, Vec4Swizzles};

use crate::hash::{
    hash_ivec2, hash_ivec3, pcg_1d, rand_unit_vector_2d, rand_unit_vector_3d, rand_vector_2d,
    rand_vector_3d,
};
use crate::random::hash::{pcg_31, pcg_33, pcg_41, pcg_44};
use crate::random::unit::{unit_vector_12, unit_vector_23};
use crate::samplers::{NoiseSampler, NoiseSamplerState, Smoothing};
use crate::util::{clamp, mix, MappingFn, SmoothingFn};

#[derive(Debug)]
pub enum PerlinMode {
    Normal,
    Ridged,
}

#[derive(Debug, Builder)]
pub struct PerlinSampler {
    pub frequency: f32,
    #[builder(default = 0)]
    pub seed: u32,
    #[builder(default = Smoothing::None)]
    pub smoothing: Smoothing,
    #[builder(default = PerlinMode::Normal)]
    pub mode: PerlinMode,
    #[builder(default = false)]
    pub vary_gradient_magnitudes: bool,
}
impl PerlinSampler {
    fn project_2d(&self, pi: glam::IVec2, pf: glam::Vec2, offset: glam::IVec2) -> f32 {
        //  FIXME: Possible quality issue with hash function?

        let cell = (pi + offset).rem_euclid(glam::IVec2::splat(self.frequency as i32));
        // let hash = pcg_31(glam::uvec3(
        //     cell.x.cast_unsigned(),
        //     cell.y.cast_unsigned(),
        //     self.seed,
        // )) & 7;

        // let gr = glam::vec2((hash & 1) as f32, ((hash >> 1) & 1) as f32) * 2.0 - 1.0;
        // let point = if hash >= 6 {
        //     glam::vec2(0.0, gr.x)
        // } else if hash >= 4 {
        //     glam::vec2(gr.x, 0.0)
        // } else {
        //     gr
        // };

        let hash = pcg_33(glam::uvec3(cell.x.cast_unsigned(), cell.y.cast_unsigned(), self.seed));
        let mut point = unit_vector_12(hash.x);

        if self.vary_gradient_magnitudes {
            point *= (hash.y as f32) / (u32::MAX as f32);
        }

        (pf - offset.as_vec2()).dot(point)
    }

    fn project_3d(&self, pi: glam::IVec3, pf: glam::Vec3, offset: glam::IVec3) -> f32 {
        let cell = (pi + offset).rem_euclid(glam::IVec3::splat(self.frequency as i32));
        // let hash = pcg_41(glam::uvec4(
        //     cell.x.cast_unsigned(),
        //     cell.y.cast_unsigned(),
        //     cell.z.cast_unsigned(),
        //     self.seed,
        // )) & 31;

        // let mut point = glam::vec3(
        //     (hash & 1) as f32,
        //     ((hash >> 1) & 1) as f32,
        //     ((hash >> 2) & 1) as f32,
        // ) * 2f32
        //     - 1f32;

        // let zeroed_out = (hash >> 3) & 3;
        // match zeroed_out {
        //     0 => {}
        //     _ => point[(zeroed_out - 1) as usize] = 0.0,
        // };

        let hash = pcg_44(glam::uvec4(
            cell.x.cast_unsigned(),
            cell.y.cast_unsigned(),
            cell.z.cast_unsigned(),
            self.seed,
        ));
        let mut point = unit_vector_23(hash.xy());

        if self.vary_gradient_magnitudes {
            point *= (hash.z as f32) / (u32::MAX as f32);
        }

        (pf - offset.as_vec3()).dot(point)
    }
}
impl NoiseSampler<f32> for PerlinSampler {
    fn sample_2d(&mut self, mut uv: glam::Vec2) -> f32 {
        uv *= self.frequency;

        let pi = uv.floor().as_ivec2();
        let pf = uv.fract().abs();

        let f = pf.quintic_smooth();

        let p00 = self.project_2d(pi, pf, glam::ivec2(0, 0));
        let p10 = self.project_2d(pi, pf, glam::ivec2(1, 0));
        let p01 = self.project_2d(pi, pf, glam::ivec2(0, 1));
        let p11 = self.project_2d(pi, pf, glam::ivec2(1, 1));

        let value = mix(mix(p00, p10, f.x), mix(p01, p11, f.x), f.y).clamped_map(-1.0, 1.0);
        match &self.mode {
            PerlinMode::Normal => self.smoothing.smooth(value),
            PerlinMode::Ridged => self.smoothing.smooth((value * 2.0 - 1.0).abs()),
        }
    }

    fn sample_3d(&mut self, mut uvw: glam::Vec3) -> f32 {
        uvw *= self.frequency;

        let pi = uvw.floor().as_ivec3();
        let pf = uvw.fract();

        let f = pf.quintic_smooth();

        let p000 = self.project_3d(pi, pf, glam::ivec3(0, 0, 0));
        let p001 = self.project_3d(pi, pf, glam::ivec3(0, 0, 1));
        let p010 = self.project_3d(pi, pf, glam::ivec3(0, 1, 0));
        let p011 = self.project_3d(pi, pf, glam::ivec3(0, 1, 1));
        let p100 = self.project_3d(pi, pf, glam::ivec3(1, 0, 0));
        let p101 = self.project_3d(pi, pf, glam::ivec3(1, 0, 1));
        let p110 = self.project_3d(pi, pf, glam::ivec3(1, 1, 0));
        let p111 = self.project_3d(pi, pf, glam::ivec3(1, 1, 1));

        let x00 = mix(p000, p100, f.x);
        let x01 = mix(p001, p101, f.x);
        let x10 = mix(p010, p110, f.x);
        let x11 = mix(p011, p111, f.x);

        let y0 = mix(x00, x10, f.y);
        let y1 = mix(x01, x11, f.y);

        let value = mix(y0, y1, f.z).clamped_map(-1.0, 1.0);
        match &self.mode {
            PerlinMode::Normal => self.smoothing.smooth(value),
            PerlinMode::Ridged => self.smoothing.smooth((value * 2.0 - 1.0).abs()),
        }
    }
}
impl NoiseSamplerState for PerlinSampler {
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
