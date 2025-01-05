use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec::{Vec3, VecExt},
};

use super::{Color, Linear, Material};

#[derive(Clone, Debug)]
pub struct Lambertian {
    albedo: Color<Linear>,
}

impl Lambertian {
    pub fn new(albedo: Color<Linear>) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color<Linear>)> {
        let mut scatter_direction = hit.normal.clone() + Vec3::random_unit();

        if scatter_direction.near_zero() {
            scatter_direction = hit.normal.clone();
        }

        let scattered = Ray::new(hit.point.clone(), scatter_direction);
        let attenuation = self.albedo.clone();
        Some((scattered, attenuation))
    }
}
