use intersectable::*;
use material::Material;
use matrix::Matrix4;
use matrix::IDENTITY_MATRIX;
use point::Point;

#[derive(Debug, Clone)]
pub struct Shape {
    pub transform: Matrix4,
    pub material: Material,
    pub intersectable: Box<Intersectable>,
}

impl Shape {
    pub fn sphere() -> Shape {
        Shape {
            transform: IDENTITY_MATRIX,
            material: Material::new(),
            intersectable: Box::new(Sphere {}),
        }
    }

    pub fn plane() -> Shape {
        Shape {
            transform: IDENTITY_MATRIX,
            material: Material::new(),
            intersectable: Box::new(Plane {}),
        }
    }

    pub fn cube() -> Shape {
        Shape {
            transform: IDENTITY_MATRIX,
            material: Material::new(),
            intersectable: Box::new(Cube {}),
        }
    }

    pub fn cylinder() -> Shape {
        Shape {
            transform: IDENTITY_MATRIX,
            material: Material::new(),
            intersectable: Box::new(Cylinder {
                minimum: None,
                maximum: None,
                closed: true,
            }),
        }
    }

    pub fn normal_at(&self, world_point: &Point) -> Point {
        let local_point = self.transform.inverse().multiply_point(&world_point);
        let local_normal = self.intersectable.local_normal_at(&local_point);
        let mut world_normal = self
            .transform
            .inverse()
            .transpose()
            .multiply_point(&local_normal);
        world_normal.w = 0.;
        world_normal.normalize()
    }
}

impl PartialEq for Shape {
    fn eq(&self, other: &Shape) -> bool {
        self.transform.equal(&other.transform) && self.material.equal(&self.material)
    }
}

#[cfg(test)]
mod tests {
    use intersectable::{Cylinder, Sphere};
    use material::Material;
    use matrix::Matrix4;
    use matrix::IDENTITY_MATRIX;
    use point::{point, vector, Point};
    use ray::Ray;
    use shape::Shape;
    use std::f64::consts::PI;

    #[test]
    fn test_shape() {
        let s = Shape::sphere();

        assert_eq!(s.transform, IDENTITY_MATRIX);
    }

    #[test]
    fn test_shape_with_non_default_transform() {
        let t = Matrix4::translation(2., 3., 4.);
        let s = Shape {
            transform: t,
            material: Material::new(),
            intersectable: Box::new(Sphere {}),
        };

        assert_eq!(s.transform, t);
    }

    #[test]
    fn test_shape_normal_at() {
        let s = Shape::sphere();

        assert!(s.normal_at(&point(1., 0., 0.)).equal(&vector(1., 0., 0.)));
        assert!(s.normal_at(&point(0., 1., 0.)).equal(&vector(0., 1., 0.)));
        assert!(s.normal_at(&point(0., 0., 1.)).equal(&vector(0., 0., 1.)));
        let sqrt_3_over_3 = 3.0_f64.sqrt() / 3.;
        let p = point(sqrt_3_over_3, sqrt_3_over_3, sqrt_3_over_3);
        let v = vector(sqrt_3_over_3, sqrt_3_over_3, sqrt_3_over_3);
        assert!(s.normal_at(&p).equal(&v));
        assert!(v.equal(&v.normalize()));
    }

    #[test]
    fn test_shape_normal_at_with_transformation() {
        let s = Shape {
            transform: Matrix4::translation(0., 1., 0.),
            material: Material::new(),
            intersectable: Box::new(Sphere {}),
        };

        assert!(s
            .normal_at(&point(0., 1.70711, -0.70711))
            .equal(&vector(0., 0.70711, -0.70711)));

        let s = Shape {
            intersectable: Box::new(Sphere {}),
            transform: Matrix4::scaling(1., 0.5, 1.).multiply(&Matrix4::rotation_z(PI / 5.)),
            material: Material::new(),
        };

        assert!(s
            .normal_at(&point(0., 2.0_f64.sqrt() / 2., -2.0_f64.sqrt() / 2.))
            .equal(&vector(0., 0.97014, -0.24254)));
    }

    #[test]
    fn test_plane_normal_at() {
        let s = Shape::plane();

        assert!(s.normal_at(&point(0., 0., 0.)).eq(&point(0., 1., 0.)));
    }

    #[test]
    fn test_plane_intersection() {
        let s = Shape::plane();
        let r = Ray {
            origin: point(0., 10., 0.),
            direction: vector(0., 0., 1.),
        };

        assert_eq!(r.intersect(&s).len(), 0);
    }

