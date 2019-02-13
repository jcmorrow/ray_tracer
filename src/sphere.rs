use material::Material;
use matrix::Matrix4;
use matrix::IDENTITY_MATRIX;
use point::point;
use point::Point;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Sphere {
    pub transform: Matrix4,
    pub material: Material,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            transform: IDENTITY_MATRIX,
            material: Material::new(),
        }
    }

    pub fn normal_at(&self, world_point: &Point) -> Point {
        let object_point = self.transform.inverse().multiply_point(&world_point);
        let object_normal = object_point.sub(&point(0.0, 0.0, 0.0));
        let mut world_normal = self
            .transform
            .inverse()
            .transpose()
            .multiply_point(&object_normal);
        world_normal.w = 0.0;
        world_normal.normalize()
    }
}

#[cfg(test)]
mod tests {
    use material::Material;
    use matrix::Matrix4;
    use matrix::IDENTITY_MATRIX;
    use point::point;
    use point::vector;
    use sphere::Sphere;
    use std::f64::consts::PI;

    #[test]
    fn test_sphere() {
        let s = Sphere::new();

        assert_eq!(s.transform, IDENTITY_MATRIX);
    }

    #[test]
    fn test_sphere_with_non_default_transform() {
        let t = Matrix4::translation(2.0, 3.0, 4.0);
        let s = Sphere {
            transform: t,
            material: Material::new(),
        };

        assert_eq!(s.transform, t);
    }

    #[test]
    fn test_sphere_normal_at() {
        let s = Sphere::new();

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
    fn test_sphere_normal_at_with_transformation() {
        let s = Sphere {
            transform: Matrix4::translation(0.0, 1.0, 0.0),
            material: Material::new(),
        };

        assert!(s
            .normal_at(&point(0.0, 1.70711, -0.70711))
            .equal(&vector(0.0, 0.70711, -0.70711)));

        let s = Sphere {
            transform: Matrix4::scaling(1.0, 0.5, 1.0).multiply(&Matrix4::rotation_z(PI / 5.0)),
            material: Material::new(),
        };

        assert!(s
            .normal_at(&point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0))
            .equal(&vector(0.0, 0.97014, -0.24254)));
    }
}
