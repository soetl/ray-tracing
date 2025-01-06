use std::{ops::Range, sync::LazyLock};

use rand::{distributions::Uniform, prelude::Distribution};

pub const INFINITY: f32 = f32::INFINITY;

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

pub trait Clamp<T> {
    fn clamp(&self, value: T) -> T;
}

impl Clamp<f32> for Range<f32> {
    fn clamp(&self, value: f32) -> f32 {
        match value {
            x if x < self.start => self.start,
            x if x > self.end => self.end,
            x => x,
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