    #[test]
    fn test_plane_coplanar() {
        let s = Shape::plane();
        let r = Ray {
            origin: point(0., 0., 0.),
            direction: vector(0., 0., 1.),
        };

        assert_eq!(r.intersect(&s).len(), 0);
    }

    #[test]
    fn test_plane_does_intersect() {
        let s = Shape::plane();
        let r = Ray {
            origin: point(0., 1., 0.),
            direction: vector(0., -1., 0.),
        };

        assert_eq!(r.intersect(&s).len(), 1);
        assert_eq!(r.intersect(&s)[0].t, 1.);
        assert_eq!(r.intersect(&s)[0].object, s);
    }

    #[test]
    fn test_cube_intersection() {
        let s = Shape::cube();
        let negative_x = Ray {
            origin: point(5., 0.5, 0.),
            direction: vector(-1., 0., 0.),
        };
        let positive_x = Ray {
            origin: point(-5., 0.5, 0.),
            direction: vector(1., 0., 0.),
        };
        let negative_y = Ray {
            origin: point(0.5, -5., 0.),
            direction: vector(0., 1., 0.),
        };
        let positive_y = Ray {
            origin: point(0.5, 5., 0.),
            direction: vector(0., -1., 0.),
        };
        let negative_z = Ray {
            origin: point(0.5, 0., -5.),
            direction: vector(0., 0., 1.),
        };
        let positive_z = Ray {
            origin: point(0.5, 0., 5.),
            direction: vector(0., 0., -1.),
        };
        let inside = Ray {
            origin: point(0., 0.5, 0.),
            direction: vector(0., 0., 1.),
        };

        let positive_x_intersections = positive_x.intersect(&s);
        assert_eq!(positive_x_intersections.len(), 2);
        assert_eq!(positive_x_intersections[0].t, 4.);
        assert_eq!(positive_x_intersections[0].object, s);
        assert_eq!(positive_x_intersections[1].t, 6.);
        assert_eq!(positive_x_intersections[1].object, s);
        let negative_x_intersections = negative_x.intersect(&s);
        assert_eq!(negative_x_intersections.len(), 2);
        assert_eq!(negative_x_intersections[0].t, 4.);
        assert_eq!(negative_x_intersections[0].object, s);
        assert_eq!(negative_x_intersections[1].t, 6.);
        assert_eq!(negative_x_intersections[1].object, s);
        let positive_y_intersections = positive_y.intersect(&s);
        assert_eq!(positive_y_intersections.len(), 2);
        assert_eq!(positive_y_intersections[0].t, 4.);
        assert_eq!(positive_y_intersections[0].object, s);
        assert_eq!(positive_y_intersections[1].t, 6.);
        assert_eq!(positive_y_intersections[1].object, s);
        let negative_y_intersections = negative_y.intersect(&s);
        assert_eq!(negative_y_intersections.len(), 2);
        assert_eq!(negative_y_intersections[0].t, 4.);
        assert_eq!(negative_y_intersections[0].object, s);
        assert_eq!(negative_y_intersections[1].t, 6.);
        assert_eq!(negative_y_intersections[1].object, s);
        let positive_z_intersections = positive_z.intersect(&s);
        assert_eq!(positive_z_intersections.len(), 2);
        assert_eq!(positive_z_intersections[0].t, 4.);
        assert_eq!(positive_z_intersections[0].object, s);
        assert_eq!(positive_z_intersections[1].t, 6.);
        assert_eq!(positive_z_intersections[1].object, s);
        let negative_z_intersections = negative_z.intersect(&s);
        assert_eq!(negative_z_intersections.len(), 2);
        assert_eq!(negative_z_intersections[0].t, 4.);
        assert_eq!(negative_z_intersections[0].object, s);
        assert_eq!(negative_z_intersections[1].t, 6.);
        assert_eq!(negative_z_intersections[1].object, s);
        let inside_intersections = inside.intersect(&s);
        assert_eq!(inside_intersections.len(), 2);
        assert_eq!(inside_intersections[0].t, -1.);
        assert_eq!(inside_intersections[0].object, s);
        assert_eq!(inside_intersections[1].t, 1.);
        assert_eq!(inside_intersections[1].object, s);
    }

