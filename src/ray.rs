use intersection::Intersection;
use matrix::Matrix4;
use point::Point;
use shape::Shape;
use world::World;

pub struct Ray {
    pub origin: Point,
    pub direction: Point,
}

impl Ray {
    pub fn position(&self, t: f64) -> Point {
        self.origin.add(&self.direction.multiply_scalar(t))
    }

    pub fn intersect(&self, shape: &Shape) -> Vec<Intersection> {
        let ray = self.transform(shape.transform.inverse());
        return shape.intersectable.local_intersect(&ray, shape);
    }

    pub fn intersect_world(&self, world: &World) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = Vec::new();
        for object in &world.objects {
            intersections.extend(self.intersect(object));
        }
        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        intersections
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
    use intersectable::Sphere;
    use material::Material;
    use matrix::Matrix4;
    use point::point;
    use point::vector;
    use ray::Ray;
    use shape::Shape;
    use world::World;

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
    fn test_ray_intersects_shape() {
        let ray = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Shape::sphere();
        let xs = ray.intersect(&s);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn test_ray_intersects_shape_tangent() {
        let ray = Ray {
            origin: point(0.0, 1.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Shape::sphere();
        let xs = ray.intersect(&s);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn test_ray_misses_shape() {
        let ray = Ray {
            origin: point(0.0, 2.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Shape::sphere();
        let xs = ray.intersect(&s);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn test_ray_originates_inside_of_shape() {
        let ray = Ray {
            origin: point(0.0, 0.0, 0.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Shape::sphere();
        let xs = ray.intersect(&s);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn test_ray_ahead_of_shape() {
        let ray = Ray {
            origin: point(0.0, 0.0, 5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Shape::sphere();
        let xs = ray.intersect(&s);

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
    fn test_ray_intersects_scaled_shape() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Shape {
            transform: Matrix4::scaling(2.0, 2.0, 2.0),
            material: Material::new(),
            intersectable: Box::new(Sphere {}),
        };

        let xs = r.intersect(&s);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn test_ray_misses_translated_shape() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Shape {
            intersectable: Box::new(Sphere {}),
            transform: Matrix4::translation(5.0, 0.0, 0.0),
            material: Material::new(),
        };

        let xs = r.intersect(&s);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn test_ray_intersect_world() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let world = World::new();
        let intersections = r.intersect_world(&world);

        assert_eq!(intersections.len(), 4);
        assert_eq!(intersections[0].t, 4.0);
        assert_eq!(intersections[1].t, 4.5);
        assert_eq!(intersections[2].t, 5.5);
        assert_eq!(intersections[3].t, 6.0);
    }
}
