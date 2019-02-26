extern crate noise;

use camera::Camera;
use color::Color;
use material::Material;
use matrix::Matrix4;
use pattern::Perlin;
use pattern::{Checker, Gradient, Solid, Stripe};
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
    world.objects = Vec::new();

    let mut sphere = Shape::sphere();
    let mut cube = Shape::cylinder();
    // cube.transform = Matrix4::rotation_x(PI / 4.0);
    sphere.transform = Matrix4::translation(2.0, 0.0, 0.0);

    let mut material = Material::new();
    material.reflective = 0.0;

    let mut pattern = Checker::new(Color::new(0.2, 0.65, 0.9), Color::white());

    pattern.transform = Matrix4::scaling(0.5, 0.5, 0.5);

    // let mut perlin_pattern = Perlin::new(Box::new(pattern.clone()));
    // perlin_pattern.factor = 0.025;
    // material.pattern = Box::new(perlin_pattern.clone());
    material.pattern = Box::new(pattern.clone());
    cube.material = material.clone();
    sphere.material = material.clone();

    let mut floor = Shape::plane();
    floor.transform = Matrix4::translation(0.0, -1.0, 0.0);

    world.objects.push(floor);
    world.objects.push(cube);
    // world.objects.push(sphere);

    let mut camera = Camera::new(600, 400, PI / 3.0);
    let from = point(0.0, 1.5, -5.0);
    let to = point(0.0, 0.0, 0.0);
    let up = point(0.0, 1.0, 0.0);
    camera.transform = TransformationMatrix::new(&from, &to, &up);

    let canvas = camera.render(&world);

    let mut file = File::create("ray_cast.ppm")?;
    file.write_all(&canvas.render_ppm().into_bytes())?;
    Ok(())
}
