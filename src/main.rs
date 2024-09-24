mod vec3;

use indicatif::ProgressIterator;
use std::error::Error;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>>{

    let mut image_file = File::create("image.ppm")?;

    let image_width = 256;
    let image_height = 256;

    write!(image_file, "P3\n{image_width} {image_height}\n255\n")?;

    for j in (0..image_height).progress() {
        for i in 0..image_width {
            let r = (i as f64) / ((image_width - 1) as f64);
            let g = (j as f64) / ((image_height - 1) as f64);
            let b = 0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            writeln!(image_file, "{ir} {ig} {ib}")?;
        }
    }

    Ok(())
}
