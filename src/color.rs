use crate::vec3::Color;
use crate::util;

pub fn write_color(pixel_color: &Color, samples_per_pixel: u32) {
    let scale = 1.0 / samples_per_pixel as f64;
    let r = (pixel_color.x * scale).sqrt();
    let g = (pixel_color.y * scale).sqrt();
    let b = (pixel_color.z * scale).sqrt();

    println!(
        "{} {} {}",
        (256.0 * util::clamp(r, 0.0, 0.999)) as i32,
        (256.0 * util::clamp(g, 0.0, 0.999)) as i32,
        (256.0 * util::clamp(b, 0.0, 0.999)) as i32,
    );
}