#![allow(dead_code)]

use std::{ops::Range, sync::LazyLock};

use rand::{distributions::Uniform, prelude::Distribution};

pub const INFINITY: f32 = f32::INFINITY;
pub const PI: f32 = std::f32::consts::PI;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub trait RangeExt<T> {
    fn surrounds(&self, value: &T) -> bool;
}

impl<T> RangeExt<T> for Range<T>
where
    T: PartialOrd,
{
    fn surrounds(&self, value: &T) -> bool {
        self.start < *value && *value < self.end
    }
}

pub trait ClampRange: Sized {
    fn clamp_range(&self, range: &Range<Self>) -> Self;
}

impl ClampRange for f32 {
    fn clamp_range(&self, range: &Range<f32>) -> f32 {
        match self {
            x if *x < range.start => range.start,
            x if *x > range.end => range.end,
            x => *x,
        }
    }
}

static UNIFORM: LazyLock<Uniform<f32>> = LazyLock::new(|| Uniform::new(0.0, 0.999));

pub trait Random<T>: Sized {
    fn random() -> Self;

    fn random_range(range: &Range<T>) -> Self;
}

impl Random<f32> for f32 {
    fn random() -> f32 {
        let mut rng = rand::thread_rng();
        UNIFORM.sample(&mut rng)
    }

    fn random_range(range: &Range<f32>) -> f32 {
        range.start + (range.end - range.start) * f32::random()
    }
}

pub trait WriteColor {
    fn write_color(&self);
}
