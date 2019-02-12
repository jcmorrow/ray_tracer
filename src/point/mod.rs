use utilities::equal;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

pub fn point(x: f64, y: f64, z: f64) -> Point {
    return Point { x, y, z, w: 1.0 };
}

pub fn vector(x: f64, y: f64, z: f64) -> Point {
    return Point { x, y, z, w: 0.0 };
}

pub fn empty_point() -> Point {
    return point(0.0, 0.0, 0.0);
}

fn empty_vector() -> Point {
    return vector(0.0, 0.0, 0.0);
}

impl Point {
    fn is_point(&self) -> bool {
        return equal(self.w, 1.0);
    }

    fn is_vector(&self) -> bool {
        return equal(self.w, 0.0);
    }

    pub fn equal(&self, other: &Point) -> bool {
        return equal(self.x, other.x)
            && equal(self.y, other.y)
            && equal(self.z, other.z)
            && equal(self.w, other.w);
    }

    pub fn add(&self, other: &Point) -> Point {
        return Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        };
    }

    pub fn sub(&self, other: &Point) -> Point {
        return Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        };
    }

    pub fn multiply_scalar(&self, other: f64) -> Point {
        return Point {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        };
    }

    fn divide_scalar(&self, other: f64) -> Point {
        return Point {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        };
    }

    fn negative(&self) -> Point {
        return empty_vector().sub(&self);
    }

    fn magnitude(&self) -> f64 {
        return (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt();
    }

    pub fn normalize(&self) -> Point {
        return self.divide_scalar(self.magnitude());
    }

    pub fn dot(&self, other: &Point) -> f64 {
        return self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w;
    }

    fn cross(&self, other: &Point) -> Point {
        return Point {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
            w: 0.0,
        };
    }
}

#[cfg(test)]
mod tests {
    use point::empty_point;
    use point::empty_vector;
    use point::equal;
    use point::Point;

