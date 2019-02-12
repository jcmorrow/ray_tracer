use std::f64::consts::PI;
use std::fs::File;
use std::io::prelude::*;

mod canvas;
mod color;
mod matrix;
mod point;
mod utilities;

fn write_blob(c: &mut canvas::Canvas, x: f64, y: f64, color: &color::Color) {
    let row = utilities::clamp(x, 0.0, c.width as f64) as usize;
    let col = utilities::clamp(y, 0.0, c.width as f64) as usize;

    c.write_pixel(row, col, &color);
    c.write_pixel(row + 1, col, &color);
    c.write_pixel(row, col + 1, &color);
    c.write_pixel(row + 1, col + 1, &color);
    c.write_pixel(row + 2, col, &color);
    c.write_pixel(row, col + 2, &color);
    c.write_pixel(row + 2, col + 2, &color);
}

fn main() -> std::io::Result<()> {
    let width = 1000;
    let mut canvas = canvas::Canvas::empty(width, width);
    let color = color::Color::new(1.0, 1.0, 1.0);

    let position = point::point(10.0, 0.0, 0.0);

    let mut hours: Vec<point::Point> = vec![position; 12];

    let rotation_z = matrix::Matrix4::rotation_z(-PI / 6.0);
    let translation = matrix::Matrix4::translation(500.0, 500.0, 0.0);
    let scale = matrix::Matrix4::scaling(25.0, 25.0, 25.0);

    for p in 1..12 {
        hours[p] = rotation_z.multiply_point(&hours[p - 1]);
    }

    for p in hours {
        let point = translation.multiply(&scale).multiply_point(&p);
        write_blob(&mut canvas, point.x, point.y, &color);
    }

    let mut file = File::create("trajectory.ppm")?;
    file.write_all(&canvas.render_ppm().into_bytes())?;
    Ok(())
}
