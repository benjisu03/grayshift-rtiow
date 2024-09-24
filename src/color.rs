use crate::vec3::Vec3;
use std::fs::File;
use std::io::Write;

pub fn write_color(image: &mut File, color: Vec3) -> std::io::Result<()> {
    let r = linear_to_gamma(color.x);
    let g = linear_to_gamma(color.y);
    let b = linear_to_gamma(color.z);

    let r_byte = (255.999 * r) as i32;
    let g_byte = (255.999 * g) as i32;
    let b_byte = (255.999 * b) as i32;

    writeln!(image, "{r_byte} {g_byte} {b_byte}")
}

fn linear_to_gamma(n: f64) -> f64 {
    if n > 0.0 {
        return n.sqrt();
    }
    0.0
}