use bounds::Bounds;
use color::Color;
use intersectable::*;
use material::Material;
use matrix::Matrix4;
use matrix::IDENTITY_MATRIX;
use patternable::Patternable;
use point::Point;
use std::sync::Arc;

#[derive(Debug)]
pub struct Shape {
    pub parent: Option<Arc<Shape>>,
    pub transform: Matrix4,
    pub material: Material,
    pub intersectable: Intersectable,
}

impl Shape {
    pub fn sphere() -> Arc<Shape> {
        Arc::new(Shape {
            parent: None,
            transform: IDENTITY_MATRIX,
            material: Material::new(),
            intersectable: Intersectable::sphere(),
        })
    }

    pub fn glass_sphere() -> Arc<Shape> {
        let mut s = Shape {
            parent: None,
            transform: IDENTITY_MATRIX,
            material: Material::new(),
            intersectable: Intersectable::sphere(),
        };
        s.material.refractive_index = 1.0;
        s.material.transparency = 1.;
        s.material.specular = 1.;
        s.material.shininess = 300.;
        s.material.diffuse = 0.;
        s.material.pattern = Patternable::solid(Color::white());
        Arc::new(s)
    }

    pub fn plane() -> Arc<Shape> {
        Arc::new(Shape {
            parent: None,
            transform: IDENTITY_MATRIX,
            material: Material::new(),
            intersectable: Intersectable::plane(),
        })
    }

    pub fn cube() -> Arc<Shape> {
        Arc::new(Shape {
            parent: None,
            transform: IDENTITY_MATRIX,
            material: Material::new(),
            intersectable: Intersectable::cube(),
        })
    }

    pub fn triangle(a: Point, b: Point, c: Point) -> Arc<Shape> {
        Arc::new(Shape {
            parent: None,
            transform: IDENTITY_MATRIX,
            material: Material::new(),
            intersectable: Intersectable::triangle(a, b, c),
        })
    }

    pub fn group() -> Arc<Shape> {
        Arc::new(Shape {
            parent: None,
            transform: IDENTITY_MATRIX,
            material: Material::new(),
            intersectable: Intersectable::group(),
        })
    }

    pub fn add_group(mut group: Arc<Shape>, mut shape: Arc<Shape>) {
        Arc::get_mut(&mut shape).unwrap().parent = Some(group.clone());
        Arc::get_mut(&mut group)
            .unwrap()
            .intersectable
            .add(shape.clone());
    }

    pub fn add_shape(mut group: Arc<Shape>, mut shape: Arc<Shape>) {
        Arc::get_mut(&mut shape).unwrap().parent = Some(group.clone());
        Arc::get_mut(&mut group).unwrap().intersectable.add(shape);
    }

    pub fn normal_at(&self, world_point: &Point) -> Point {
        let local_point = self.transform.inverse().multiply_point(&world_point);
        let local_normal = self.intersectable.local_normal_at(&local_point);
        self.normal_to_world(&local_normal)
    }

    pub fn world_to_object(&self, world_point: &Point) -> Point {
        let point = match self.parent {
            Some(ref p) => p.world_to_object(world_point),
            None => *world_point,
        };
        self.transform.inverse().multiply_point(&point)
    }

    pub fn normal_to_world(&self, normal: &Point) -> Point {
        let mut local_normal = self.transform.inverse().transpose().multiply_point(&normal);
        local_normal.w = 0.;
        if let Some(ref p) = self.parent {
            p.normal_to_world(&local_normal).normalize()
        } else {
            local_normal.normalize()
        }
    }

    pub fn bounds(&self) -> Bounds {
        self.intersectable.bounds(self)
    }
}

impl PartialEq for Shape {
    fn eq(&self, other: &Shape) -> bool {
        self.transform.equal(&other.transform) && self.material.equal(&self.material)
    }
}

