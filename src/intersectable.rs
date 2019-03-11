use bounds::Bounds;
use intersection::Intersection;
use point::{bounds, point, vector, Point};
use ray::Ray;
use shape::Shape;
use std::f64::INFINITY;
use std::sync::Arc;
use utilities::EPSILON;
use utilities::{max, min};

#[derive(Debug, Clone)]
pub enum IntersectableType {
    Cube,
    Group,
    Plane,
    Sphere,
    Triangle,
}

#[derive(Debug, Clone)]
pub struct Intersectable {
    intersectable_type: IntersectableType,
    pub e1: Point,
    pub e2: Point,
    pub normal: Point,
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
    children: Vec<Arc<Shape>>,
}

impl Intersectable {
    pub fn sphere() -> Intersectable {
        Intersectable {
            children: Vec::new(),
            e1: point(0., 0., 0.),
            e2: point(0., 0., 0.),
            intersectable_type: IntersectableType::Sphere,
            normal: point(0., 0., 0.),
            p1: point(0., 0., 0.),
            p2: point(0., 0., 0.),
            p3: point(0., 0., 0.),
        }
    }

    pub fn plane() -> Intersectable {
        Intersectable {
            children: Vec::new(),
            e1: point(0., 0., 0.),
            e2: point(0., 0., 0.),
            intersectable_type: IntersectableType::Plane,
            normal: point(0., 0., 0.),
            p1: point(0., 0., 0.),
            p2: point(0., 0., 0.),
            p3: point(0., 0., 0.),
        }
    }

    pub fn cube() -> Intersectable {
        Intersectable {
            children: Vec::new(),
            e1: point(0., 0., 0.),
            e2: point(0., 0., 0.),
            intersectable_type: IntersectableType::Cube,
            normal: point(0., 0., 0.),
            p1: point(0., 0., 0.),
            p2: point(0., 0., 0.),
            p3: point(0., 0., 0.),
        }
    }

    pub fn triangle(p1: Point, p2: Point, p3: Point) -> Intersectable {
        let e1 = p2.sub(&p1);
        let e2 = p3.sub(&p1);
        Intersectable {
            children: Vec::new(),
            p1,
            p2,
            p3,
            e1,
            e2,
            normal: e1.cross(&e2).normalize(),
            intersectable_type: IntersectableType::Triangle,
        }
    }

    pub fn group() -> Intersectable {
        Intersectable {
            children: Vec::new(),
            e1: point(0., 0., 0.),
            e2: point(0., 0., 0.),
            intersectable_type: IntersectableType::Group,
            normal: point(0., 0., 0.),
            p1: point(0., 0., 0.),
            p2: point(0., 0., 0.),
            p3: point(0., 0., 0.),
        }
    }

    pub fn local_normal_at(&self, point: &Point) -> Point {
        match self.intersectable_type {
            IntersectableType::Cube => self.local_normal_at_cube(point),
            IntersectableType::Plane => self.local_normal_at_plane(point),
            IntersectableType::Sphere => self.local_normal_at_sphere(point),
            _ => vector(0., 0., 0.),
        }
    }

    pub fn add(&mut self, shape: Arc<Shape>) {
        match self.intersectable_type {
            IntersectableType::Group => self.add_group(shape),
            _ => (),
        }
    }

    pub fn local_intersect(&self, ray: &Ray, object: Arc<Shape>) -> Vec<Intersection> {
        match self.intersectable_type {
            IntersectableType::Cube => self.local_intersect_cube(ray, object),
            IntersectableType::Sphere => self.local_intersect_sphere(ray, object),
            IntersectableType::Plane => self.local_intersect_plane(ray, object),
            _ => Vec::new(),
        }
    }

    pub fn bounds(&self, shape: &Shape) -> Bounds {
        match self.intersectable_type {
            IntersectableType::Cube => self.bounds_cube(shape),
            IntersectableType::Sphere => self.bounds_sphere(shape),
            IntersectableType::Plane => self.bounds_plane(shape),
            _ => Bounds::new(0., 0., 0., 0., 0., 0.),
        }
    }

    fn local_normal_at_sphere(&self, local_point: &Point) -> Point {
        local_point.sub(&point(0., 0., 0.))
    }

    fn bounds_sphere(&self, _shape: &Shape) -> Bounds {
        Bounds::new(-1.0, 1.0, -1.0, 1.0, -1.0, 1.0)
    }

