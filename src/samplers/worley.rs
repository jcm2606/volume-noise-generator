use bon::Builder;
use glam::{Vec3Swizzles, Vec4Swizzles};

use crate::hash::{hash_ivec2, hash_ivec3, pcg_1d, rand_unit_vector_2d, rand_unit_vector_3d, rand_vector_2d, rand_vector_3d};
use crate::random::hash::{pcg_33, pcg_44};
use crate::samplers::{NoiseSampler, NoiseSamplerState, Smoothing};
use crate::util::{SmoothingFn, clamp, cubic_smooth};

#[derive(Debug)]
pub enum WorleyMode {
    F1,
    F2,
    OneMinusF1,
    OneMinusF2,
    F2MinusF1,
    F1MinusF2,
}

#[derive(Debug, Builder)]
pub struct WorleySampler {
    pub frequency: f32,
    #[builder(default = 0)]
    pub seed: u32,
    #[builder(default = Smoothing::None)]
    pub smoothing: Smoothing,
    #[builder(default = WorleyMode::OneMinusF1)]
    pub mode: WorleyMode,
}
impl NoiseSampler<f32> for WorleySampler {
    fn sample_2d(&mut self, mut uv: glam::Vec2) -> f32 {
        uv *= glam::Vec2::splat(self.frequency);

        let p = uv.floor().as_ivec2();
        let f = uv.fract();

        let mut closest_dist: f32 = 1.0;
        let mut second_closest_dist: f32 = 1.0;

        for x in -1..=1 {
            for y in -1..=1 {
                let offset = glam::IVec2::new(x, y);
                let cell = (p + offset).rem_euclid(glam::IVec2::splat(self.frequency as i32));

                let hash3 = pcg_33(glam::UVec3::new(cell.x.cast_unsigned(), cell.y.cast_unsigned(), self.seed));
                let point = (hash3.xy().as_vec2() / (u32::MAX as f32)) + offset.as_vec2();

                let dist = f.distance(point);
                if dist < closest_dist {
                    second_closest_dist = closest_dist;
                    closest_dist = dist;
                } else if dist < second_closest_dist {
                    second_closest_dist = dist;
                }
            }
        }

        let value = match &self.mode {
            WorleyMode::F1 => closest_dist,
            WorleyMode::F2 => second_closest_dist,
            WorleyMode::OneMinusF1 => 1.0 - closest_dist,
            WorleyMode::OneMinusF2 => 1.0 - second_closest_dist,
            WorleyMode::F2MinusF1 => second_closest_dist - closest_dist,
            WorleyMode::F1MinusF2 => closest_dist - second_closest_dist,
        };

        self.smoothing.smooth(value.clamp(0.0, 1.0))
    }
    
    fn sample_3d(&mut self, mut uvw: glam::Vec3) -> f32 {
        uvw *= glam::Vec3::splat(self.frequency);

        let p = uvw.floor().as_ivec3();
        let f = uvw.fract();

        let mut closest_dist: f32 = 1.0;
        let mut second_closest_dist: f32 = 1.0;

        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    let offset = glam::IVec3::new(x, y, z);
                    let cell = (p + offset).rem_euclid(glam::IVec3::splat(self.frequency as i32));

                    let hash4 = pcg_44(glam::UVec4::new(cell.x.cast_unsigned(), cell.y.cast_unsigned(), cell.z.cast_unsigned(), self.seed));
                    let point = (hash4.xyz().as_vec3() / (u32::MAX as f32)) + offset.as_vec3();

                    let dist = f.distance(point);
                    if dist < closest_dist {
                        second_closest_dist = closest_dist;
                        closest_dist = dist;
                    } else if dist < second_closest_dist {
                        second_closest_dist = dist;
                    }
                }
            }
        }

        let value = match &self.mode {
            WorleyMode::F1 => closest_dist,
            WorleyMode::F2 => second_closest_dist,
            WorleyMode::OneMinusF1 => 1.0 - closest_dist,
            WorleyMode::OneMinusF2 => 1.0 - second_closest_dist,
            WorleyMode::F2MinusF1 => second_closest_dist - closest_dist,
            WorleyMode::F1MinusF2 => closest_dist - second_closest_dist,
        };

        self.smoothing.smooth(value.clamp(0.0, 1.0))
    }
}
impl NoiseSamplerState for WorleySampler {
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