#[cfg(test)]
mod tests {
    use intersection::Intersection;
    use intersection::Precompute;
    use material::Material;
    use matrix::Matrix4;
    use matrix::IDENTITY_MATRIX;
    use point::point;
    use point::vector;
    use ray::Ray;
    use shape::*;
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
            parent: None,
            transform: t,
            material: Material::new(),
            intersectable: Intersectable::sphere(),
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
            parent: None,
            transform: Matrix4::translation(0., 1., 0.),
            material: Material::new(),
            intersectable: Intersectable::sphere(),
        };

        assert!(s
            .normal_at(&point(0., 1.70711, -0.70711))
            .equal(&vector(0., 0.70711, -0.70711)));

        let s = Shape {
            parent: None,
            intersectable: Intersectable::sphere(),
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

        assert_eq!(r.intersect(s).len(), 0);
    }

    #[test]
    fn test_plane_coplanar() {
        let s = Shape::plane();
        let r = Ray {
            origin: point(0., 0., 0.),
            direction: vector(0., 0., 1.),
        };

        assert_eq!(r.intersect(s).len(), 0);
    }

    #[test]
    fn test_plane_does_intersect() {
        let s = Shape::plane();
        let r = Ray {
            origin: point(0., 1., 0.),
            direction: vector(0., -1., 0.),
        };

        assert_eq!(r.intersect(s.clone()).len(), 1);
        assert_eq!(r.intersect(s.clone())[0].t, 1.);
        assert_eq!(r.intersect(s.clone())[0].object, s);
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

        let positive_x_intersections = positive_x.intersect(s.clone());
        assert_eq!(positive_x_intersections.len(), 2);
        assert_eq!(positive_x_intersections[0].t, 4.);
        assert_eq!(positive_x_intersections[0].object, s);
        assert_eq!(positive_x_intersections[1].t, 6.);
        assert_eq!(positive_x_intersections[1].object, s);
        let negative_x_intersections = negative_x.intersect(s.clone());
        assert_eq!(negative_x_intersections.len(), 2);
        assert_eq!(negative_x_intersections[0].t, 4.);
        assert_eq!(negative_x_intersections[0].object, s);
        assert_eq!(negative_x_intersections[1].t, 6.);
        assert_eq!(negative_x_intersections[1].object, s);
        let positive_y_intersections = positive_y.intersect(s.clone());
        assert_eq!(positive_y_intersections.len(), 2);
        assert_eq!(positive_y_intersections[0].t, 4.);
        assert_eq!(positive_y_intersections[0].object, s);
        assert_eq!(positive_y_intersections[1].t, 6.);
        assert_eq!(positive_y_intersections[1].object, s);
        let negative_y_intersections = negative_y.intersect(s.clone());
        assert_eq!(negative_y_intersections.len(), 2);
        assert_eq!(negative_y_intersections[0].t, 4.);
        assert_eq!(negative_y_intersections[0].object, s);
        assert_eq!(negative_y_intersections[1].t, 6.);
        assert_eq!(negative_y_intersections[1].object, s);
        let positive_z_intersections = positive_z.intersect(s.clone());
        assert_eq!(positive_z_intersections.len(), 2);
        assert_eq!(positive_z_intersections[0].t, 4.);
        assert_eq!(positive_z_intersections[0].object, s);
        assert_eq!(positive_z_intersections[1].t, 6.);
        assert_eq!(positive_z_intersections[1].object, s);
        let negative_z_intersections = negative_z.intersect(s.clone());
        assert_eq!(negative_z_intersections.len(), 2);
        assert_eq!(negative_z_intersections[0].t, 4.);
        assert_eq!(negative_z_intersections[0].object, s);
        assert_eq!(negative_z_intersections[1].t, 6.);
        assert_eq!(negative_z_intersections[1].object, s);
        let inside_intersections = inside.intersect(s.clone());
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

        assert_eq!(ray.intersect(s).len(), 0);
    }

    fn triangle() -> Arc<Shape> {
        Shape::triangle(point(0., 1., 0.), point(-1., 0., 0.), point(1., 0., 0.))
    }

    fn glass_sphere(t: Matrix4, r: f64) -> Arc<Shape> {
        let mut s = Shape::glass_sphere();
        Arc::get_mut(&mut s).unwrap().transform = t;
        Arc::get_mut(&mut s).unwrap().material.refractive_index = r;
        s
    }

    #[test]
    fn test_triangle_intersection_misses() {
        let t = triangle();
        let ray1 = Ray {
            origin: point(0., -1., -2.),
            direction: vector(0., 1., 0.),
        };
        let ray2 = Ray {
            origin: point(1., 1., -2.),
            direction: vector(0., 0., 1.),
        };
        let ray3 = Ray {
            origin: point(0., -1., -2.),
            direction: vector(0., 0., 1.),
        };

        assert_eq!(ray1.intersect(t.clone()).len(), 0);
        assert_eq!(ray2.intersect(t.clone()).len(), 0);
        assert_eq!(ray3.intersect(t.clone()).len(), 0);
    }

    //     #[test]
    //     fn test_triangle_intersection_hits() {
    //         let t = triangle();
    //         let ray = Ray {
    //             origin: point(0., 0.5, -2.),
    //             direction: vector(0., 0., 1.),
    //         };

    //         assert_eq!(ray.intersect(t.clone()).len(), 1);
    //         assert_eq!(ray.intersect(t.clone())[0].t, 2.);
    //     }

    #[test]
    fn test_glass_sphere() {
        let a = glass_sphere(Matrix4::scaling(2., 2., 2.), 1.5);
        let b = glass_sphere(Matrix4::translation(0., 0., -0.25), 2.);
        let c = glass_sphere(Matrix4::translation(0., 0., 0.25), 2.5);

        let r = Ray {
            origin: point(0., 0., -4.),
            direction: vector(0., 0., 1.),
        };

        let xs: Vec<Intersection> = vec![
            Intersection {
                object: a.clone(),
                t: 2.,
            },
            Intersection {
                object: b.clone(),
                t: 2.75,
            },
            Intersection {
                object: c.clone(),
                t: 3.25,
            },
            Intersection {
                object: b.clone(),
                t: 4.75,
            },
            Intersection {
                object: c.clone(),
                t: 5.25,
            },
            Intersection {
                object: a.clone(),
                t: 6.,
            },
        ];
        let prepared_xs: Vec<Precompute> = xs
            .iter()
            .map(|int| int.precompute(&r, xs.clone()))
            .collect();

        let ns: Vec<(f64, f64)> = prepared_xs.iter().map(|x| (x.n1, x.n2)).collect();

        let expectations: Vec<(usize, f64, f64)> = vec![
            (0, 1.0, 1.5),
            (1, 1.5, 2.0),
            (2, 2.0, 2.5),
            (3, 2.5, 2.5),
            (4, 2.5, 1.5),
            (5, 1.5, 1.0),
        ];

        for e in expectations {
            assert_eq!(prepared_xs[e.0].n1, e.1);
            assert_eq!(prepared_xs[e.0].n2, e.2);
        }
    }
}
