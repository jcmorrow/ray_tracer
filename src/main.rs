use std::fs::File;
use std::io::prelude::*;

mod canvas;
mod color;
mod matrix;
mod point;
mod utilities;

struct Projectile {
    position: point::Point,
    velocity: point::Point,
}

struct Environment {
    gravity: point::Point,
    wind: point::Point,
}

impl Projectile {
    fn tick(&self, env: &Environment) -> Projectile {
        Projectile {
            position: self.position.add(&self.velocity),
            velocity: self.velocity.add(&env.gravity).add(&env.wind),
        }
    }
}

fn main() -> std::io::Result<()> {
    let width = 1000;
    let mut canvas = canvas::Canvas::empty(width, width);
    let color = color::Color::new(1.0, 0.0, 0.0);
    let mut projectile = Projectile {
        position: point::point(0.0, 1.0, 0.0),
        velocity: point::vector(5.0, 10.0, 0.0),
    };

    let environment = Environment {
        gravity: point::vector(0.0, -0.1, 0.0),
        wind: point::vector(-0.01, 0.0, 0.0),
    };

    while projectile.position.y > 0.0 {
        projectile = projectile.tick(&environment);
        canvas.write_pixel(
            utilities::clamp(projectile.position.x, 0.0, width as f64) as usize,
            utilities::clamp(width as f64 - projectile.position.y, 0.0, width as f64) as usize,
            &color,
        );
    }
    let mut file = File::create("trajectory.ppm")?;
    file.write_all(&canvas.render_ppm().into_bytes())?;
    Ok(())
}
