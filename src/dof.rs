use camera::Camera;
use canvas::Canvas;
use matrix::Matrix4;
use point::Point;
use transformation_matrix::TransformationMatrix;
use world::World;

pub struct Dof {
    pub camera: Camera,
    pub takes: usize,
    pub canvases: Vec<Canvas>,
    pub from: Point,
    pub to: Point,
    pub up: Point,
}

impl Dof {
    pub fn render(&mut self, world: &World) -> Canvas {
        let mut i = 0;
        while i < self.takes {
            self.camera.transform = TransformationMatrix::new(
                &Matrix4::translation(0.0005 * i as f64, 0.0005 * i as f64, 0.)
                    .multiply_point(&self.from),
                &self.to,
                &self.up,
            );
            self.canvases.push(self.camera.render(&world));
            i = i + 1;
        }
        let mut final_canvas = Canvas::empty(self.camera.hsize as i64, self.camera.vsize as i64);

        for canvas in &self.canvases {
            for (i, pixel) in canvas.pixels.iter().enumerate() {
                final_canvas.pixels[i] = final_canvas.pixels[i].add(&pixel);
            }
        }

        let mut j = 0;
        while j < final_canvas.pixels.len() {
            let pixel = final_canvas.pixels[j];
            final_canvas.pixels[j] = pixel.divide(self.takes as f64);
            j = j + 1;
        }

        final_canvas
    }
}
