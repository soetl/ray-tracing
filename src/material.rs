pub mod dielectric;
pub mod lambertian;
pub mod metal;

use crate::{
    color::{Color, Linear},
    hittable::HitRecord,
    ray::Ray,
};

pub trait Material: Send + Sync + std::fmt::Debug {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color<Linear>)>;
}
