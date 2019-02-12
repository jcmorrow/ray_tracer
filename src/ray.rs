use intersection::Intersection;
use matrix::Matrix4;
use point::point;
use point::Point;
use sphere::Sphere;

pub struct Ray {
    pub origin: Point,
    pub direction: Point,
}

impl Ray {
    pub fn position(&self, t: f64) -> Point {
        self.origin.add(&self.direction.multiply_scalar(t))
    }

    pub fn intersect(&self, sphere: Sphere) -> Vec<Intersection> {
        let ray = self.transform(sphere.transform.inverse());
        let sphere_to_ray = ray.origin.sub(&point(0.0, 0.0, 0.0));
        let a = ray.direction.dot(&ray.direction);
        let b = ray.direction.dot(&sphere_to_ray) * 2.0;
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0.0 {
            return Vec::new();
        } else {
            return Intersection::intersections(
                Intersection {
                    t: (-b - discriminant.sqrt()) / (2.0 * a),
                    object: sphere,
                },
                Intersection {
                    t: (-b + discriminant.sqrt()) / (2.0 * a),
                    object: sphere,
                },
            );
        }
    }

    pub fn transform(&self, transformation: Matrix4) -> Ray {
        Ray {
            origin: transformation.multiply_point(&self.origin),
            direction: transformation.multiply_point(&self.direction),
        }
    }
}

#[cfg(test)]
mod tests {
    use matrix::Matrix4;
    use point::point;
    use point::vector;
    use ray::Ray;
    use sphere::Sphere;

    #[test]
    fn test_ray_creation() {
        let ray = Ray {
            origin: point(1.0, 2.0, 3.0),
            direction: vector(4.0, 5.0, 6.0),
        };

        assert!(ray.origin.equal(&point(1.0, 2.0, 3.0)));
        assert!(ray.direction.equal(&vector(4.0, 5.0, 6.0)));
    }

    #[test]
    fn test_ray_position() {
        let ray = Ray {
            origin: point(2.0, 3.0, 4.0),
            direction: vector(1.0, 0.0, 0.0),
        };

        assert!(ray.position(0.0).equal(&ray.origin));
        assert!(ray.position(1.0).equal(&point(3.0, 3.0, 4.0)));
        assert!(ray.position(-1.0).equal(&point(1.0, 3.0, 4.0)));
        assert!(ray.position(2.5).equal(&point(4.5, 3.0, 4.0)));
    }

    #[test]
    fn test_ray_intersects_sphere() {
        let ray = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Sphere::new();
        let xs = ray.intersect(s);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn test_ray_intersects_sphere_tangent() {
        let ray = Ray {
            origin: point(0.0, 1.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Sphere::new();
        let xs = ray.intersect(s);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn test_ray_misses_sphere() {
        let ray = Ray {
            origin: point(0.0, 2.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Sphere::new();
        let xs = ray.intersect(s);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn test_ray_originates_inside_of_sphere() {
        let ray = Ray {
            origin: point(0.0, 0.0, 0.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Sphere::new();
        let xs = ray.intersect(s);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn test_ray_ahead_of_sphere() {
        let ray = Ray {
            origin: point(0.0, 0.0, 5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Sphere::new();
        let xs = ray.intersect(s);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn test_ray_transform() {
        let r = Ray {
            origin: point(1.0, 2.0, 3.0),
            direction: vector(0.0, 1.0, 0.0),
        };
        let m = Matrix4::translation(3.0, 4.0, 5.0);
        let r2 = r.transform(m);

        assert!(r2.origin.equal(&point(4.0, 6.0, 8.0)));
        assert!(r2.direction.equal(&vector(0.0, 1.0, 0.0)));
    }

    #[test]
    fn test_ray_scale() {
        let r = Ray {
            origin: point(1.0, 2.0, 3.0),
            direction: vector(0.0, 1.0, 0.0),
        };
        let m = Matrix4::scaling(2.0, 3.0, 4.0);
        let r2 = r.transform(m);

        assert!(r2.origin.equal(&point(2.0, 6.0, 12.0)));
        assert!(r2.direction.equal(&vector(0.0, 3.0, 0.0)));
    }

    #[test]
    fn test_ray_intersects_scaled_sphere() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Sphere {
            transform: Matrix4::scaling(2.0, 2.0, 2.0),
        };

        let xs = r.intersect(s);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn test_ray_misses_translated_sphere() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Sphere {
            transform: Matrix4::translation(5.0, 0.0, 0.0),
        };

        let xs = r.intersect(s);

        assert_eq!(xs.len(), 0);
    }
}
