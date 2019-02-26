use point::empty_point;
use point::Point;
use utilities::equal;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Matrix4 {
    members: [[f64; 4]; 4],
}

pub const IDENTITY_MATRIX: Matrix4 = Matrix4 {
    members: [
        [1., 0., 0., 0.],
        [0., 1., 0., 0.],
        [0., 0., 1., 0.],
        [0., 0., 0., 1.],
    ],
};

impl Matrix4 {
    pub fn new(members: [[f64; 4]; 4]) -> Matrix4 {
        Matrix4 { members: members }
    }

    pub fn empty() -> Matrix4 {
        Matrix4 {
            members: [[0.; 4]; 4],
        }
    }

    pub fn equal(&self, other: &Matrix4) -> bool {
        for x in 0..4 {
            for y in 0..4 {
                if !equal(self.members[x][y], other.members[x][y]) {
                    return false;
                }
            }
        }
        true
    }

    pub fn multiply(&self, other: &Matrix4) -> Matrix4 {
        let mut result = Matrix4::empty();
        for row in 0..4 {
            for col in 0..4 {
                result.members[row][col] = self.members[row][0] * other.members[0][col]
                    + self.members[row][1] * other.members[1][col]
                    + self.members[row][2] * other.members[2][col]
                    + self.members[row][3] * other.members[3][col];
            }
        }
        result
    }

    pub fn multiply_point(&self, point: &Point) -> Point {
        let mut result = empty_point();
        result.x = self.members[0][0] * point.x
            + self.members[0][1] * point.y
            + self.members[0][2] * point.z
            + self.members[0][3] * point.w;
        result.y = self.members[1][0] * point.x
            + self.members[1][1] * point.y
            + self.members[1][2] * point.z
            + self.members[1][3] * point.w;
        result.z = self.members[2][0] * point.x
            + self.members[2][1] * point.y
            + self.members[2][2] * point.z
            + self.members[2][3] * point.w;
        result.w = self.members[3][0] * point.x
            + self.members[3][1] * point.y
            + self.members[3][2] * point.z
            + self.members[3][3] * point.w;
        result
    }

    pub fn transpose(&self) -> Matrix4 {
        let mut result = Matrix4::empty();
        for x in 0..4 {
            for y in 0..4 {
                result.members[x][y] = self.members[y][x];
            }
        }
        result
    }

    pub fn submatrix(&self, not_col: usize, not_row: usize) -> Matrix3 {
        let mut result = Matrix3::empty();
        let mut y = 0;
        for row in 0..4 {
            let mut x = 0;
            for col in 0..4 {
                if not_col != col && not_row != row {
                    result.members[x][y] = self.members[col][row];
                    x = x + 1;
                }
            }
            if row != not_row {
                y = y + 1;
            }
        }
        result
    }

    pub fn minor(&self, col: usize, row: usize) -> f64 {
        self.submatrix(col, row).determinant()
    }

    pub fn cofactor(&self, col: usize, row: usize) -> f64 {
        let minor = self.minor(col, row);
        if (col + row) % 2 == 0 {
            return minor;
        } else {
            return minor * -1.;
        }
    }

    pub fn determinant(&self) -> f64 {
        let mut result = 0.;
        for i in 0..4 {
            result = result + self.members[0][i] * self.cofactor(0, i);
        }
        result
    }

    pub fn invertible(&self) -> bool {
        self.determinant() != 0.
    }

    pub fn inverse(&self) -> Matrix4 {
        if !self.invertible() {
            panic!("Matrix is not invertible");
        }

        let mut result = Matrix4::empty();
        let determinant = self.determinant();

        for row in 0..4 {
            for col in 0..4 {
                let mut cofactor = self.cofactor(col, row);
                result.members[row][col] = cofactor / determinant;
            }
        }

        result
    }

