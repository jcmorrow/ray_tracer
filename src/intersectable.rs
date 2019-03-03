use intersection::Intersection;
use point::point;
use point::vector;
use point::Point;
use ray::Ray;
use shape::Shape;
use std::f64::INFINITY;
use std::fmt::Debug;
use std::rc::Rc;
use utilities::EPSILON;

pub trait Intersectable: Debug + IntersectableClone {
    fn local_normal_at(&self, point: &Point) -> Point;
    fn add(&mut self, shape: Shape, owner: Rc<Shape>);
    fn local_intersect(&self, ray: &Ray, object: &Shape) -> Vec<Intersection>;
}

pub trait IntersectableClone {
    fn clone_box(&self) -> Box<Intersectable>;
}

impl<T> IntersectableClone for T
where
    T: 'static + Intersectable + Clone,
{
    fn clone_box(&self) -> Box<Intersectable> {
        Box::new(self.clone())
    }
}

impl Clone for Box<Intersectable> {
    fn clone(&self) -> Box<Intersectable> {
        self.clone_box()
    }
}

#[derive(Clone, Debug)]
pub struct Sphere {}

impl Intersectable for Sphere {
    fn local_normal_at(&self, local_point: &Point) -> Point {
        local_point.sub(&point(0., 0., 0.))
    }

    fn add(&mut self, _shape: Shape, _owner: Rc<Shape>) {}

    fn local_intersect(&self, ray: &Ray, object: &Shape) -> Vec<Intersection> {
        let shape_to_ray = ray.origin.sub(&point(0., 0., 0.));
        let a = ray.direction.dot(&ray.direction);
        let b = ray.direction.dot(&shape_to_ray) * 2.0;
        let c = shape_to_ray.dot(&shape_to_ray) - 1.;

        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0. {
            return Vec::new();
        } else {
            return Intersection::intersections(
                Intersection {
                    t: (-b - discriminant.sqrt()) / (2.0 * a),
                    object: object.clone(),
                },
                Intersection {
                    t: (-b + discriminant.sqrt()) / (2.0 * a),
                    object: object.clone(),
                },
            );
        }
    }
}

#[derive(Clone, Debug)]
pub struct Plane {}

impl Intersectable for Plane {
    fn local_normal_at(&self, _local_point: &Point) -> Point {
        point(0., 1., 0.)
    }

    fn add(&mut self, _shape: Shape, _owner: Rc<Shape>) {}

    fn local_intersect(&self, ray: &Ray, object: &Shape) -> Vec<Intersection> {
        if ray.direction.y.abs() < EPSILON {
            return Vec::new();
        }

        vec![Intersection {
            object: object.clone(),
            t: -ray.origin.y / ray.direction.y,
        }]
    }
}

#[derive(Clone, Debug)]
pub struct Cube {}

impl Cube {
    fn check_axis(&self, origin: f64, direction: f64) -> (f64, f64) {
        let tmin: f64;
        let tmax: f64;
        let tmin_numerator = -1. - origin;
        let tmax_numerator = 1. - origin;
        if direction.abs() >= EPSILON {
            tmin = tmin_numerator / direction;
            tmax = tmax_numerator / direction;
        } else {
            tmin = tmin_numerator * INFINITY;
            tmax = tmax_numerator * INFINITY;
        }
        if tmin > tmax {
            (tmax, tmin)
        } else {
            (tmin, tmax)
        }
    }
}

impl Intersectable for Cube {
    fn local_normal_at(&self, local_point: &Point) -> Point {
        let maxc = vec![
            local_point.x.abs(),
            local_point.y.abs(),
            local_point.z.abs(),
        ]
        .iter()
        .cloned()
        .fold(0. / 0., f64::max);

        if maxc == local_point.x.abs() {
            vector(local_point.x, 0., 0.)
        } else if maxc == local_point.y.abs() {
            vector(0., local_point.y, 0.)
        } else {
            vector(0., 0., local_point.z)
        }
    }

    fn add(&mut self, _shape: Shape, _owner: Rc<Shape>) {}

    fn local_intersect(&self, ray: &Ray, object: &Shape) -> Vec<Intersection> {
        let (xmin, xmax) = self.check_axis(ray.origin.x, ray.direction.x);
        let (ymin, ymax) = self.check_axis(ray.origin.y, ray.direction.y);
        let (zmin, zmax) = self.check_axis(ray.origin.z, ray.direction.z);

        let mins: Vec<f64> = vec![xmin, ymin, zmin];
        let maxs: Vec<f64> = vec![xmax, ymax, zmax];

        let tmin = mins.iter().cloned().fold(0. / 0., f64::max);
        let tmax = maxs.iter().cloned().fold(0. / 0., f64::min);

        if tmin > tmax {
            return Vec::new();
        }

        vec![
            Intersection {
                t: tmin,
                object: object.clone(),
            },
            Intersection {
                t: tmax,
                object: object.clone(),
            },
        ]
    }
}

