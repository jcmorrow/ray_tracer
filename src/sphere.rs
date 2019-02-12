use matrix::Matrix4;
use matrix::IDENTITY_MATRIX;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Sphere {
    pub transform: Matrix4,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            transform: IDENTITY_MATRIX,
        }
    }
}

#[cfg(test)]
mod tests {
    use matrix::Matrix4;
    use matrix::IDENTITY_MATRIX;
    use sphere::Sphere;

    #[test]
    fn test_sphere() {
        let s = Sphere::new();

        assert_eq!(s.transform, IDENTITY_MATRIX);
    }

    #[test]
    fn test_sphere_with_non_default_transform() {
        let t = Matrix4::translation(2.0, 3.0, 4.0);
        let s = Sphere { transform: t };

        assert_eq!(s.transform, t);
    }
}
