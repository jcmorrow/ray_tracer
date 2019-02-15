use color::Color;
use intersection::Intersection;
use material::Material;
use matrix::Matrix4;
use point::point;
use point_light::PointLight;
use sphere::Sphere;

use std::fs::File;
use std::io::prelude::*;

mod canvas;
mod color;
mod intersection;
mod material;
mod matrix;
mod point;
mod point_light;
mod ray;
mod sphere;
mod utilities;
mod world;

fn main() -> std::io::Result<()> {
    let size = 300;
    let mut canvas = canvas::Canvas::empty(size, size);

    let origin = point(0.0, 0.0, -5.0);
    let wall = point(0.0, 0.0, 10.0);
    let mut sphere = Sphere {
        transform: Matrix4::translation(0.0, 0.0, 0.0),
        material: Material::new(),
    };
    let light = PointLight {
        intensity: Color::new(1.0, 1.0, 1.0),
        position: point(-10.0, 10.0, -10.0),
    };
    sphere.material.color = Color::new(1.0, 0.2, 1.0);

    let pixel_size_in_world = 7.0 / size as f64;
    let x_min = -3.5;
    let y_max = 3.5;

    for y in 0..size {
        let y_target = y_max - pixel_size_in_world * y as f64;
        for x in 0..size {
            let x_target = x_min + pixel_size_in_world * x as f64;
            let r = ray::Ray {
                origin: origin,
                direction: point::vector(x_target, y_target, wall.z).normalize(),
            };
            let mut hits = r.intersect(sphere);
            if hits.len() > 0 {
                let hit = Intersection::hit(&mut hits).unwrap();
                let point = r.position(hit.t);
                let normal = hit.object.normal_at(&point);
                let eye = r.direction.multiply_scalar(-1.0);
                let color = hit.object.material.lighting(&light, &point, &eye, &normal);
                canvas.write_pixel(y as usize, x as usize, &color);
            }
        }
    }

    let mut file = File::create("ray_cast.ppm")?;
    file.write_all(&canvas.render_ppm().into_bytes())?;
    Ok(())
}