    pub fn translation(x: f64, y: f64, z: f64) -> Matrix4 {
        let mut result = IDENTITY_MATRIX;
        result.members[0][3] = x;
        result.members[1][3] = y;
        result.members[2][3] = z;
        result
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Matrix4 {
        let mut result = IDENTITY_MATRIX;
        result.members[0][0] = x;
        result.members[1][1] = y;
        result.members[2][2] = z;
        result
    }

    pub fn rotation_x(radians: f64) -> Matrix4 {
        let mut result = IDENTITY_MATRIX;
        result.members[1][1] = radians.cos();
        result.members[1][2] = -radians.sin();
        result.members[2][1] = radians.sin();
        result.members[2][2] = radians.cos();
        result
    }

    pub fn rotation_y(radians: f64) -> Matrix4 {
        let mut result = IDENTITY_MATRIX;
        result.members[0][0] = radians.cos();
        result.members[0][2] = radians.sin();
        result.members[2][0] = -radians.sin();
        result.members[2][2] = radians.cos();
        result
    }

    pub fn rotation_z(radians: f64) -> Matrix4 {
        let mut result = IDENTITY_MATRIX;
        result.members[0][0] = radians.cos();
        result.members[0][1] = -radians.sin();
        result.members[1][0] = radians.sin();
        result.members[1][1] = radians.cos();
        result
    }

    pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix4 {
        let mut result = IDENTITY_MATRIX;
        result.members[0][1] = xy;
        result.members[0][2] = xz;
        result.members[1][0] = yx;
        result.members[1][2] = yz;
        result.members[2][0] = zx;
        result.members[2][1] = zy;
        result
    }
}

#[derive(Debug)]
pub struct Matrix3 {
    members: [[f64; 3]; 3],
}

impl Matrix3 {
    pub fn new(members: [[f64; 3]; 3]) -> Matrix3 {
        Matrix3 { members: members }
    }

    pub fn empty() -> Matrix3 {
        Matrix3::new([[0.; 3]; 3])
    }

    pub fn equal(&self, other: &Matrix3) -> bool {
        for x in 0..3 {
            for y in 0..3 {
                if !equal(self.members[x][y], other.members[x][y]) {
                    return false;
                }
            }
        }
        true
    }

    pub fn submatrix(&self, not_col: usize, not_row: usize) -> Matrix2 {
        let mut result = Matrix2::empty();
        let mut y = 0;
        for row in 0..3 {
            let mut x = 0;
            for col in 0..3 {
                if not_col != col && not_row != row {
                    result.members[x][y] = self.members[col][row];
                    x = x + 1;
                }
            }
            if row != not_row {
                y = y + 1;
            }
        }
        result
    }

    pub fn minor(&self, col: usize, row: usize) -> f64 {
        self.submatrix(col, row).determinant()
    }

    pub fn cofactor(&self, col: usize, row: usize) -> f64 {
        let minor = self.minor(col, row);
        if col + row % 2 == 0 {
            return minor;
        } else {
            return minor * -1.;
        }
    }

    pub fn determinant(&self) -> f64 {
        let mut result = 0.;
        for i in 0..3 {
            result = result + self.members[0][i] * self.cofactor(0, i);
        }
        result
    }
}

#[derive(Debug)]
pub struct Matrix2 {
    members: [[f64; 2]; 2],
}

impl Matrix2 {
    pub fn new(members: [[f64; 2]; 2]) -> Matrix2 {
        Matrix2 { members: members }
    }

    pub fn empty() -> Matrix2 {
        Matrix2::new([[0.; 2]; 2])
    }

    pub fn equal(&self, other: &Matrix2) -> bool {
        for x in 0..2 {
            for y in 0..2 {
                if !equal(self.members[x][y], other.members[x][y]) {
                    return false;
                }
            }
        }
        true
    }

    pub fn determinant(&self) -> f64 {
        self.members[0][0] * self.members[1][1] - self.members[1][0] * self.members[0][1]
    }
}

#[cfg(test)]
mod tests {
    use matrix::Matrix2;
    use matrix::Matrix3;
    use matrix::Matrix4;
    use matrix::IDENTITY_MATRIX;
    use point::point;
    use point::vector;
    use std::f64::consts::PI;
    use utilities::equal;

