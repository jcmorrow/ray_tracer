extern crate chrono;
extern crate noise;
extern crate rayon;

use camera::Camera;
use chrono::prelude::*;
use color::Color;
use dof::Dof;
use material::Material;
use matrix::Matrix4;
// use obj_parser::ObjParser;
use patternable::*;
use point::point;
use shape::Shape;
use std::f64::consts::PI;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;
use transformation_matrix::TransformationMatrix;
use world::World;

mod bounds;
mod camera;
mod canvas;
mod color;
mod dof;
mod intersectable;
mod intersection;
mod material;
mod matrix;
// mod obj_parser;
mod patternable;
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

    let mut sphere = Shape::glass_sphere();
    Arc::get_mut(&mut sphere).unwrap().transform = Matrix4::translation(0., 0.5, 0.);
    let mut floor = Shape::plane();
    // let mut wall = Shape::plane();
    Arc::get_mut(&mut floor).unwrap().transform = Matrix4::translation(0., 0.1, 0.);

    let pattern = Patternable::checker(Color::black(), Color::white());
    // let mut sphere_material = Material::new();
    // sphere_material.transparency = 1.;
    // sphere_material.refractive_index = 1.5;
    // sphere_material.ambient = 1.;
    // sphere_material.pattern = Patternable::solid(Color::new(0.1, 0.1, 0.1));
    let mut floor_material = Material::new();
    floor_material.pattern = pattern;
    floor_material.pattern.transform = Matrix4::scaling(0.2, 0.2, 0.2);

    // Arc::get_mut(&mut sphere).unwrap().material = sphere_material.clone();
    Arc::get_mut(&mut floor).unwrap().material = floor_material;
    // Arc::get_mut(&mut wall).unwrap().material = floor_material;

    world.objects.push(sphere);
    world.objects.push(floor);

    let mut camera = Camera::new(600, 600, PI / 8.);
    let from = point(0., 3., -2.);
    let to = point(0., 0.07, 0.5);
    let up = point(0., 0., 1.);
    camera.transform = TransformationMatrix::new(&from, &to, &up);

    let now = Local::now();
    let filename = format!("output/{}.ppm", now.format("%Y-%m-%d_%H-%M-%S"));
    let mut file = File::create(filename)?;

    let mut dof = Dof {
        camera,
        canvases: Vec::new(),
        from,
        takes: 2,
        to,
        up,
    };

    let canvas = dof.render(&world);

    file.write_all(&canvas.render_ppm().into_bytes())?;
    Ok(())
}
