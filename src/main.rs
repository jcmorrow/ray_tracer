extern crate noise;

use camera::Camera;
use color::Color;
use material::Material;
use matrix::Matrix4;
use pattern::*;
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

    let mut cube = Shape::cube();
    cube.transform = Matrix4::rotation_x(PI / 2.)
        .multiply(&Matrix4::translation(0., 0., 1.))
        .multiply(&Matrix4::scaling(2., 2., 2.));

    let mut floor = Shape::plane();
    floor.transform = Matrix4::translation(0., -4., 0.);

    let mut material = Material::new();
    material.reflective = 0.;

    let mut pattern = Checker::new(Color::new(0.2, 0.65, 0.9), Color::white());
    pattern.transform = Matrix4::scaling(0.25, 0.25, 0.25);

    // let mut perlin_pattern = Perlin::new(Box::new(pattern.clone()));
    // perlin_pattern.factor = 0.25;
    // material.pattern = Box::new(perlin_pattern.clone());
    material.pattern = Box::new(pattern.clone());
    cube.material = material.clone();
    floor.material = material.clone();

    world.objects.push(cube);
    world.objects.push(floor);

    let mut camera = Camera::new(1000, 600, PI / 4.);
    let from = point(0., 3.5, -5.);
    let to = point(0., 0., 0.);
    let up = point(0., 1., 0.);
    camera.transform = TransformationMatrix::new(&from, &to, &up);

    let canvas = camera.render(&world);

    let mut file = File::create("ray_cast.ppm")?;
    file.write_all(&canvas.render_ppm().into_bytes())?;
    Ok(())
}
