use intersectable::Intersectable;
use intersectable::Plane;
use intersectable::Sphere;
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

    pub fn normal_at(&self, world_point: &Point) -> Point {
        let local_point = self.transform.inverse().multiply_point(&world_point);
        let local_normal = self.intersectable.local_normal_at(&local_point);
        let mut world_normal = self
            .transform
            .inverse()
            .transpose()
            .multiply_point(&local_normal);
        world_normal.w = 0.0;
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
    use intersectable::Sphere;
    use material::Material;
    use matrix::Matrix4;
    use matrix::IDENTITY_MATRIX;
    use point::point;
    use point::vector;
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
        let t = Matrix4::translation(2.0, 3.0, 4.0);
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

        assert!(s
            .normal_at(&point(1.0, 0.0, 0.0))
            .equal(&vector(1.0, 0.0, 0.0)));
        assert!(s
            .normal_at(&point(0.0, 1.0, 0.0))
            .equal(&vector(0.0, 1.0, 0.0)));
        assert!(s
            .normal_at(&point(0.0, 0.0, 1.0))
            .equal(&vector(0.0, 0.0, 1.0)));
        let sqrt_3_over_3 = 3.0_f64.sqrt() / 3.0;
        let p = point(sqrt_3_over_3, sqrt_3_over_3, sqrt_3_over_3);
        let v = vector(sqrt_3_over_3, sqrt_3_over_3, sqrt_3_over_3);
        assert!(s.normal_at(&p).equal(&v));
        assert!(v.equal(&v.normalize()));
    }

    #[test]
    fn test_shape_normal_at_with_transformation() {
        let s = Shape {
            transform: Matrix4::translation(0.0, 1.0, 0.0),
            material: Material::new(),
            intersectable: Box::new(Sphere {}),
        };

        assert!(s
            .normal_at(&point(0.0, 1.70711, -0.70711))
            .equal(&vector(0.0, 0.70711, -0.70711)));

        let s = Shape {
            intersectable: Box::new(Sphere {}),
            transform: Matrix4::scaling(1.0, 0.5, 1.0).multiply(&Matrix4::rotation_z(PI / 5.0)),
            material: Material::new(),
        };

        assert!(s
            .normal_at(&point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0))
            .equal(&vector(0.0, 0.97014, -0.24254)));
    }

    #[test]
    fn test_plane_normal_at() {
        let s = Shape::plane();

        assert!(s.normal_at(&point(0.0, 0.0, 0.0)).eq(&point(0.0, 1.0, 0.0)));
    }

    #[test]
    fn test_plane_intersection() {
        let s = Shape::plane();
        let r = Ray {
            origin: point(0.0, 10.0, 0.0),
            direction: vector(0.0, 0.0, 1.0),
        };

        assert_eq!(r.intersect(&s).len(), 0);
    }

    #[test]
    fn test_plane_coplanar() {
        let s = Shape::plane();
        let r = Ray {
            origin: point(0.0, 0.0, 0.0),
            direction: vector(0.0, 0.0, 1.0),
        };

        assert_eq!(r.intersect(&s).len(), 0);
    }

    #[test]
    fn test_plane_does_intersect() {
        let s = Shape::plane();
        let r = Ray {
            origin: point(0.0, 1.0, 0.0),
            direction: vector(0.0, -1.0, 0.0),
        };

        assert_eq!(r.intersect(&s).len(), 1);
        assert_eq!(r.intersect(&s)[0].t, 1.0);
        assert_eq!(r.intersect(&s)[0].object, s);
    }
}
