use atomic_float::AtomicF32;
use std::f32;
use std::sync::atomic::Ordering;

pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    f32::min(f32::max(value, f32::min(min, max)), f32::max(min, max))
}

pub fn clamp_vec3(value: glam::Vec3, min: f32, max: f32) -> glam::Vec3 {
    glam::Vec3::min(
        glam::Vec3::max(value, glam::Vec3::splat(min)),
        glam::Vec3::splat(max),
    )
}

pub fn mix(x: f32, y: f32, alpha: f32) -> f32 {
    x * (1.0f32 - alpha) + y * alpha
}

pub fn mix_vec3(x: glam::Vec3, y: glam::Vec3, alpha: f32) -> glam::Vec3 {
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

pub fn map(min: f32, max: f32, value: f32) -> f32 {
    if (max - min).abs() < f32::EPSILON {
        return value;
    }

    (value - min) / (max - min)
}

pub fn clamped_map(min: f32, max: f32, value: f32) -> f32 {
    clamp(map(min, max, value), 0f32, 1f32)
}

pub fn smoothstep(a: f32, b: f32, mut value: f32) -> f32 {
    value = clamp((value - a) / (b - a), 0f32, 1f32);
    value * value * (3f32 - 2f32 * value)
}

pub fn cubic_smooth(mut value: f32) -> f32 {
    value = value.clamp(0.0, 1.0);
    value * value * (3.0 - 2.0 * value)
}

pub struct Normalized {
    min: AtomicF32,
    max: AtomicF32,
}
impl Normalized {
    pub fn new() -> Self {
        Self {
            min: AtomicF32::new(f32::MAX),
            max: AtomicF32::new(-f32::MAX),
        }
    }

    pub fn update_with(&self, value: f32) -> f32 {
        self.min.fetch_min(value, Ordering::Relaxed);
        self.max.fetch_max(value, Ordering::Relaxed);

        value
    }

    pub fn min(&self) -> f32 {
        self.min.load(Ordering::Relaxed)
    }

    pub fn max(&self) -> f32 {
        self.max.load(Ordering::Relaxed)
    }
}

pub trait MappingFn<T> {
    fn map(self, min: T, max: T) -> Self;

    fn clamped_map(self, min: T, max: T) -> Self;

    fn remap(self, old_min: T, old_max: T, new_min: T, new_max: T) -> Self;
}

pub trait SmoothMappingFn<T>: MappingFn<T> + SmoothingFn {
    fn smoothstep(self, min: T, max: T) -> Self;

    fn smootherstep(self, min: T, max: T) -> Self;
}

impl<T: MappingFn<T> + SmoothingFn> SmoothMappingFn<T> for T {
    fn smoothstep(self, min: T, max: T) -> Self {
        self.map(min, max).cubic_smooth()
    }

    fn smootherstep(self, min: T, max: T) -> Self {
        self.map(min, max).quintic_smooth()
    }
}

impl MappingFn<f32> for f32 {
    fn map(self, min: f32, max: f32) -> Self {
        if (max - min).abs() < f32::EPSILON {
            return self;
        }

        (self - min) / (max - min)
    }

    fn clamped_map(self, min: f32, max: f32) -> Self {
        self.map(min, max).clamp(0.0, 1.0)
    }

    fn remap(self, old_min: f32, old_max: f32, new_min: f32, new_max: f32) -> Self {
        ((self - old_min) / (old_max - old_min)) * (new_max - new_min) + new_min
    }
}

pub trait SmoothingFn {
    fn cubic_smooth(self) -> Self;

    fn quintic_smooth(self) -> Self;
}

impl SmoothingFn for f32 {
    fn cubic_smooth(mut self) -> Self {
        self = self.clamp(0.0, 1.0);
        self * self * (3.0 - 2.0 * self)
    }

    fn quintic_smooth(mut self) -> Self {
        self = self.clamp(0.0, 1.0);
        self * self * self * (self * (self * 6.0 - 15.0) + 10.0)
    }
}

impl SmoothingFn for glam::Vec2 {
    fn cubic_smooth(mut self) -> Self {
        self = self.clamp(glam::Vec2::ZERO, glam::Vec2::ONE);
        self * self * (3.0 - 2.0 * self)
    }

    fn quintic_smooth(mut self) -> Self {
        self = self.clamp(glam::Vec2::ZERO, glam::Vec2::ONE);
        self * self * self * (self * (self * 6.0 - 15.0) + 10.0)
    }
}

impl SmoothingFn for glam::Vec3 {
    fn cubic_smooth(mut self) -> Self {
        self = self.clamp(glam::Vec3::ZERO, glam::Vec3::ONE);
        self * self * (3.0 - 2.0 * self)
    }

    fn quintic_smooth(mut self) -> Self {
        self = self.clamp(glam::Vec3::ZERO, glam::Vec3::ONE);
        self * self * self * (self * (self * 6.0 - 15.0) + 10.0)
    }
}

pub trait CoordWrapping {
    fn wrap_coord(self, size: Self) -> Self;
}

macro_rules! impl_coord_wrapping_for_glam_vector {
    ($vector_type:ty) => {
        impl CoordWrapping for $vector_type {
            fn wrap_coord(self, size: Self) -> Self {
                ((self - 0.5) * size).rem_euclid(size) / size
            }
        }
    };
}

impl_coord_wrapping_for_glam_vector!(glam::Vec2);
impl_coord_wrapping_for_glam_vector!(glam::Vec3);
