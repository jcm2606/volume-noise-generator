use std::ops::{Add, Mul, Rem};

use glam::Vec3;
use image::GenericImage;

pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    f32::min(f32::max(value, min), max)
}

pub fn mix(x: f32, y: f32, alpha: f32) -> f32 {
    x * (1.0f32 - alpha) + y * alpha
}

pub fn remap(value: f32, old_min: f32, old_max: f32, new_min: f32, new_max: f32) -> f32 {
    new_min + (((value - old_min) / (old_max - old_min)) * (new_max - new_min))
}

pub fn remap_clamp(value: f32, old_min: f32, old_max: f32, new_min: f32, new_max: f32) -> f32 {
    clamp(
        remap(value, old_min, old_max, new_min, new_max),
        new_min,
        new_max,
    )
}

pub fn smoothstep(a: f32, b: f32, mut value: f32) -> f32 {
    value = clamp((value - a) / (b - a), 0f32, 1f32);
    value * value * (3f32 - 2f32 * value)
}