#[derive(Clone, Debug)]
pub struct Triangle {
    pub e1: Point,
    pub e2: Point,
    pub normal: Point,
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
}

impl Triangle {
    pub fn new(p1: Point, p2: Point, p3: Point) -> Triangle {
        let e1 = p2.sub(&p1);
        let e2 = p3.sub(&p1);
        Triangle {
            p1,
            p2,
            p3,
            e1,
            e2,
            normal: e1.cross(&e2).normalize(),
        }
    }
}

impl Intersectable for Triangle {
    fn local_normal_at(&self, _local_point: &Point) -> Point {
        self.normal
    }

    fn add(&mut self, _shape: Shape, _owner: Rc<Shape>) {}

    fn local_intersect(&self, ray: &Ray, object: &Shape) -> Vec<Intersection> {
        let dir_cross_e2 = ray.direction.cross(&self.e2);
        let det = self.e1.dot(&dir_cross_e2);
        if det.abs() < EPSILON {
            return Vec::new();
        }

        let f = 1. / det;
        let p1_to_origin = ray.origin.sub(&self.p1);
        let u = f * p1_to_origin.dot(&dir_cross_e2);
        if u < 0. || u > 1. {
            return Vec::new();
        }

        let origin_cross_e1 = p1_to_origin.cross(&self.e1);
        let v = f * ray.direction.dot(&origin_cross_e1);

        if v < 0. || (u + v) > 1. {
            return Vec::new();
        }

        let t = f * self.e2.dot(&origin_cross_e1);

        vec![Intersection {
            object: object.clone(),
            t,
        }]
    }
}

#[derive(Clone, Debug)]
pub struct Group {
    children: Vec<Shape>,
}

impl Group {
    pub fn new() -> Group {
        Group {
            children: Vec::new(),
        }
    }
}

impl Intersectable for Group {
    fn local_normal_at(&self, _local_point: &Point) -> Point {
        vector(1., 0., 0.)
    }

    fn add(&mut self, mut shape: Shape, owner: Rc<Shape>) {
        shape.parent = Some(owner);
        self.children.push(shape);
    }

    fn local_intersect(&self, ray: &Ray, _object: &Shape) -> Vec<Intersection> {
        let mut intersects: Vec<Intersection> = Vec::new();
        for obj in &self.children {
            intersects.extend(ray.intersect(obj));
        }
        intersects
    }
}

#[cfg(test)]
mod tests {
    use intersectable::Group;
    use intersectable::*;
    use material::Material;
    use matrix::Matrix4;
    use matrix::IDENTITY_MATRIX;
    use std::f64::consts::PI;

    #[test]
    fn test_new_triangle() {
        let s = Triangle::new(point(0., 1., 0.), point(-1., 0., 0.), point(1., 0., 0.));

        assert_eq!(s.e1, vector(-1., -1., 0.));
        assert_eq!(s.e2, vector(1., -1., 0.));
        assert_eq!(s.normal, vector(0., 0., 1.));
    }

    #[test]
    fn test_group_intersect_misses() {
        let g = Group::new();
        let s = Shape {
            parent: None,
            intersectable: Box::new(g),
            material: Material::new(),
            transform: IDENTITY_MATRIX,
        };
        let ray = Ray {
            origin: point(0., 0., 0.),
            direction: vector(0., 0., 1.),
        };

        assert_eq!(ray.intersect(&s).len(), 0);
    }

    #[test]
    fn test_group_intersect_hits() {
        let mut g = Shape::group();
        let s1 = Shape::sphere();
        let mut s2 = Shape::sphere();
        s2.transform = Matrix4::translation(0., 0., -3.);
        let mut s3 = Shape::sphere();
        s3.transform = Matrix4::translation(5., 0., 0.);
        let ray = Ray {
            origin: point(0., 0., -5.),
            direction: vector(0., 0., 1.),
        };
        let f = Shape::group();

        Shape::add(&mut *g, s1, f);
        // g.add(s2, g.clone());
        // g.add(s3, g.clone());

        assert_eq!(ray.intersect(&g).len(), 4);
    }

    //     #[test]
    //     fn test_group_local_to_world_space() {
    //         let group = Shape::group();
    //         let mut s = Shape::sphere();
    //         s.transform = Matrix4::scaling(2., 2., 2.);
    //         let mut s2 = Shape::group();
    //         s2.transform = Matrix4::rotation_y(PI / 2.);
    //         Shape::add(s2, s);
    //         let mut s1 = Shape::group();
    //         s1.transform = Matrix4::rotation_y(PI / 2.);
    //         Shape::add(s1, s2);

    //         assert_eq!(
    //             s1.world_to_object(&point(-2., 0., -10.)),
    //             point(0., 0., -1.)
    //         );
    //     }
}
