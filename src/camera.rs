use canvas::Canvas;
use matrix::Matrix4;
use matrix::IDENTITY_MATRIX;
use point::point;
use ray::Ray;
use world::World;

pub struct Camera {
    field_of_view: f64,
    half_height: f64,
    half_width: f64,
    hsize: usize,
    pub transform: Matrix4,
    vsize: usize,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Camera {
        let half_height: f64;
        let half_width: f64;
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;

        if aspect > 1.0 {
            half_height = half_view / aspect;
            half_width = half_view;
        } else {
            half_height = half_view;
            half_width = half_view * aspect;
        }

        return Camera {
            field_of_view,
            half_height,
            half_width,
            hsize,
            transform: IDENTITY_MATRIX,
            vsize,
        };
    }

    pub fn pixel_size(&self) -> f64 {
        self.half_width * 2.0 / self.hsize as f64
    }

    pub fn ray_for_pixel(&self, h: usize, v: usize) -> Ray {
        let x_offset = (h as f64 + 0.5) * self.pixel_size();
        let y_offset = (v as f64 + 0.5) * self.pixel_size();
        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;
        let inverse = self.transform.inverse();
        let pixel = inverse.multiply_point(&point(world_x, world_y, -1.0));
        let origin = inverse.multiply_point(&point(0.0, 0.0, 0.0));
        return Ray {
            origin,
            direction: (pixel.sub(&origin)).normalize(),
        };
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut canvas = Canvas::empty(self.hsize as i64, self.vsize as i64);

        for x in 0..self.hsize - 1 {
            for y in 0..self.vsize - 1 {
                canvas.write_pixel(x, y, &world.color_at(&self.ray_for_pixel(x, y), 5));
            }
        }

        canvas
    }
}

#[cfg(test)]
mod tests {
    use camera::Camera;
    use color::Color;
    use matrix::Matrix4;
    use matrix::IDENTITY_MATRIX;
    use point::point;
    use point::vector;
    use std::f64::consts::PI;
    use transformation_matrix::TransformationMatrix;
    use utilities::equal;
    use world::World;

    #[test]
    fn test_camera_new() {
        let camera = Camera::new(160, 120, PI / 2.0);

        assert_eq!(camera.hsize, 160);
        assert_eq!(camera.vsize, 120);
        assert_eq!(camera.field_of_view, PI / 2.0);
        assert_eq!(camera.transform, IDENTITY_MATRIX);
    }

    #[test]
    fn test_camera_pixel_size() {
        let camera = Camera::new(200, 125, PI / 2.0);

        assert!(equal(camera.pixel_size(), 0.01));
    }

    #[test]
    fn test_camera_ray_for_pixel_1() {
        let camera = Camera::new(201, 101, PI / 2.0);
        let r = camera.ray_for_pixel(100, 50);

        assert_eq!(r.origin, point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_camera_ray_for_pixel_2() {
        let camera = Camera::new(201, 101, PI / 2.0);
        let r = camera.ray_for_pixel(0, 0);

        assert_eq!(r.origin, point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, vector(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn test_camera_ray_for_pixel_3() {
        let mut camera = Camera::new(201, 101, PI / 2.0);
        camera.transform =
            Matrix4::rotation_y(PI / 4.0).multiply(&Matrix4::translation(0.0, -2.0, 5.0));
        let r = camera.ray_for_pixel(100, 50);

        assert_eq!(r.origin, point(0.0, 2.0, -5.0));
        assert_eq!(
            r.direction,
            vector(2_f64.sqrt() / 2.0, 0.0, -2_f64.sqrt() / 2.0)
        );
    }

    #[test]
    fn test_world_with_camera() {
        let world = World::new();
        let mut camera = Camera::new(11, 11, PI / 2.0);

        let from = point(0.0, 0.0, -5.0);
        let to = point(0.0, 0.0, 0.0);
        let up = point(0.0, 1.0, 0.0);
        camera.transform = TransformationMatrix::new(&from, &to, &up);

        let image = camera.render(&world);
        assert_eq!(image.pixel_at(5, 5), Color::new(0.38066, 0.47583, 0.2855));
    }
}
