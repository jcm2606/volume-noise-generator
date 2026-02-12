use bon::Builder;
use glam::{Vec3Swizzles, Vec4Swizzles};

use crate::random::hash::{pcg_33, pcg_44};
use crate::samplers::{NoiseSampler, NoiseSamplerState, Smoothing};
use crate::util::SmoothingFn;

#[derive(Debug, Builder)]
pub struct AlligatorSampler {
    pub frequency: f32,
    #[builder(default = 0)]
    pub seed: u32,
    #[builder(default = Smoothing::None)]
    pub smoothing: Smoothing,
    #[builder(default = true)]
    pub randomize_cell_strength: bool
}
impl NoiseSampler<f32> for AlligatorSampler {
    fn sample_2d(&mut self, mut uv: glam::Vec2) -> f32 {
        uv *= glam::Vec2::splat(self.frequency);

        let p = uv.floor().as_ivec2();
        let f = uv.fract();

        let mut smallest_strength: f32 = 0.0;
        let mut second_smallest_strength: f32 = 0.0;

        for x in -1..=1 {
            for y in -1..=1 {
                let offset = glam::IVec2::new(x, y);
                let cell = (p + offset).rem_euclid(glam::IVec2::splat(self.frequency as i32));

                let hash3 = pcg_33(glam::UVec3::new(cell.x.cast_unsigned(), cell.y.cast_unsigned(), self.seed));
                let point = (hash3.xy().as_vec2() / (u32::MAX as f32)) + offset.as_vec2();

                let strength = if self.randomize_cell_strength {
                    (hash3.z as f32) / (u32::MAX as f32)
                } else {
                    1.0
                };

                let dist = (1.0 - f.distance(point)).cubic_smooth() * strength;
                if dist > smallest_strength {
                    second_smallest_strength = smallest_strength;
                    smallest_strength = dist;
                } else if dist > second_smallest_strength {
                    second_smallest_strength = dist
                }
            }
        }

        smallest_strength = smallest_strength.clamp(0.0, 1.0);
        second_smallest_strength = second_smallest_strength.clamp(0.0, 1.0);

        self.smoothing.smooth(smallest_strength - second_smallest_strength)
    }
    
    fn sample_3d(&mut self, mut uvw: glam::Vec3) -> f32 {
        uvw *= glam::Vec3::splat(self.frequency);

        let p = uvw.floor().as_ivec3();
        let f = uvw.fract();

        let mut smallest_dist: f32 = 0.0;
        let mut second_smallest_dist: f32 = 0.0;

        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    let offset = glam::IVec3::new(x, y, z);
                    let cell = (p + offset).rem_euclid(glam::IVec3::splat(self.frequency as i32));

                    let hash4 = pcg_44(glam::UVec4::new(cell.x.cast_unsigned(), cell.y.cast_unsigned(), cell.z.cast_unsigned(), self.seed));
                    let point = (hash4.xyz().as_vec3() / (u32::MAX as f32)) + offset.as_vec3();

                    let strength = if self.randomize_cell_strength {
                        (hash4.w as f32) / (u32::MAX as f32)
                    } else {
                        1.0
                    };

                    let dist = (1.0 - f.distance(point)).quintic_smooth() * strength;
                    if dist > smallest_dist {
                        second_smallest_dist = smallest_dist;
                        smallest_dist = dist;
                    } else if dist > second_smallest_dist {
                        second_smallest_dist = dist
                    }
                }
            }
        }

        smallest_dist = smallest_dist.clamp(0.0, 1.0);
        second_smallest_dist = second_smallest_dist.clamp(0.0, 1.0);

        self.smoothing.smooth(smallest_dist - second_smallest_dist)
    }
}
impl NoiseSamplerState for AlligatorSampler {
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