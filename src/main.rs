extern crate noise;

use camera::Camera;
use color::Color;
use material::Material;
use matrix::Matrix4;
use obj_parser::ObjParser;
use pattern::*;
use point::point;
use shape::Shape;
use std::f64::consts::PI;
use std::fs;
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
mod obj_parser;
mod pattern;
mod point;
mod point_light;
mod ray;
mod shape;
mod transformation_matrix;
mod utilities;
mod world;

fn main() -> std::io::Result<()> {
    let mut obj = File::open("ray_cast.ppm")?;
    let obj_parser = ObjParser::parse(&fs::read_to_string("fixtures/teapot.obj")?);

    println!("{:?}", obj_parser);

    let mut world = World::new();
    world.objects = Vec::new();

    let mut sphere = Shape::sphere();
    sphere.transform = Matrix4::translation(-2.5, 0., 4.).multiply(&Matrix4::scaling(2., 2., 2.));

    let mut floor = Shape::plane();
    floor.transform = Matrix4::translation(0., -2., 0.);

    let mut sphere_material = Material::new();
    sphere_material.reflective = 0.1;
    let mut floor_material = Material::new();
    floor_material.reflective = 0.1;

    let mut pattern = Checker::new(Color::new(0.2, 0.65, 0.9), Color::white());
    let mut gradient = Gradient::new(Color::new(0.2, 0.65, 0.9), Color::white());
    sphere_material.pattern = Box::new(gradient.clone());
    pattern.transform = Matrix4::scaling(0.25, 0.25, 0.25);

    let mut perlin_pattern = Perlin::new(Box::new(pattern.clone()));
    perlin_pattern.factor = 0.25;
    floor_material.pattern = Box::new(pattern.clone());
    sphere.material = sphere_material.clone();
    floor.material = floor_material.clone();

    world.objects.push(sphere.clone());

    sphere.transform = sphere
        .transform
        .multiply(&Matrix4::translation(2.5, 0., 0.));

    world.objects.push(sphere.clone());

    world.objects.push(floor);

    let mut camera = Camera::new(1400, 1000, PI / 4.);
    let from = point(0., 0., -5.);
    let to = point(0., 0., 0.);
    let up = point(0., 1., 0.);
    camera.transform = TransformationMatrix::new(&from, &to, &up);

    // let canvas = camera.render(&world);

    // let mut file = File::create("ray_cast.ppm")?;
    // file.write_all(&canvas.render_ppm().into_bytes())?;
    Ok(())
}
