use crate::samplers::{hash33, NoiseSampler, SamplerState};
use crate::util::mix;

#[derive(Debug, Clone)]
pub struct PerlinNoiseSampler;
impl NoiseSampler for PerlinNoiseSampler {
    fn sample(mut uvw: glam::Vec3, state: &SamplerState) -> f32 {
        uvw *= state.frequency;

        let p = uvw.floor();
        let f = uvw.fract();

        let u = f * f * f * (f * (f * 6f32 - 15f32) + 10f32);
        // let u = f * f * (3f32 - 2f32 * f);

        let projection_a = project(p, f, glam::vec3(0f32, 0f32, 0f32), state);
        let projection_b = project(p, f, glam::vec3(1f32, 0f32, 0f32), state);
        let projection_c = project(p, f, glam::vec3(0f32, 0f32, 1f32), state);
        let projection_d = project(p, f, glam::vec3(1f32, 0f32, 1f32), state);
        let projection_e = project(p, f, glam::vec3(0f32, 1f32, 0f32), state);
        let projection_f = project(p, f, glam::vec3(1f32, 1f32, 0f32), state);
        let projection_g = project(p, f, glam::vec3(0f32, 1f32, 1f32), state);
        let projection_h = project(p, f, glam::vec3(1f32, 1f32, 1f32), state);

        mix(
            mix(
                mix(projection_a, projection_b, u.x),
                mix(projection_c, projection_d, u.x),
                u.z,
            ),
            mix(
                mix(projection_e, projection_f, u.x),
                mix(projection_g, projection_h, u.x),
                u.z,
            ),
            u.y,
        ) * 0.5f32
            + 0.5f32
    }
}

fn project(p: glam::Vec3, f: glam::Vec3, offset: glam::Vec3, state: &SamplerState) -> f32 {
    let gradient = hash33((p + offset + state.seed).rem_euclid(glam::Vec3::splat(state.frequency)))
        * 2.0f32
        - 1.0f32;

    glam::Vec3::dot(gradient, f - offset)
}