    #[test]
    fn test_matrix_new() {
        let mat2 = Matrix2::new([[-3., 5.], [1., -2.]]);

        assert!(equal(mat2.members[0][0], -3.));
        assert!(equal(mat2.members[0][1], 5.));
        assert!(equal(mat2.members[1][0], 1.));
        assert!(equal(mat2.members[1][1], -2.));

        let mat3 = Matrix3::new([[-3., 5., 0.], [1., -2., -7.], [0., 1., 1.]]);

        assert!(equal(mat3.members[0][0], -3.));
        assert!(equal(mat3.members[0][1], 5.));
        assert!(equal(mat3.members[0][2], 0.));
        assert!(equal(mat3.members[1][0], 1.));
        assert!(equal(mat3.members[1][1], -2.));
        assert!(equal(mat3.members[1][2], -7.));
        assert!(equal(mat3.members[2][0], 0.));
        assert!(equal(mat3.members[2][1], 1.));
        assert!(equal(mat3.members[2][2], 1.));

        let mat4 = Matrix4::new([
            [1., 2., 3., 4.],
            [5.5, 6.5, 7.5, 8.5],
            [9., 10., 11., 12.],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert!(equal(mat4.members[0][0], 1.));
        assert!(equal(mat4.members[0][1], 2.));
        assert!(equal(mat4.members[0][2], 3.));
        assert!(equal(mat4.members[0][3], 4.));
        assert!(equal(mat4.members[1][0], 5.5));
        assert!(equal(mat4.members[1][1], 6.5));
        assert!(equal(mat4.members[1][2], 7.5));
        assert!(equal(mat4.members[1][3], 8.5));
        assert!(equal(mat4.members[2][0], 9.));
        assert!(equal(mat4.members[2][1], 10.));
        assert!(equal(mat4.members[2][2], 11.));
        assert!(equal(mat4.members[2][3], 12.));
        assert!(equal(mat4.members[3][0], 13.5));
        assert!(equal(mat4.members[3][1], 14.5));
        assert!(equal(mat4.members[3][2], 15.5));
        assert!(equal(mat4.members[3][3], 16.5));
    }

    #[test]
    fn test_matrix_equals() {
        let a = Matrix2::new([[1., 2.], [3., 4.]]);
        let b = Matrix2::new([[1., 2.], [3., 4.]]);

        assert!(a.equal(&b));

        let a = Matrix2::new([[2., 2.], [3., 4.]]);
        let b = Matrix2::new([[1., 2.], [3., 4.]]);

        assert!(!a.equal(&b));

        let a = Matrix3::new([[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]]);
        let b = Matrix3::new([[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]]);

        assert!(a.equal(&b));

        let a = Matrix3::new([[2., 2., 3.], [4., 5., 6.], [7., 8., 9.]]);
        let b = Matrix3::new([[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]]);

        assert!(!a.equal(&b));

        let a = Matrix4::new([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 10., 11., 12.],
            [13., 14., 15., 16.],
        ]);
        let b = Matrix4::new([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 10., 11., 12.],
            [13., 14., 15., 16.],
        ]);

        assert!(a.equal(&b));

        let a = Matrix4::new([
            [2., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 10., 11., 12.],
            [13., 14., 15., 16.],
        ]);
        let b = Matrix4::new([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 10., 11., 12.],
            [13., 14., 15., 16.],
        ]);

        assert!(!a.equal(&b));
    }

    #[test]
    fn test_matrix_multiply() {
        let a = Matrix4::new([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.],
        ]);
        let b = Matrix4::new([
            [-2., 1., 2., 3.],
            [3., 2., 1., -1.],
            [4., 3., 6., 5.],
            [1., 2., 7., 8.],
        ]);
        let c = Matrix4::new([
            [20., 22., 50., 48.],
            [44., 54., 114., 108.],
            [40., 58., 110., 102.],
            [16., 26., 46., 42.],
        ]);