    #[test]
    fn test_cube_intersection_misses() {
        let s = Shape::cube();
        let ray = Ray {
            origin: point(-2., 0., 0.),
            direction: vector(0.2673, 0.5345, 0.8018),
        };

        assert_eq!(ray.intersect(&s).len(), 0);
    }

    #[test]
    fn test_cylinder_intersection_misses() {
        let mut s = Shape::cylinder();
        let cylinder = Cylinder {
            minimum: None,
            maximum: None,
            closed: false,
        };
        s.intersectable = Box::new(cylinder);
        let a = Ray {
            origin: point(1., 0., 0.),
            direction: vector(0., 1., 0.),
        };
        let b = Ray {
            origin: point(0., 0., 0.),
            direction: vector(0., 1., 0.),
        };
        let c = Ray {
            origin: point(0., 0., -5.),
            direction: vector(1., 1., 1.),
        };

        assert_eq!(a.intersect(&s).len(), 0);
        assert_eq!(b.intersect(&s).len(), 0);
        assert_eq!(c.intersect(&s).len(), 0);
    }

    #[test]
    fn test_cylinder_intersection_hits() {
        let mut s = Shape::cylinder();
        let cylinder = Cylinder {
            minimum: None,
            maximum: None,
            closed: false,
        };
        s.intersectable = Box::new(cylinder);
        let inputs: Vec<(Point, Point, f64, f64)> = vec![
            (point(1., 0., -5.), vector(0., 0., 1.), 5., 5.),
            (point(0., 0., -5.), vector(0., 0., 1.), 4., 6.),
            (
                point(0.5, 0., -5.),
                vector(0.1, 1., 1.),
                6.80798191702732,
                7.088723439378861,
            ),
        ];

        for input in inputs {
            let ray = Ray {
                origin: input.0,
                direction: input.1.normalize(),
            };

            assert_eq!(ray.intersect(&s).len(), 2);
            assert_eq!(ray.intersect(&s)[0].t, input.2);
            assert_eq!(ray.intersect(&s)[1].t, input.3);
        }
    }

    #[test]
    fn test_cylinder_intersection_normals() {
        let s = Shape::cylinder();
        let inputs: Vec<(Point, Point)> = vec![
            (point(1., 0., 0.), vector(1., 0., 0.)),
            (point(0., 5., -1.), vector(0., 0., -1.)),
            (point(0., -2., 1.), vector(0., 0., 1.)),
            (point(-1., 1., 0.), vector(-1., 0., 0.)),
        ];

        for input in inputs {
            assert_eq!(s.normal_at(&input.0), input.1);
        }
    }

    #[test]
    fn test_cylinder_intersection_truncated() {
        let mut s = Shape::cylinder();
        let cylinder = Cylinder {
            minimum: Some(1.),
            maximum: Some(2.),
            closed: false,
        };
        s.intersectable = Box::new(cylinder);
        let inputs: Vec<(Point, Point, usize)> = vec![
            (point(0., 1.5, 0.), vector(0.1, 1., 0.), 0),
            (point(0., 3., -5.), vector(0., 0., 1.), 0),
            (point(0., 0., -5.), vector(0., 0., 1.), 0),
            (point(0., 2., -5.), vector(0., 0., 1.), 0),
            (point(0., 1., -5.), vector(0., 0., 1.), 0),
            (point(0., 1.5, -2.), vector(0., 0., 1.), 0),
        ];

        for input in inputs {
            let ray = Ray {
                origin: input.0,
                direction: input.1.normalize(),
            };
            assert_eq!(ray.intersect(&s).len(), input.2);
        }
    }

    #[test]
    fn test_cylinder_intersection_capped() {
        let mut s = Shape::cylinder();
        let cylinder = Cylinder {
            minimum: Some(1.),
            maximum: Some(2.),
            closed: true,
        };
        s.intersectable = Box::new(cylinder);
        let inputs: Vec<(Point, Point, usize)> = vec![
            (point(0., 3., 0.), vector(0., -1., 0.), 2),
            (point(0., 3., -2.), vector(0., -1., 2.), 2),
            (point(0., 4., -2.), vector(0., -1., 1.), 2),
            (point(0., 0., -2.), vector(0., 1., 2.), 2),
            (point(0., -1., -2.), vector(0., 1., 1.), 2),
        ];

        for input in inputs {
            println!("{:?}", input);
            let ray = Ray {
                origin: input.0,
                direction: input.1.normalize(),
            };
            assert_eq!(ray.intersect(&s).len(), input.2);
        }
    }
}