    #[test]
    fn test_point() {
        let a: Point = Point {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
        };

        assert!(equal(a.x, 4.3));
        assert!(equal(a.y, -4.2));
        assert!(equal(a.z, 3.1));
        assert!(equal(a.w, 1.0));
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn test_vector() {
        let a: Point = Point {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.0,
        };

        assert!(equal(a.x, 4.3));
        assert!(equal(a.y, -4.2));
        assert!(equal(a.z, 3.1));
        assert!(equal(a.w, 0.0));
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn test_new_point() {
        let a: Point = empty_point();

        assert!(equal(a.x, 0.0));
        assert!(equal(a.y, 0.0));
        assert!(equal(a.z, 0.0));
        assert!(equal(a.w, 1.0));
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn test_new_vector() {
        let a: Point = empty_vector();

        assert!(equal(a.x, 0.0));
        assert!(equal(a.y, 0.0));
        assert!(equal(a.z, 0.0));
        assert!(equal(a.w, 0.0));
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn test_vector_equality() {
        let a: Point = empty_vector();
        let b: Point = empty_vector();

        assert!(a.equal(&b));
    }

    #[test]
    fn test_point_equality() {
        let a: Point = empty_point();
        let b: Point = empty_point();

        assert!(a.equal(&b));
    }

    #[test]
    fn test_add_vector_to_vector() {
        let a: Point = Point {
            x: 3.0,
            y: -2.0,
            z: 5.0,
            w: 0.0,
        };
        let b: Point = Point {
            x: -2.0,
            y: 3.0,
            z: 1.0,
            w: 0.0,
        };
        let sum = a.add(&b);

        assert!(equal(sum.x, 1.0));
        assert!(equal(sum.y, 1.0));
        assert!(equal(sum.z, 6.0));
        assert!(equal(sum.w, 0.0));
    }

    #[test]
    fn test_add_vector_to_point() {
        let a: Point = Point {
            x: 3.0,
            y: -2.0,
            z: 5.0,
            w: 0.0,
        };
        let b: Point = Point {
            x: -2.0,
            y: 3.0,
            z: 1.0,
            w: 1.0,
        };
        let sum = a.add(&b);

        assert!(equal(sum.x, 1.0));
        assert!(equal(sum.y, 1.0));
        assert!(equal(sum.z, 6.0));
        assert!(equal(sum.w, 1.0));
    }

    #[test]
    fn test_subtract_point_from_point() {
        let a: Point = Point {
            x: 3.0,
            y: 2.0,
            z: 1.0,
            w: 1.0,
        };
        let b: Point = Point {
            x: 5.0,
            y: 6.0,
            z: 7.0,
            w: 1.0,
        };
        let diff = a.sub(&b);

        assert!(equal(diff.x, -2.0));
        assert!(equal(diff.y, -4.0));
        assert!(equal(diff.z, -6.0));
        assert!(equal(diff.w, 0.0));
    }

    #[test]
    fn test_subtract_vector_from_point() {
        let a: Point = Point {
            x: 3.0,
            y: 2.0,
            z: 1.0,
            w: 1.0,
        };
        let b: Point = Point {
            x: 5.0,
            y: 6.0,
            z: 7.0,
            w: 0.0,
        };
        let diff = a.sub(&b);

        assert!(equal(diff.x, -2.0));
        assert!(equal(diff.y, -4.0));
        assert!(equal(diff.z, -6.0));
        assert!(equal(diff.w, 1.0));
    }

    #[test]
    fn test_negative() {
        let vector = Point {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: 0.0,
        };
        assert!(vector.negative().equal(&Point {
            x: -1.0,
            y: 2.0,
            z: -3.0,
            w: 0.0,
        }))
    }

    #[test]
    fn test_vector_multiply_scalar() {
        let a: Point = Point {
            x: 3.0,
            y: 2.0,
            z: 1.0,
            w: 0.0,
        };
        let product = a.multiply_scalar(2.0);

        assert!(equal(product.x, 6.0));
        assert!(equal(product.y, 4.0));
        assert!(equal(product.z, 2.0));
        assert!(equal(product.w, 0.0));
    }

    #[test]
    fn test_vector_divide_scalar() {
        let a: Point = Point {
            x: 3.0,
            y: 2.0,
            z: 1.0,
            w: 0.0,
        };
        let product = a.divide_scalar(2.0);

        assert!(equal(product.x, 1.5));
        assert!(equal(product.y, 1.0));
        assert!(equal(product.z, 0.5));
        assert!(equal(product.w, 0.0));
    }

    #[test]
    fn test_magnitude() {
        let a: Point = Point {
            x: -1.0,
            y: -2.0,
            z: -3.0,
            w: 0.0,
        };

        const FOURTEEN: f64 = 14.0;

        assert!(equal(a.magnitude(), FOURTEEN.sqrt()));
    }

    #[test]
    fn test_normalize() {
        let a: Point = Point {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 0.0,
        };

        const FOURTEEN: f64 = 14.0;

        assert!(a.normalize().equal(&Point {
            x: 1.0 / FOURTEEN.sqrt(),
            y: 2.0 / FOURTEEN.sqrt(),
            z: 3.0 / FOURTEEN.sqrt(),
            w: 0.0
        }));
    }

    #[test]
    fn test_dot_product() {
        let a: Point = Point {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 0.0,
        };
        let b: Point = Point {
            x: 2.0,
            y: 3.0,
            z: 4.0,
            w: 0.0,
        };

        assert!(equal(a.dot(&b), 20.0));
    }

    #[test]
    fn test_cross_product() {
        let a: Point = Point {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 0.0,
        };
        let b: Point = Point {
            x: 2.0,
            y: 3.0,
            z: 4.0,
            w: 0.0,
        };

        assert!(a.cross(&b).equal(&Point {
            x: -1.0,
            y: 2.0,
            z: -1.0,
            w: 0.0,
        }));
        assert!(b.cross(&a).equal(&Point {
            x: 1.0,
            y: -2.0,
            z: 1.0,
            w: 0.0,
        }));
    }
}
