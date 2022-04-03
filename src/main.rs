mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod util;
mod vec3;

use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{write_color, Color, Point3, Vec3};

fn main() {
    // Image settings
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = image_width as f64 / aspect_ratio;

    // World objects
    let mut world = HittableList::new();
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    // Camera settings
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render the image

    println!("P3\n{} {}\n255\n", image_width, image_height);

    // Traverse the scene from left to right, button to top
    for row in (0..image_height as i32).rev() {
        eprintln!("Scanlines remaining: {}", row);
        for column in 0..image_width {
            // u (horizontal) and v (vertical) scale from 0.0 to 1.0
            let u: f64 = column as f64 / (image_width - 1) as f64;
            let v: f64 = row as f64 / (image_height - 1.0);
            // Ray from origin to point on scene
            let ray = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            let pixel_color = ray_color(&ray, &world);
            write_color(&pixel_color);
        }
    }

    eprintln!("Done.");
}

fn ray_color(ray: &Ray, world: &impl Hittable) -> Color {
    let mut rec = HitRecord::default();
    if world.hit(&ray, 0.0, util::INFINITY, &mut rec) {
        // map x/y/z to r/g/b
        (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5
    } else {
        // Compute a background gradient based on y-coordinate of ray
        let unit_direction = ray.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
    }
}
