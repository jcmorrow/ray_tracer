use camera::Camera;
use color::Color;
use intersection::Intersection;
use material::Material;
use matrix::Matrix4;
use point::point;
use point_light::PointLight;
use sphere::Sphere;
use transformation_matrix::TransformationMatrix;
use world::World;

use std::f64::consts::PI;
use std::fs::File;
use std::io::prelude::*;

mod camera;
mod canvas;
mod color;
mod intersection;
mod material;
mod matrix;
mod point;
mod point_light;
mod ray;
mod sphere;
mod transformation_matrix;
mod utilities;
mod world;

fn main() -> std::io::Result<()> {
    let mut world = World::new();

    let mut material = Material::new();
    material.color = Color::new(1.0, 0.9, 0.9);

    let mut floor = Sphere::new();
    floor.transform =
        Matrix4::translation(0.0, -1.0, 0.0).multiply(&Matrix4::scaling(10.0, 0.01, 10.0));
    floor.material = material;
    world.objects.push(floor);

    let mut left_wall = Sphere::new();
    left_wall.transform = Matrix4::translation(0.0, 0.0, 5.0)
        .multiply(&Matrix4::rotation_y(-PI / 4.0))
        .multiply(&Matrix4::rotation_x(PI / 2.0))
        .multiply(&Matrix4::scaling(10.0, 0.01, 10.0));
    left_wall.material = material;
    world.objects.push(left_wall);

    let mut right_wall = Sphere::new();
    right_wall.transform = Matrix4::translation(0.0, 0.0, 5.0)
        .multiply(&Matrix4::rotation_y(PI / 4.0))
        .multiply(&Matrix4::rotation_x(PI / 2.0))
        .multiply(&Matrix4::scaling(10.0, 0.01, 10.0));
    right_wall.material = material;
    world.objects.push(right_wall);

    let mut camera = Camera::new(200, 100, PI / 3.0);
    let from = point(0.0, 1.5, -5.0);
    let to = point(0.0, 1.0, 0.0);
    let up = point(0.0, 1.0, 0.0);
    camera.transform = TransformationMatrix::new(&from, &to, &up);

    let canvas = camera.render(&world);

    let mut file = File::create("ray_cast.ppm")?;
    file.write_all(&canvas.render_ppm().into_bytes())?;
    Ok(())
}
