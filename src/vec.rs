use std::ops::Range;

use crate::utils::Random;

pub type Vec3 = glam::Vec3A;

impl Random<f32> for Vec3 {
    fn random() -> Vec3 {
        Vec3::new(f32::random(), f32::random(), f32::random())
    }

    fn random_range(range: &Range<f32>) -> Vec3 {
        Vec3::new(
            f32::random_range(range),
            f32::random_range(range),
            f32::random_range(range),
        )
    }
}

pub(crate) trait VecExt {
    fn near_zero(&self) -> bool;
    fn random_unit() -> Vec3;
    fn random_in_unit_disk() -> Vec3;
}

impl VecExt for Vec3 {
    fn near_zero(&self) -> bool {
        const S: f32 = 1e-8;
        self.x.abs() < S && self.y.abs() < S && self.z.abs() < S
    }

    fn random_unit() -> Vec3 {
        loop {
            let p = Vec3::random_range(&(-1.0..1.0));
            let lensq = p.length_squared();
            if 1e-45 < lensq && lensq <= 1.0 {
                return p.normalize();
            }
        }
    }

    fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(
                f32::random_range(&(-1.0..1.0)),
                f32::random_range(&(-1.0..1.0)),
                0.0,
            );
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
}
