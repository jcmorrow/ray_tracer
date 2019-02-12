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

    pub fn intersect(&self, sphere: Sphere) -> Vec<f64> {
        let sphere_to_ray = self.origin.sub(&point(0.0, 0.0, 0.0));
        let a = self.direction.dot(&self.direction);
        let b = self.direction.dot(&sphere_to_ray) * 2.0;
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0.0 {
            return Vec::new();
        } else {
            return vec![
                (-b - discriminant.sqrt()) / (2.0 * a),
                (-b + discriminant.sqrt()) / (2.0 * a),
            ];
        }
    }
}

#[cfg(test)]
mod tests {
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
        let s = Sphere {};
        let xs = ray.intersect(s);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 4.0);
        assert_eq!(xs[1], 6.0);
    }

    #[test]
    fn test_ray_intersects_sphere_tangent() {
        let ray = Ray {
            origin: point(0.0, 1.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Sphere {};
        let xs = ray.intersect(s);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 5.0);
        assert_eq!(xs[1], 5.0);
    }

    #[test]
    fn test_ray_misses_sphere() {
        let ray = Ray {
            origin: point(0.0, 2.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Sphere {};
        let xs = ray.intersect(s);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn test_ray_originates_inside_of_sphere() {
        let ray = Ray {
            origin: point(0.0, 0.0, 0.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Sphere {};
        let xs = ray.intersect(s);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -1.0);
        assert_eq!(xs[1], 1.0);
    }

    #[test]
    fn test_ray_ahead_of_sphere() {
        let ray = Ray {
            origin: point(0.0, 0.0, 5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Sphere {};
        let xs = ray.intersect(s);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -6.0);
        assert_eq!(xs[1], -4.0);
    }
}
