use matrix::Matrix4;
use sphere::Sphere;

use std::fs::File;
use std::io::prelude::*;

mod canvas;
mod color;
mod intersection;
mod matrix;
mod point;
mod ray;
mod sphere;
mod utilities;

fn main() -> std::io::Result<()> {
    let size = 100;
    let mut canvas = canvas::Canvas::empty(size, size);

    let origin = point::point(0.0, 0.0, -5.0);
    let wall = point::point(0.0, 0.0, 10.0);
    let sphere = Sphere {
        transform: Matrix4::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    };

    let pixel_size_in_world = 7.0 / size as f64;
    let x_min = -3.5;
    let y_max = 3.5;

    for y in 0..size {
        let y_target = y_max - pixel_size_in_world * y as f64;
        for x in 0..size {
            let x_target = x_min + pixel_size_in_world * x as f64;
            let r = ray::Ray {
                origin: origin,
                direction: point::vector(x_target, y_target, wall.z),
            };
            if r.intersect(sphere).len() > 0 {
                canvas.write_pixel(
                    y as usize,
                    x as usize,
                    &color::Color::new(1.0, x as f64 / size as f64, y as f64 / size as f64),
                );
            }
        }
    }

    let mut file = File::create("ray_cast.ppm")?;
    file.write_all(&canvas.render_ppm().into_bytes())?;
    Ok(())
}
