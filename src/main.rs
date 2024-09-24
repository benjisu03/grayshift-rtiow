mod vec3;
mod color;
mod ray;

use indicatif::ProgressIterator;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use log::info;
use log::LevelFilter;
use crate::color::write_color;
use crate::vec3::Vec3;

fn main() -> Result<(), Box<dyn Error>>{

    let mut logger = colog::default_builder();
    logger.filter_level(LevelFilter::Trace);
    logger.init();

    let mut image_file = File::create("image.ppm")?;

    let image_width = 256;
    let image_height = 256;

    write!(image_file, "P3\n{image_width} {image_height}\n255\n")?;

    for j in (0..image_height).progress() {
        for i in 0..image_width {
            let pixel_color = Vec3::new(
                (i as f64) / ((image_width - 1) as f64),
                (j as f64) / ((image_height - 1) as f64),
                0.0
            );

           write_color(&mut image_file, pixel_color)?;
        }
    }

    info!("Done");
    Ok(())
}
