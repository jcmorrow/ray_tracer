extern crate chrono;
extern crate noise;

use camera::Camera;
use canvas::Canvas;
use chrono::prelude::*;
use color::Color;
use dof::Dof;
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
use std::time::{Duration, SystemTime};
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
    let mut intersections = 0;
    let mut king = ObjParser::parse(&fs::read_to_string("fixtures/king.obj")?);
    let mut world = World::new();
    world.objects = Vec::new();

    let mut sphere = Shape::sphere();
    sphere.transform =
        Matrix4::translation(0., 0.25, 0.5).multiply(&Matrix4::scaling(0.3, 0.3, 0.3));
    // .multiply(&Matrix4::rotation_z(PI / 4.));
    let mut sphere2 = Shape::sphere();
    sphere2.transform = Matrix4::translation(1.25, 0.25, 7.0)
        .multiply(&Matrix4::scaling(0.3, 0.3, 0.3))
        .multiply(&Matrix4::rotation_y(PI / 2.));

    let mut sphere3 = Shape::sphere();
    sphere3.transform = Matrix4::translation(-1.4, 0.25, 7.0)
        .multiply(&Matrix4::scaling(0.3, 0.3, 0.3))
        .multiply(&Matrix4::rotation_y(-PI / 2.));
    let mut floor = Shape::plane();
    let mut wall = Shape::plane();
    wall.transform = Matrix4::translation(0., 0., 3.).multiply(&Matrix4::rotation_x(PI / 2.));
    // floor.transform = Matrix4::translation(0., -2., 0.);

    let mut king_material = Material::new();
    king_material.reflective = 0.0;
    king_material.pattern = Box::new(Solid::new(Color::white()));

    let mut sphere_material = Material::new();
    sphere_material.reflective = 0.4;
    let mut floor_material = Material::new();
    floor_material.reflective = 0.0;

    let mut pattern = Checker::new(Color::new(0.2, 0.65, 0.9), Color::white());
    let gradient = Gradient::new(Color::new(0.2, 0.65, 0.9), Color::white());
    sphere_material.pattern = Box::new(gradient.clone());
    pattern.transform = Matrix4::scaling(0.25, 0.25, 0.25);

    let mut perlin_pattern = Perlin::new(Box::new(pattern.clone()));
    perlin_pattern.factor = 0.25;
    floor_material.pattern = Box::new(pattern.clone());
    sphere.material = sphere_material.clone();
    sphere2.material = sphere_material.clone();
    sphere3.material = sphere_material.clone();
    floor.material = floor_material.clone();
    wall.material = floor_material.clone();

    world.objects.push(sphere);
    world.objects.push(sphere2);
    world.objects.push(sphere3);
    world.objects.push(floor);

    let mut camera = Camera::new(2000, 2000, PI / 8.);
    let mut from = point(0., 0.5, -2.);
    let to = point(0., 0.25, 0.5);
    let up = point(0., 1., 0.);
    camera.transform = TransformationMatrix::new(&from, &to, &up);

    let now = Local::now();
    let filename = format!("output/{}.ppm", now.format("%Y-%m-%d_%H-%M-%S"));
    let mut file = File::create(filename)?;

    let mut dof = Dof {
        camera,
        canvases: Vec::new(),
        from,
        takes: 8,
        to,
        up,
    };

    let canvas = dof.render(&world);

    file.write_all(&canvas.render_ppm().into_bytes())?;
    Ok(())
}
