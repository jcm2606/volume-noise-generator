use crate::samplers::{hash33, NoiseSampler, SamplerState};
use crate::util::clamp;

#[derive(Debug, Clone)]
pub struct WorleyNoiseSampler;
impl NoiseSampler for WorleyNoiseSampler {
    fn sample(mut uvw: glam::Vec3, state: &SamplerState) -> f32 {
        let splatted_frequency = glam::Vec3::splat(state.frequency);
        uvw *= splatted_frequency;

        let p = glam::Vec3::floor(uvw);
        let f = glam::Vec3::fract(uvw);

        let mut min_dist = 1.0;

        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    let offset = glam::vec3(x as f32, y as f32, z as f32);
                    let point =
                        hash33((p + offset + state.seed).rem_euclid(splatted_frequency)) + offset;

                    let delta = f - point;
                    min_dist = f32::min(min_dist, glam::Vec3::dot(delta, delta));
                }
            }
        }

        1f32 - clamp(f32::sqrt(min_dist), 0f32, 1f32)
    }
}
