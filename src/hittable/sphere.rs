use std::{ops::Range, sync::Arc};

use crate::{material::Material, point::Point3, ray::Ray, utils::RangeExt};

use super::{HitRecord, Hittable};

pub struct Sphere {
    center: Point3,
    radius: f32,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Arc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Range<f32>) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let discriminant_sqrt = discriminant.sqrt();
        let root = (h - discriminant_sqrt) / a;
        if !ray_t.surrounds(&root) {
            let root = (h + discriminant_sqrt) / a;
            if !ray_t.surrounds(&root) {
                return None;
            }
        }

        let point = ray.at(root);
        let mut hit_rec = HitRecord::new(
            ray.direction,
            point,
            ((point - self.center) / self.radius).normalize(),
            root,
            self.material.clone(),
        );
        let outward_normal = (hit_rec.point - self.center) / self.radius;
        hit_rec.set_face_normal(ray, outward_normal.normalize());

        Some(hit_rec)
    }
}
