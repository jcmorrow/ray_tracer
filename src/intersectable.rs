// taken completely from https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object
use intersection::Intersection;
use point::point;
use point::vector;
use point::Point;
use ray::Ray;
use shape::Shape;
use std::f64::INFINITY;
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
        local_point.sub(&point(0., 0., 0.))
    }

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
pub struct Cylinder {
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
    pub closed: bool,
}

impl Cylinder {
    fn minimum(&self) -> f64 {
        match self.minimum {
            Some(n) => n,
            None => -INFINITY,
        }
    }

    fn maximum(&self) -> f64 {
        match self.maximum {
            Some(n) => n,
            None => INFINITY,
        }
    }

    fn check_cap(&self, ray: &Ray, t: f64) -> bool {
        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;

        x.powi(2) + z.powi(2) <= 1.
    }

    fn check_caps(&self, ray: &Ray, object: &Shape) -> Vec<Intersection> {
        let mut hits: Vec<Intersection> = Vec::new();
        if !self.closed || ray.direction.y.abs() < EPSILON {
            return hits;
        }

        let mut t = (self.minimum() - ray.origin.y) / ray.direction.y;
        if self.check_cap(&ray, t) {
            hits.push(Intersection {
                object: object.clone(),
                t,
            });
        }

        t = (self.maximum() - ray.origin.y) / ray.direction.y;
        if self.check_cap(&ray, t) {
            hits.push(Intersection {
                object: object.clone(),
                t,
            });
        }

        hits
    }
}

impl Intersectable for Cylinder {
    fn local_normal_at(&self, local_point: &Point) -> Point {
        let dist = local_point.x.powi(2) + local_point.z.powi(2);
        if dist < 1. && local_point.y >= self.maximum() - EPSILON {
            return vector(0., 1., 0.);
        }
        if dist < 1. && local_point.y <= self.minimum() + EPSILON {
            return vector(0., -1., 0.);
        }
        vector(local_point.x, 0., local_point.z)
    }

    fn local_intersect(&self, ray: &Ray, object: &Shape) -> Vec<Intersection> {
        let a = ray.direction.x.powi(2) + ray.direction.z.powi(2);
        if a.abs() < EPSILON {
            return self.check_caps(&ray, &object);
        }

        let b = 2. * ray.origin.x * ray.direction.x + 2. * ray.origin.z * ray.direction.z;
        let c = ray.origin.x.powi(2) + ray.origin.z.powi(2) - 1.;

        let disc = b.powi(2) - 4. * a * c;

        if disc < 0. {
            return Vec::new();
        } else {
            let a2 = 2. * a;
            let disc_sqrt = disc.sqrt();
            let mut t0 = (-b - disc_sqrt) / a2;
            let mut t1 = (-b + disc_sqrt) / a2;
            if t1 < t0 {
                let tmp = t1;
                t1 = t0;
                t0 = tmp;
            }
            let mut hits: Vec<Intersection> = Vec::new();
            let y0 = ray.origin.y + t0 * ray.direction.y;
            if y0 > self.minimum() && y0 < self.maximum() {
                hits.push(Intersection {
                    t: t0,
                    object: object.clone(),
                });
            }
            let y1 = ray.origin.y + t1 * ray.direction.y;
            if y1 > self.minimum() && y1 < self.maximum() {
                hits.push(Intersection {
                    t: t1,
                    object: object.clone(),
                });
            }
            hits.extend(self.check_caps(&ray, &object));
            hits
        }
    }
}
