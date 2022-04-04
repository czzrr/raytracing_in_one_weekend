mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod util;
mod vec3;
mod color;
mod camera;

use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Vec3, Color, Point3};
use crate::color::write_color;
use crate::camera::Camera;

use rand;

fn main() {
    // Image settings
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = image_width as f64 / aspect_ratio;
    let samples_per_pixel = 100;

    // World objects
    let mut world = HittableList::new();
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    // Camera settings
    let cam = Camera::new();

    // Render the image

    println!("P3\n{} {}\n255\n", image_width, image_height);

    // Traverse the scene from left to right, button to top
    for row in (0..image_height as i32).rev() {
        eprintln!("Scanlines remaining: {}", row);
        for column in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u: f64 = (column as f64 + rand::random::<f64>()) / (image_width - 1) as f64;
                let v: f64 = (row as f64 + rand::random::<f64>()) / (image_height - 1.0);
                let ray = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&ray, &world);
            }
            write_color(&pixel_color, samples_per_pixel);
        }
    }

    eprintln!("Done.");
}

fn ray_color(ray: &Ray, world: &impl Hittable) -> Color {
    const MAX_DEPTH: u32 = 50;

    fn helper(ray: &Ray, world: &impl Hittable, depth: u32) -> Color {
        let mut rec = HitRecord::default();
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if world.hit(&ray, 0.001, util::INFINITY, &mut rec) {
            // Random point in unit sphere tangent to surface hit point
            //let target = rec.p + rec.normal + Vec3::random_in_unit_sphere();

            // Lambertian random point on unit sphere
            let target = rec.p + rec.normal + Vec3::random_unit_vector();

            // Random point in normal hemisphere (uniform scatter direction for all angles away from the hit point)
            //let target = rec.p + Vec3::random_in_hemisphere(&rec.normal);

            // Send ray from hit point towards target point
            helper(&Ray::new(rec.p, target - rec.p), world, depth - 1) * 0.5
        } else {
            // Compute a background gradient based on y-coordinate of ray
            let unit_direction = ray.direction.unit_vector();
            let t = 0.5 * (unit_direction.y + 1.0);
            Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
        }
    }

    helper(ray, world, MAX_DEPTH)
}

