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
use pattern::*;
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
mod pattern;
mod point;
mod point_light;
mod ray;
mod shape;
mod transformation_matrix;
mod utilities;
mod world;

fn main() -> std::io::Result<()> {
    // let mut king = Arc::new(ObjParser::parse(&fs::read_to_string("fixtures/king.obj")?));
    let mut world = World::new();
    world.objects = Vec::new();

    let mut sphere = Shape::sphere();
    Arc::get_mut(&mut sphere).unwrap().transform =
        Matrix4::translation(0., 0.25, 0.5).multiply(&Matrix4::scaling(0.3, 0.3, 0.3));
    // .multiply(&Matrix4::rotation_z(PI / 4.));
    let mut sphere2 = Shape::sphere();
    Arc::get_mut(&mut sphere2).unwrap().transform = Matrix4::translation(1.25, 0.25, 7.0)
        .multiply(&Matrix4::scaling(0.3, 0.3, 0.3))
        .multiply(&Matrix4::rotation_y(PI / 2.));

    let mut sphere3 = Shape::sphere();
    Arc::get_mut(&mut sphere3).unwrap().transform = Matrix4::translation(-1.4, 0.25, 7.0)
        .multiply(&Matrix4::scaling(0.3, 0.3, 0.3))
        .multiply(&Matrix4::rotation_y(-PI / 2.));
    let mut floor = Shape::plane();
    // let mut wall = Shape::plane();
    // Arc::get_mut(&mut wall).unwrap().transform =
    //     Matrix4::translation(0., 0., 3.).multiply(&Matrix4::rotation_x(PI / 2.));
    Arc::get_mut(&mut floor).unwrap().transform = Matrix4::translation(0., 0.25, 0.);

    let mut king_material = Material::new();
    king_material.reflective = 0.0;
    king_material.pattern = Solid::new(Color::white());

    let mut sphere_material = Material::new();
    sphere_material.reflective = 0.4;
    let mut floor_material = Material::new();
    floor_material.reflective = 0.0;

    let mut pattern = Checker::new(Color::new(0.9, 0.1, 0.1), Color::white());
    let mut pattern2 = Checker::new(Color::new(0.9, 0.1, 0.1), Color::white());
    let gradient = Gradient::new(Color::new(0.9, 0.1, 0.1), Color::white());
    sphere_material.pattern = Solid::new(Color::white());
    pattern.transform = Matrix4::scaling(0.25, 0.25, 0.25);
    pattern2.transform = Matrix4::scaling(0.25, 0.25, 0.25);

    let mut perlin_pattern = Perlin::new(Arc::new(pattern));
    perlin_pattern.factor = 0.25;
    floor_material.pattern = Solid::new(Color::white());
    Arc::get_mut(&mut sphere).unwrap().material = sphere_material;
    // Arc::get_mut(&mut sphere2).unwrap().material = sphere_material;
    // Arc::get_mut(&mut sphere3).unwrap().material = sphere_material;
    // Arc::get_mut(&mut floor).unwrap().material = floor_material;
    // Arc::get_mut(&mut wall).unwrap().material = floor_material;

    world.objects.push(sphere);
    world.objects.push(sphere2);
    world.objects.push(sphere3);
    world.objects.push(floor);

    let mut camera = Camera::new(100, 100, PI / 8.);
    let from = point(0., 0.5, -2.);
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
