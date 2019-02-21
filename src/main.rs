use camera::Camera;
use color::Color;
use material::Material;
use matrix::Matrix4;
use pattern::StripePattern;
use point::point;
use shape::Shape;
use std::f64::consts::PI;
use std::fs::File;
use std::io::prelude::*;
use transformation_matrix::TransformationMatrix;
use world::World;

mod camera;
mod canvas;
mod color;
mod intersectable;
mod intersection;
mod material;
mod matrix;
mod pattern;
mod point;
mod point_light;
mod ray;
mod shape;
mod transformation_matrix;
mod utilities;
mod world;

fn main() -> std::io::Result<()> {
    let mut world = World::new();

    let mut material = Material::new();
    material.pattern = Box::new(StripePattern::new(Color::black(), Color::white()));

    let mut floor = Shape::plane();
    floor.transform = Matrix4::translation(0.0, -1.0, 0.0);
    world.objects.push(floor);

    let mut camera = Camera::new(200, 100, PI / 3.0);
    let from = point(0.0, 1.5, -5.0);
    let to = point(0.0, 0.0, 0.0);
    let up = point(0.0, 1.0, 0.0);
    camera.transform = TransformationMatrix::new(&from, &to, &up);

    let canvas = camera.render(&world);

    let mut file = File::create("ray_cast.ppm")?;
    file.write_all(&canvas.render_ppm().into_bytes())?;
    Ok(())
}
