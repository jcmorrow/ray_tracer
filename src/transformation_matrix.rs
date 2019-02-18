use matrix::Matrix4;
use point::Point;

pub struct TransformationMatrix {}

impl TransformationMatrix {
    pub fn new(from: &Point, to: &Point, up: &Point) -> Matrix4 {
        let forward = to.sub(from).normalize();
        let up_normal = up.normalize();
        let left = forward.cross(&up_normal);
        let true_up = left.cross(&forward);
        let backward = forward.multiply_scalar(-1.0);
        let negative_from = from.multiply_scalar(-1.0);

        let translation_matrix = Matrix4::new([
            [left.x, left.y, left.z, 0.0],
            [true_up.x, true_up.y, true_up.z, 0.0],
            [backward.x, backward.y, backward.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        translation_matrix.multiply(&Matrix4::translation(
            negative_from.x,
            negative_from.y,
            negative_from.z,
        ))
    }
}

#[cfg(test)]
mod tests {
    use matrix::Matrix4;
    use matrix::IDENTITY_MATRIX;
    use point::point;
    use point::vector;
    use transformation_matrix::TransformationMatrix;

    #[test]
    fn test_transformation_matrix_new_1() {
        let from = point(0.0, 0.0, 0.0);
        let to = point(0.0, 0.0, -1.0);
        let up = vector(0.0, 1.0, 0.0);

        let transformation_matrix = TransformationMatrix::new(&from, &to, &up);

        assert_eq!(IDENTITY_MATRIX, transformation_matrix);
    }

    #[test]
    fn test_transformation_matrix_new_2() {
        let from = point(0.0, 0.0, 0.0);
        let to = point(0.0, 0.0, 1.0);
        let up = vector(0.0, 1.0, 0.0);

        let transformation_matrix = TransformationMatrix::new(&from, &to, &up);

        assert_eq!(Matrix4::scaling(-1.0, 1.0, -1.0), transformation_matrix);
    }

    #[test]
    fn test_transformation_matrix_new_3() {
        let from = point(0.0, 0.0, 8.0);
        let to = point(0.0, 0.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);

        let transformation_matrix = TransformationMatrix::new(&from, &to, &up);

        assert_eq!(Matrix4::translation(0.0, 0.0, -8.0), transformation_matrix);
    }
}
