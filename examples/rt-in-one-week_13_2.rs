use std::sync::Arc;

use ray_tracing::prelude::*;

fn main() {
    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_bubble = Dielectric::new(1.0 / 1.5);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Arc::new(material_ground),
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        Arc::new(material_center),
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Arc::new(material_left),
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        Arc::new(material_bubble),
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Arc::new(material_right),
    )));

    match render(
        &world,
        "output/rt-in-one-week-13.2.png",
        &CameraConfig {
            look_from: Point3::new(-2.0, 2.0, 1.0),
            vfov: 20.0,
            defocus_angle: 10.0,
            focus_distance: 3.4,
            ..Default::default()
        },
    ) {
        Ok(_) => println!("Done."),
        Err(e) => println!("Error: {}", e),
    }
}
