use point::empty_point;
use point::Point;
use utilities::equal;

#[derive(Debug)]
struct Matrix4 {
    members: [[f64; 4]; 4],
}

const IDENTITY_MATRIX: Matrix4 = Matrix4 {
    members: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ],
};

impl Matrix4 {
    pub fn new(members: [[f64; 4]; 4]) -> Matrix4 {
        Matrix4 { members: members }
    }

    pub fn empty() -> Matrix4 {
        Matrix4 {
            members: [[0.0; 4]; 4],
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
        if col + row % 2 == 0 {
            return minor;
        } else {
            return minor * -1.0;
        }
    }

    pub fn determinant(&self) -> f64 {
        let mut result = 0.0;
        for i in 0..4 {
            result = result + self.members[0][i] * self.cofactor(0, i);
        }
        result
    }
}

#[derive(Debug)]
struct Matrix3 {
    members: [[f64; 3]; 3],
}

impl Matrix3 {
    pub fn new(members: [[f64; 3]; 3]) -> Matrix3 {
        Matrix3 { members: members }
    }

    pub fn empty() -> Matrix3 {
        Matrix3::new([[0.0; 3]; 3])
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
            return minor * -1.0;
        }
    }

    pub fn determinant(&self) -> f64 {
        let mut result = 0.0;
        for i in 0..3 {
            result = result + self.members[0][i] * self.cofactor(0, i);
        }
        result
    }
}

#[derive(Debug)]
struct Matrix2 {
    members: [[f64; 2]; 2],
}

impl Matrix2 {
    pub fn new(members: [[f64; 2]; 2]) -> Matrix2 {
        Matrix2 { members: members }
    }

    pub fn empty() -> Matrix2 {
        Matrix2::new([[0.0; 2]; 2])
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
    use utilities::equal;

    #[test]
    fn test_matrix_new() {
        let mat2 = Matrix2::new([[-3.0, 5.0], [1.0, -2.0]]);

        assert!(equal(mat2.members[0][0], -3.0));
        assert!(equal(mat2.members[0][1], 5.0));
        assert!(equal(mat2.members[1][0], 1.0));
        assert!(equal(mat2.members[1][1], -2.0));

        let mat3 = Matrix3::new([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);

        assert!(equal(mat3.members[0][0], -3.0));
        assert!(equal(mat3.members[0][1], 5.0));
        assert!(equal(mat3.members[0][2], 0.0));
        assert!(equal(mat3.members[1][0], 1.0));
        assert!(equal(mat3.members[1][1], -2.0));
        assert!(equal(mat3.members[1][2], -7.0));
        assert!(equal(mat3.members[2][0], 0.0));
        assert!(equal(mat3.members[2][1], 1.0));
        assert!(equal(mat3.members[2][2], 1.0));

        let mat4 = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert!(equal(mat4.members[0][0], 1.0));
        assert!(equal(mat4.members[0][1], 2.0));
        assert!(equal(mat4.members[0][2], 3.0));
        assert!(equal(mat4.members[0][3], 4.0));
        assert!(equal(mat4.members[1][0], 5.5));
        assert!(equal(mat4.members[1][1], 6.5));
        assert!(equal(mat4.members[1][2], 7.5));
        assert!(equal(mat4.members[1][3], 8.5));
        assert!(equal(mat4.members[2][0], 9.0));
        assert!(equal(mat4.members[2][1], 10.0));
        assert!(equal(mat4.members[2][2], 11.0));
        assert!(equal(mat4.members[2][3], 12.0));
        assert!(equal(mat4.members[3][0], 13.5));
        assert!(equal(mat4.members[3][1], 14.5));
        assert!(equal(mat4.members[3][2], 15.5));
        assert!(equal(mat4.members[3][3], 16.5));
    }

    #[test]
    fn test_matrix_equals() {
        let a = Matrix2::new([[1.0, 2.0], [3.0, 4.0]]);
        let b = Matrix2::new([[1.0, 2.0], [3.0, 4.0]]);

        assert!(a.equal(&b));

        let a = Matrix2::new([[2.0, 2.0], [3.0, 4.0]]);
        let b = Matrix2::new([[1.0, 2.0], [3.0, 4.0]]);

        assert!(!a.equal(&b));

        let a = Matrix3::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        let b = Matrix3::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);

        assert!(a.equal(&b));

        let a = Matrix3::new([[2.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        let b = Matrix3::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);

        assert!(!a.equal(&b));

        let a = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        let b = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        assert!(a.equal(&b));

        let a = Matrix4::new([
            [2.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        let b = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        assert!(!a.equal(&b));
    }

    #[test]
    fn test_matrix_multiply() {
        let a = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix4::new([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);
        let c = Matrix4::new([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]);

        assert!(a.multiply(&b).equal(&c));
    }

    #[test]
    fn test_matrix_multiply_point() {
        let a = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let b = point(1.0, 2.0, 3.0);
        let c = point(18.0, 24.0, 33.0);

        assert!(a.multiply_point(&b).equal(&c));
    }

    #[test]
    fn test_matrix_multiply_identity() {
        let a = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        assert!(a.multiply(&IDENTITY_MATRIX).equal(&a));
    }

    #[test]
    fn test_matrix_transpose() {
        let a = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        let b = Matrix4::new([
            [1.0, 5.0, 9.0, 13.0],
            [2.0, 6.0, 10.0, 14.0],
            [3.0, 7.0, 11.0, 15.0],
            [4.0, 8.0, 12.0, 16.0],
        ]);

        assert!(a.transpose().equal(&b));
        assert!(IDENTITY_MATRIX.transpose().equal(&IDENTITY_MATRIX));
    }

    #[test]
    fn test_matrix_determinant() {
        let a = Matrix2 {
            members: [[1.0, 5.0], [-3.0, 2.0]],
        };

        assert!(equal(a.determinant(), 17.0));
    }

    #[test]
    fn test_matrix_submatrix() {
        let a = Matrix3 {
            members: [[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]],
        };
        let b = Matrix2 {
            members: [[-3.0, 2.0], [0.0, 6.0]],
        };

        assert!(a.submatrix(0, 2).equal(&b));

        let a = Matrix4::new([
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]);
        let b = Matrix3::new([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]]);

        assert!(a.submatrix(2, 1).equal(&b));
    }

    #[test]
    fn test_matrix_minor() {
        let a = Matrix3 {
            members: [[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]],
        };

        assert!(equal(a.minor(1, 0), 25.0));
    }

    #[test]
    fn test_matrix_3_cofactor() {
        let a = Matrix3 {
            members: [[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]],
        };

        assert!(equal(a.cofactor(0, 0), -12.0));
        assert!(equal(a.cofactor(1, 0), -25.0));
    }

    #[test]
    fn test_matrix_4_cofactor() {
        let a = Matrix4 {
            members: [
                [-2.0, -8.0, 3.0, 5.0],
                [-3.0, 1.0, 7.0, 3.0],
                [1.0, 2.0, -9.0, 6.0],
                [-6.0, 7.0, 7.0, -9.0],
            ],
        };

        assert!(equal(a.cofactor(0, 0), 690.0));
        assert!(equal(a.cofactor(0, 1), 447.0));
        assert!(equal(a.determinant(), -4071.0));
    }

    #[test]
    fn test_matrix_3_determinant() {
        let a = Matrix3 {
            members: [[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]],
        };

        assert!(equal(a.determinant(), -196.0));
    }
}