    fn local_intersect_sphere(&self, ray: &Ray, object: Arc<Shape>) -> Vec<Intersection> {
        let shape_to_ray = ray.origin.sub(&point(0., 0., 0.));
        let a = ray.direction.dot(&ray.direction);
        let b = ray.direction.dot(&shape_to_ray) * 2.0;
        let c = shape_to_ray.dot(&shape_to_ray) - 1.;

        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0. {
            Vec::new()
        } else {
            Intersection::intersections(
                Intersection {
                    t: (-b - discriminant.sqrt()) / (2.0 * a),
                    object: object.clone(),
                },
                Intersection {
                    t: (-b + discriminant.sqrt()) / (2.0 * a),
                    object: object.clone(),
                },
            )
        }
    }

    fn local_normal_at_plane(&self, _local_point: &Point) -> Point {
        point(0., 1., 0.)
    }

    fn bounds_plane(&self, _shape: &Shape) -> Bounds {
        Bounds::new(-INFINITY, INFINITY, 0.0, 0.0, -INFINITY, INFINITY)
    }

    fn local_intersect_plane(&self, ray: &Ray, object: Arc<Shape>) -> Vec<Intersection> {
        if ray.direction.y.abs() < EPSILON {
            return Vec::new();
        }

        vec![Intersection {
            object: object.clone(),
            t: -ray.origin.y / ray.direction.y,
        }]
    }

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

