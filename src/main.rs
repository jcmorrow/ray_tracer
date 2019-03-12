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

    let mut sphere = Shape::sphere();
    Arc::get_mut(&mut sphere).unwrap().transform =
        Matrix4::translation(0., 0.25, 0.5).multiply(&Matrix4::scaling(0.3, 0.3, 0.3));
    let mut sphere2 = Shape::sphere();
    Arc::get_mut(&mut sphere2).unwrap().transform = Matrix4::translation(-0.75, 0.25, 0.)
        .multiply(&Matrix4::scaling(0.3, 0.3, 0.3))
        .multiply(&Matrix4::rotation_y(PI / 2.));

    let mut sphere3 = Shape::sphere();
    Arc::get_mut(&mut sphere3).unwrap().transform = Matrix4::translation(0.75, 0.25, 0.)
        .multiply(&Matrix4::scaling(0.3, 0.3, 0.3))
        .multiply(&Matrix4::rotation_y(-PI / 2.));
    let mut floor = Shape::plane();
    // let mut wall = Shape::plane();
    Arc::get_mut(&mut floor).unwrap().transform = Matrix4::translation(0., -0.01, 0.);

    let mut sphere_material = Material::new();
    sphere_material.transparency = 0.1;
    sphere_material.refractive_index = 1.5;
    let mut floor_material = Material::new();
    floor_material.reflective = 0.8;

    Arc::get_mut(&mut sphere).unwrap().material = sphere_material.clone();
    Arc::get_mut(&mut sphere2).unwrap().material = sphere_material.clone();
    Arc::get_mut(&mut sphere3).unwrap().material = sphere_material.clone();
    Arc::get_mut(&mut floor).unwrap().material = floor_material;
    // Arc::get_mut(&mut wall).unwrap().material = floor_material;

    world.objects.push(sphere);
    world.objects.push(sphere2);
    world.objects.push(sphere3);
    world.objects.push(floor);

    let mut camera = Camera::new(600, 600, PI / 8.);
    let from = point(0., 2.0, -2.);
    let to = point(0., 0.07, 0.5);
    let up = point(0., 1., 0.);
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
