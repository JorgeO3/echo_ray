#![allow(unused)]

use std::io::{BufWriter, Write};

use anyhow::Result;

use ray::Ray;
use vec3::{Point3, Vec3};

mod ray;
mod vec3;
mod color;


fn main() -> Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let img_width = 400.0;
    let file = std::fs::File::create("image.ppm")?;
    let mut out = BufWriter::new(file);

    // Image height
    let img_heigh = f64::max(1.0, img_width / aspect_ratio);

    // Insert the headers file
    writeln!(out, "P3\n{} {}\n255", img_width, img_heigh)?;

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (img_width / img_heigh);
    let camera_center = Point3::new(0, 0, 0);

    dbg!(focal_length, viewport_height, viewport_width, camera_center);

    // vectors across the horizontal and down the vertical viewport edges
    let viewport_u = Vec3::new(viewport_width, 0, 0);
    let viewport_v = Vec3::new(0, -viewport_height, 0);

    dbg!(viewport_u, viewport_v);

    // horizontal and vertical delta vectors from pixcel to pixel
    let pixel_delta_u = viewport_u / img_width;
    let pixel_delta_v = viewport_v / img_heigh;

    dbg!(pixel_delta_u, pixel_delta_v);

    // the location of the upper left pixel
    let viewport_upper_left = camera_center - Vec3::new(0, 0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    dbg!(viewport_upper_left, pixel00_loc);

    for j in 0..(img_heigh as i32) {
        for i in 0..(img_width as i32) {
            let i = f64::from(i);
            let j = f64::from(j);
            let pixel_center = pixel00_loc + pixel_delta_u * i + pixel_delta_v * j;
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = Ray::ray_color(&r);
            writeln!(out, "{}", pixel_color)?;
        }
    }

    out.flush()?;
    Ok(())
}

// fn main() -> Result<()> {
//     let (image_width, image_height) = (256, 256);
//     let file = File::create("image.ppm")?;
//     let mut out = BufWriter::new(file);
//
//     writeln!(out, "P3\n{} {}\n255", image_width, image_height)?;
//
//     for j in 0..image_height {
//         for i in 0..image_width {
//             let r = f64::from(i) / f64::from(image_width - 1);
//             let g = f64::from(j) / f64::from(image_height - 1);
//             let b = 0.0;
//
//             let pixel_color = Vec3::new(r, g, b);
//             writeln!(out, "{}", pixel_color)?;
//         }
//     }
//
//     out.flush()?;
//     Ok(())
// }