    fn local_normal_at_cube(&self, local_point: &Point) -> Point {
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

    fn bounds_cube(&self, _shape: &Shape) -> Bounds {
        Bounds::new(-1.0, 1.0, -1.0, 1.0, -1.0, 1.0)
    }

    fn local_intersect_cube(&self, ray: &Ray, object: Arc<Shape>) -> Vec<Intersection> {
        let (xmin, xmax) = self.check_axis(ray.origin.x, ray.direction.x);
        let (ymin, ymax) = self.check_axis(ray.origin.y, ray.direction.y);
        let (zmin, zmax) = self.check_axis(ray.origin.z, ray.direction.z);

        let mins: Vec<f64> = vec![xmin, ymin, zmin];
        let maxs: Vec<f64> = vec![xmax, ymax, zmax];

        let tmin = max(&mins);
        let tmax = min(&maxs);

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

    fn local_normal_at_triangle(&self, _local_point: &Point) -> Point {
        self.normal
    }

    fn bounds_triangle(&self, _shape: &Shape) -> Bounds {
        Bounds::new(
            min(&vec![self.p1.x, self.p2.x, self.p3.x]),
            max(&vec![self.p1.x, self.p2.x, self.p3.x]),
            min(&vec![self.p1.y, self.p2.y, self.p3.y]),
            max(&vec![self.p1.y, self.p2.y, self.p3.y]),
            min(&vec![self.p1.z, self.p2.z, self.p3.z]),
            max(&vec![self.p1.z, self.p2.z, self.p3.z]),
        )
    }

    fn local_intersect_triangle(&self, ray: &Ray, object: Arc<Shape>) -> Vec<Intersection> {
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

    fn bounds_group(&self, shape: &Shape) -> Bounds {
        let mut child_bounds: Vec<Point> = Vec::new();
        for ref child in self.children.iter() {
            let bounds = child.bounds();
            child_bounds.push(child.transform.multiply_point(&bounds.min));
            child_bounds.push(child.transform.multiply_point(&bounds.max));
        }
        let mut local_bounds: Bounds = bounds(child_bounds);
        local_bounds.min = shape.transform.inverse().multiply_point(&local_bounds.min);
        local_bounds.max = shape.transform.inverse().multiply_point(&local_bounds.max);
        bounds(vec![local_bounds.min, local_bounds.max])
    }

    fn local_intersect_group(&self, ray: &Ray, object: Arc<Shape>) -> Vec<Intersection> {
        if !object.bounds().hits(ray) {
            return Vec::new();
        }
        let mut intersects: Vec<Intersection> = Vec::new();
        for obj in &self.children {
            intersects.extend(ray.intersect(obj.clone()));
        }
        intersects
    }

    fn add_group(&mut self, shape: Arc<Shape>) {
        self.children.push(shape);
    }
}

#[cfg(test)]
mod tests {
    use intersectable::*;
    use material::Material;
    use matrix::Matrix4;
    use matrix::IDENTITY_MATRIX;
    use std::f64::consts::PI;

    #[test]
    fn test_new_triangle() {
        let s = Shape::triangle(point(0., 1., 0.), point(-1., 0., 0.), point(1., 0., 0.));

        assert_eq!(s.intersectable.e1, vector(-1., -1., 0.));
        assert_eq!(s.intersectable.e2, vector(1., -1., 0.));
        assert_eq!(s.intersectable.normal, vector(0., 0., 1.));
    }

    #[test]
    fn test_group_intersect_misses() {
        let s = Arc::new(Shape {
            parent: None,
            intersectable: Intersectable::group(),
            material: Material::new(),
            transform: IDENTITY_MATRIX,
        });
        let ray = Ray {
            origin: point(0., 0., 0.),
            direction: vector(0., 0., 1.),
        };

        assert_eq!(ray.intersect(s).len(), 0);
    }

    #[test]
    fn test_group_intersect_hits() {
        let mut g = Shape::group();
        let s1 = Shape::sphere();
        let mut s2 = Shape::sphere();
        Arc::get_mut(&mut s2).unwrap().transform = Matrix4::translation(0., 0., -3.);
        let mut s3 = Shape::sphere();
        Arc::get_mut(&mut s3).unwrap().transform = Matrix4::translation(5., 0., 0.);
        let ray = Ray {
            origin: point(0., 0., -5.),
            direction: vector(0., 0., 1.),
        };

        Shape::add_shape(g.clone(), s1);
        Shape::add_shape(g.clone(), s2);
        Shape::add_shape(g.clone(), s3);

        assert_eq!(ray.intersect(g).len(), 4);
    }

    #[test]
    fn test_group_local_to_world_space() {
        let mut g1 = Shape::group();
        Arc::get_mut(&mut g1).unwrap().transform = Matrix4::rotation_y(PI / 2.);
        let mut g2 = Shape::group();
        Arc::get_mut(&mut g2).unwrap().transform = Matrix4::scaling(2., 2., 2.);
        let mut s = Shape::sphere();
        Arc::get_mut(&mut s).unwrap().transform = Matrix4::translation(5., 0., 0.);
        // I can't do the adding, because it consumes the shape
        Arc::get_mut(&mut s).unwrap().parent = Some(g1.clone());
        Shape::add_group(g2.clone(), g1);

        assert_eq!(s.world_to_object(&point(-2., 0., -10.)), point(0., 0., -1.));
    }

    #[test]
    fn test_group_local_to_world_normal() {
        let mut g1 = Shape::group();
        Arc::get_mut(&mut g1).unwrap().transform = Matrix4::rotation_y(PI / 2.);
        let mut g2 = Shape::group();
        Arc::get_mut(&mut g2).unwrap().transform = Matrix4::scaling(1., 2., 3.);
        let mut s = Shape::sphere();
        Arc::get_mut(&mut s).unwrap().transform = Matrix4::translation(5., 0., 0.);
        // I can't do the adding, because it consumes the shape
        Arc::get_mut(&mut s).unwrap().parent = Some(g2.clone());
        let sqrt_3_over_3 = 3_f64.sqrt() / 3.;
        let v = vector(sqrt_3_over_3, sqrt_3_over_3, sqrt_3_over_3);
        Shape::add_group(g1.clone(), g2);

        assert_eq!(s.normal_to_world(&v), vector(0.28571, 0.42857, -0.85714));
    }

    #[test]
    fn test_group_normal_at_child() {
        let mut g1 = Shape::group();
        Arc::get_mut(&mut g1).unwrap().transform = Matrix4::rotation_y(PI / 2.);
        let mut g2 = Shape::group();
        Arc::get_mut(&mut g2).unwrap().transform = Matrix4::scaling(1., 2., 3.);
        let mut s = Shape::sphere();
        Arc::get_mut(&mut s).unwrap().transform = Matrix4::translation(5., 0., 0.);
        // I can't do the adding, because it consumes the shape
        Arc::get_mut(&mut s).unwrap().parent = Some(g2.clone());
        let sqrt_3_over_3 = 3_f64.sqrt() / 3.;
        let v = vector(sqrt_3_over_3, sqrt_3_over_3, sqrt_3_over_3);
        Shape::add_group(g1.clone(), g2);

        assert_eq!(s.normal_at(&v), vector(0.28571, 0.42857, -0.85714));
    }
}
