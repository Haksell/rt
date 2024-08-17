use crate::{matrix::Matrix, Tuple};

pub fn view_transform(from: &Tuple, to: &Tuple, up: &Tuple) -> Matrix<4> {
    Matrix::identity()
}

#[cfg(test)]
mod tests {
    use super::view_transform;
    use crate::{matrix::Matrix, Tuple};

    #[test]
    fn test_view_transform_identity() {
        let from = Tuple::zero_point();
        let to = Tuple::new_point(0., 0., -1.);
        let up = Tuple::up();
        assert_eq!(view_transform(&from, &to, &up), Matrix::identity());
    }

    #[test]
    fn test_view_transform_behind() {
        let from = Tuple::zero_point();
        let to = Tuple::new_point(0., 0., 1.);
        let up = Tuple::up();
        assert_eq!(
            view_transform(&from, &to, &up),
            Matrix::new([
                [-1., 0., 0., 0.],
                [0., 1., 0., 0.],
                [0., 0., -1., 0.],
                [0., 0., 0., 1.],
            ])
        );
    }

    #[test]
    fn test_view_transform_move_world_not_eye() {
        let from = Tuple::new_point(0., 0., 8.);
        let to = Tuple::zero_point();
        let up = Tuple::up();
        assert_eq!(
            view_transform(&from, &to, &up),
            Matrix::new([
                [-1., 0., 0., 0.],
                [0., 1., 0., 0.],
                [0., 0., -1., 0.],
                [0., 0., 0., 1.],
            ])
        );
    }
}