        assert!(a.multiply(&b).equal(&c));
    }

    #[test]
    fn test_matrix_multiply_point() {
        let a = Matrix4::new([
            [1., 2., 3., 4.],
            [2., 4., 4., 2.],
            [8., 6., 4., 1.],
            [0., 0., 0., 1.],
        ]);
        let b = point(1., 2., 3.);
        let c = point(18., 24., 33.);

        assert!(a.multiply_point(&b).equal(&c));
    }

    #[test]
    fn test_matrix_multiply_identity() {
        let a = Matrix4::new([
            [1., 2., 3., 4.],
            [2., 4., 4., 2.],
            [8., 6., 4., 1.],
            [0., 0., 0., 1.],
        ]);

        assert!(a.multiply(&IDENTITY_MATRIX).equal(&a));
    }

    #[test]
    fn test_matrix_transpose() {
        let a = Matrix4::new([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 10., 11., 12.],
            [13., 14., 15., 16.],
        ]);
        let b = Matrix4::new([
            [1., 5., 9., 13.],
            [2., 6., 10., 14.],
            [3., 7., 11., 15.],
            [4., 8., 12., 16.],
        ]);

        assert!(a.transpose().equal(&b));
        assert!(IDENTITY_MATRIX.transpose().equal(&IDENTITY_MATRIX));
    }

    #[test]
    fn test_matrix_determinant() {
        let a = Matrix2 {
            members: [[1., 5.], [-3., 2.]],
        };

        assert!(equal(a.determinant(), 17.));
    }

    #[test]
    fn test_matrix_submatrix() {
        let a = Matrix3 {
            members: [[1., 5., 0.], [-3., 2., 7.], [0., 6., -3.]],
        };
        let b = Matrix2 {
            members: [[-3., 2.], [0., 6.]],
        };

        assert!(a.submatrix(0, 2).equal(&b));

        let a = Matrix4::new([
            [-6., 1., 1., 6.],
            [-8., 5., 8., 6.],
            [-1., 0., 8., 2.],
            [-7., 1., -1., 1.],
        ]);
        let b = Matrix3::new([[-6., 1., 6.], [-8., 8., 6.], [-7., -1., 1.]]);

        assert!(a.submatrix(2, 1).equal(&b));
    }

    #[test]
    fn test_matrix_minor() {
        let a = Matrix3 {
            members: [[3., 5., 0.], [2., -1., -7.], [6., -1., 5.]],
        };

        assert!(equal(a.minor(1, 0), 25.));
    }

    #[test]
    fn test_matrix_3_cofactor() {
        let a = Matrix3 {
            members: [[3., 5., 0.], [2., -1., -7.], [6., -1., 5.]],
        };

        assert!(equal(a.cofactor(0, 0), -12.));
        assert!(equal(a.cofactor(1, 0), -25.));
    }

    #[test]
    fn test_matrix_4_cofactor() {
        let a = Matrix4 {
            members: [
                [-2., -8., 3., 5.],
                [-3., 1., 7., 3.],
                [1., 2., -9., 6.],
                [-6., 7., 7., -9.],
            ],
        };

        assert!(equal(a.cofactor(0, 0), 690.));
        assert!(equal(a.cofactor(0, 1), 447.));
        assert!(equal(a.determinant(), -4071.));
    }

    #[test]
    fn test_matrix_3_determinant() {
        let a = Matrix3 {
            members: [[1., 2., 6.], [-5., 8., -4.], [2., 6., 4.]],
        };

        assert!(equal(a.determinant(), -196.));
    }

    #[test]
    fn test_matrix_4_invertible() {
        let a = Matrix4 {
            members: [
                [-2., -8., 3., 5.],
                [-3., 1., 7., 3.],
                [1., 2., -9., 6.],
                [-6., 7., 7., -9.],
            ],
        };

        assert!(a.invertible());

        let b = Matrix4 {
            members: [
                [-4., 2., -2., -3.],
                [9., 6., 2., 6.],
                [0., -5., 1., -5.],
                [0., 0., 0., 0.],
            ],
        };

        assert!(!b.invertible());
    }

    #[test]
    fn test_matrix_4_inverse() {
        let a = Matrix4 {
            members: [
                [-5., 2., 6., -8.],
                [1., -5., 1., 8.],
                [7., 7., -6., -7.],
                [1., -3., 7., 4.],
            ],
        };

        let inverse = Matrix4 {
            members: [
                [0.21805, 0.45113, 0.24060, -0.04511],
                [-0.80827, -1.45677, -0.44361, 0.52068],
                [-0.07895, -0.22368, -0.05263, 0.19737],
                [-0.52256, -0.81391, -0.30075, 0.30639],
            ],
        };

        assert!(equal(a.determinant(), 532.));
        assert!(equal(a.cofactor(2, 3), -160.));
        assert!(a.inverse().equal(&inverse));

        let b = Matrix4 {
            members: [
                [-2., -8., 3., 5.],
                [-3., 1., 7., 3.],
                [1., 2., -9., 6.],
                [-6., 7., 7., -9.],
            ],
        };
        assert!(a.multiply(&b).multiply(&b.inverse()).equal(&a));
    }

    #[test]
    fn test_translation() {
        let transform = Matrix4::translation(5., -3., 2.);
        let p = point(-3., 4., 5.);
        let v = vector(-3., 4., 5.);

        assert!(transform.multiply_point(&p).equal(&point(2., 1., 7.)));
        assert!(transform
            .inverse()
            .multiply_point(&transform.multiply_point(&p))
            .equal(&p));
        assert!(transform.multiply_point(&v).equal(&v));
    }

    #[test]
    fn test_scaling() {
        let transform = Matrix4::scaling(2., 3., 4.);
        let v = vector(-4., 6., 8.);

        assert!(transform.multiply_point(&v).equal(&vector(-8., 18., 32.)));
        assert!(transform
            .inverse()
            .multiply_point(&v)
            .equal(&vector(-2., 2., 2.)));
    }

    #[test]
    fn test_reflection() {
        let transform = Matrix4::scaling(-1., 1., 1.);
        let v = vector(2., 3., 4.);

        assert!(transform.multiply_point(&v).equal(&vector(-2., 3., 4.)));
    }

    #[test]
    fn test_rotation_x() {
        let half_quarter = Matrix4::rotation_x(PI / 4.);
        let full_quarter = Matrix4::rotation_x(PI / 2.);
        let p = point(0., 1., 0.);

        assert!(half_quarter.multiply_point(&p).equal(&point(
            0.,
            2.0_f64.sqrt() / 2.,
            2.0_f64.sqrt() / 2.
        )));
        assert!(full_quarter.multiply_point(&p).equal(&point(0., 0., 1.)));
    }

    #[test]
    fn test_rotation_y() {
        let half_quarter = Matrix4::rotation_y(PI / 4.);
        let full_quarter = Matrix4::rotation_y(PI / 2.);
        let p = point(0., 0., 1.);

        assert!(half_quarter.multiply_point(&p).equal(&point(
            2.0_f64.sqrt() / 2.,
            0.,
            2.0_f64.sqrt() / 2.
        )));
        assert!(full_quarter.multiply_point(&p).equal(&point(1., 0., 0.)));
    }

    #[test]
    fn test_rotation_z() {
        let half_quarter = Matrix4::rotation_z(PI / 4.);
        let full_quarter = Matrix4::rotation_z(PI / 2.);
        let p = point(0., 1., 0.);

        assert!(half_quarter.multiply_point(&p).equal(&point(
            -2.0_f64.sqrt() / 2.,
            2.0_f64.sqrt() / 2.,
            0.,
        )));
        assert!(full_quarter.multiply_point(&p).equal(&point(-1., 0., 0.)));
    }

    #[test]
    fn test_shearing() {
        let transform = Matrix4::shearing(1., 0., 0., 0., 0., 0.);
        let p = point(2., 3., 4.);

        assert!(transform.multiply_point(&p).equal(&point(5., 3., 4.)));

        let transform = Matrix4::shearing(0., 1., 0., 0., 0., 0.);

        assert!(transform.multiply_point(&p).equal(&point(6., 3., 4.)));

        let transform = Matrix4::shearing(0., 0., 1., 0., 0., 0.);

        assert!(transform.multiply_point(&p).equal(&point(2., 5., 4.)));

        let transform = Matrix4::shearing(0., 0., 0., 1., 0., 0.);

        assert!(transform.multiply_point(&p).equal(&point(2., 7., 4.)));

        let transform = Matrix4::shearing(0., 0., 0., 0., 1., 0.);

        assert!(transform.multiply_point(&p).equal(&point(2., 3., 6.)));

        let transform = Matrix4::shearing(0., 0., 0., 0., 0., 1.);

        assert!(transform.multiply_point(&p).equal(&point(2., 3., 7.)));
    }

    #[test]
    fn test_chaining_transformations() {
        let p = point(1., 0., 1.);
        let a = Matrix4::rotation_x(PI / 2.);
        let b = Matrix4::scaling(5., 5., 5.);
        let c = Matrix4::translation(10., 5., 7.);

        let p2 = a.multiply_point(&p);
        assert!(p2.equal(&point(1., -1., 0.)));

        let p3 = b.multiply_point(&p2);
        assert!(p3.equal(&point(5., -5., 0.)));

        let p4 = c.multiply_point(&p3);
        assert!(p4.equal(&point(15., 0., 7.)));

        assert!(c.multiply(&b).multiply(&a).multiply_point(&p).equal(&p4));
    }
}
