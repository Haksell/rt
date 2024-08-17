// TODO: in transform.rs, or mod transform, or mod camera

use crate::{matrix::Matrix, Tuple};

pub fn view_transform(from: &Tuple, to: &Tuple, up: &Tuple) -> Matrix<4> {
    let forward = (to.clone() - from.clone()).normalize();
    let left = forward.cross(&up.normalize());
    let true_up = left.cross(&forward);
    Matrix::new([
        [left.x, left.y, left.z, -from.dot(&left)],
        [true_up.x, true_up.y, true_up.z, -from.dot(&true_up)],
        [-forward.x, -forward.y, -forward.z, from.dot(&forward)],
        [0., 0., 0., 1.],
    ])
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
                [1., 0., 0., 0.],
                [0., 1., 0., 0.],
                [0., 0., 1., -8.],
                [0., 0., 0., 1.],
            ])
        );
    }

    #[test]
    fn test_view_transform_complete() {
        let from = Tuple::new_point(1., 3., 2.);
        let to = Tuple::new_point(4., -2., 8.);
        let up = Tuple::new_vector(1., 1., 0.);
        assert!(view_transform(&from, &to, &up).is_close(&Matrix::new([
            [-0.50709254, 0.50709254, 0.6761234, -2.366432],
            [0.76771593, 0.6060915, 0.121218294, -2.828427],
            [-0.35856858, 0.5976143, -0.71713716, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ])));
    }
}
