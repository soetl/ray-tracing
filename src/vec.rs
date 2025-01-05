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
    fn reflect(&self, n: Vec3) -> Vec3;
    fn refract(&self, n: Vec3, etai_over_etat: f32) -> Vec3;
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

    fn reflect(&self, n: Vec3) -> Vec3 {
        self - 2.0 * self.dot(n) * n
    }

    fn refract(&self, n: Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = (-self).dot(n).min(1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
        r_out_perp + r_out_parallel
    }
}
