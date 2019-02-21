// taken completely from https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object
use intersection::Intersection;
use point::point;
use point::Point;
use ray::Ray;
use shape::Shape;
use std::fmt::Debug;
use utilities::EPSILON;

pub trait Intersectable: Debug + IntersectableClone {
    fn local_normal_at(&self, point: &Point) -> Point;
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
        local_point.sub(&point(0.0, 0.0, 0.0))
    }

    fn local_intersect(&self, ray: &Ray, object: &Shape) -> Vec<Intersection> {
        let shape_to_ray = ray.origin.sub(&point(0.0, 0.0, 0.0));
        let a = ray.direction.dot(&ray.direction);
        let b = ray.direction.dot(&shape_to_ray) * 2.0;
        let c = shape_to_ray.dot(&shape_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0.0 {
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
        point(0.0, 1.0, 0.0)
    }

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
