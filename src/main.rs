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
use crate::ray::Ray;
use crate::vec3::Vec3;

fn ray_color(ray: &Ray) -> Vec3 {
    let blend = 0.5 * (ray.direction.unit().y + 1.0);
    let white = Vec3::new(1.0, 1.0, 1.0);
    let blue = Vec3::new(0.5, 0.7, 1.0);

    blend * blue + (1.0 - blend) * white
}

fn main() -> Result<(), Box<dyn Error>>{

    // LOGGING //

    let mut logger = colog::default_builder();
    logger.filter_level(LevelFilter::Trace);
    logger.init();

    // IMAGE //

    let mut image_file = File::create("image.ppm")?;

    let image_width = 256;
    let aspect_ratio = 16.0 / 9.0;

    let image_height = ((image_width as f64) / aspect_ratio) as i32;

    // CAMERA //

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let camera_center = Vec3::ZERO;

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / (image_width as f64);
    let pixel_delta_v = viewport_v / (image_height as f64);

    let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let starting_pixel_location = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);


    write!(image_file, "P3\n{image_width} {image_height}\n255\n")?;

    for j in (0..image_height).progress() {
        for i in 0..image_width {
            let pixel_center = starting_pixel_location + (i as f64) * pixel_delta_u + (j as f64) * pixel_delta_v;

            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&ray);
            write_color(&mut image_file, pixel_color)?;
        }
    }

    info!("Done");
    Ok(())
}
