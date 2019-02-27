use point::point;
use point::Point;
use ray::Ray;
use std::f64::INFINITY;
use utilities::{max, min, EPSILON};

pub struct Bounds {
    pub max: Point,
    pub min: Point,
}

impl Bounds {
    pub fn new(xmin: f64, xmax: f64, ymin: f64, ymax: f64, zmin: f64, zmax: f64) -> Bounds {
        Bounds {
            min: point(xmin, ymin, zmin),
            max: point(xmax, ymax, zmax),
        }
    }

    fn check_axis(&self, origin: f64, direction: f64, min: f64, max: f64) -> (f64, f64) {
        let tmin: f64;
        let tmax: f64;
        let tmin_numerator = min - origin;
        let tmax_numerator = max - origin;
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

    pub fn hits(&self, ray: &Ray) -> bool {
        let (xmin, xmax) = self.check_axis(ray.origin.x, ray.direction.x, self.min.x, self.max.x);
        let (ymin, ymax) = self.check_axis(ray.origin.y, ray.direction.y, self.min.y, self.max.y);
        let (zmin, zmax) = self.check_axis(ray.origin.z, ray.direction.z, self.min.z, self.max.z);

        let mins: Vec<f64> = vec![xmin, ymin, zmin];
        let maxs: Vec<f64> = vec![xmax, ymax, zmax];

        let tmin = max(&mins);
        let tmax = min(&maxs);

        tmin < tmax
    }
}
