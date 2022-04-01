mod vec3;
mod ray;
use crate::vec3::*;
use crate::ray::*;
fn main() {
    //first_image();
    gradient()
}

fn gradient() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = image_width as f64 / aspect_ratio;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    println!("P3\n{} {}\n255\n", image_width, image_height);

    for row in (0..image_height as i32).rev() {
        eprintln!("Scanlines remaining: {}", row);
        for column in 0..image_width {
            let u: f64 = column as f64 / (image_width - 1) as f64;
            let v: f64 = row as f64 / (image_height - 1.0);
            let ray = Ray::new(origin, lower_left_corner + horizontal * u  + vertical * v  - origin);
            let pixel_color = ray_color(&ray);
            write_color(pixel_color);
        }
    }
    eprintln!("Done.");
}

fn ray_color(ray: &Ray) -> Color {
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t)  + Color::new(0.5, 0.7, 1.0) * t
}

fn first_image() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{} {}\n255\n", image_width, image_height);

    for row in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", row);
        for column in 0..image_width {
            let r: f64 = column as f64 / (image_width - 1) as f64;
            let g: f64 = row as f64 / (image_height - 1) as f64;
            let b = 0.25;
            let pixel_color = Color::new(r, g, b);

            write_color(pixel_color);
        }
    }
    eprintln!("Done.");
}