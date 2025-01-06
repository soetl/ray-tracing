use std::sync::Arc;

use camera::Camera;
use color::Color;
use hittable::{sphere::Sphere, HittableList};
use material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal, Material};
use point::Point3;
use utils::Random;
use vec::Vec3;

mod camera;
mod color;
mod hittable;
mod material;
mod point;
mod ray;
mod utils;
mod vec;

fn main() {
    let mut world = HittableList::new();

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(ground_material),
    )));

    (-11..11).for_each(|a| {
        (-11..11).for_each(|b| {
            let choose_material = f32::random();
            let center = Point3::new(
                a as f32 + 0.9 * f32::random(),
                0.2,
                b as f32 + 0.9 * f32::random(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material> = match choose_material {
                    x if x < 0.8 => {
                        let albedo = Color::from(Vec3::random() * Vec3::random());
                        Arc::new(Lambertian::new(albedo))
                    }
                    x if x < 0.95 => {
                        let albedo = Color::from(Vec3::random_range(&(0.5..1.0)));
                        let fuzz = f32::random_range(&(0.0..0.5));
                        Arc::new(Metal::new(albedo, fuzz))
                    }
                    _ => Arc::new(Dielectric::new(1.5)),
                };

                world.push(Box::new(Sphere::new(center, 0.2, sphere_material)));
            }
        });
    });

    let material_1 = Dielectric::new(1.5);
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(material_1),
    )));

    let material_2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.push(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(material_2),
    )));

    let material_3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.push(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(material_3),
    )));

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1200;
    let samples_per_pixel = 500;
    let max_depth = 50;

    let vfov = 20.0;
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::Y;

    let defocus_angle = 0.6;
    let focus_distance = 10.0;

    let camera = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        vfov,
        look_from,
        look_at,
        vup,
        defocus_angle,
        focus_distance,
    );

    let image = camera.render(&world);
    match image.save("output.png") {
        Ok(_) => println!("Image saved successfully."),
        Err(e) => eprintln!("Failed to save image: {}", e),
    }
}
