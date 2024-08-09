use super::Matrix;
use crate::Float;

pub fn translation(x: Float, y: Float, z: Float) -> Matrix<4> {
    Matrix::new(&[
        [1.0, 0.0, 0.0, x],
        [0.0, 1.0, 0.0, y],
        [0.0, 0.0, 1.0, z],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn scaling(x: Float, y: Float, z: Float) -> Matrix<4> {
    Matrix::new(&[
        [x, 0.0, 0.0, 0.0],
        [0.0, y, 0.0, 0.0],
        [0.0, 0.0, z, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn scaling_constant(s: Float) -> Matrix<4> {
    scaling(s, s, s)
}

pub fn rotate_x(angle: Float) -> Matrix<4> {
    let (s, c) = angle.sin_cos();
    Matrix::new(&[
        [1.0, 0.0, 0.0, 0.0],
        [0.0, c, -s, 0.0],
        [0.0, s, c, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

// pub fn rotate_y(angle: Float) -> Matrix<4> {
//     let (s, c) = angle.sin_cos();
//     Matrix::new(&[
//         [c, 0.0, s, 0.0],
//         [0.0, 1.0, 0.0, 0.0],
//         [-s, 0.0, c, 0.0],
//         [0.0, 0.0, 0.0, 1.0],
//     ])
// }

// pub fn rotate_z(angle: Float) -> Matrix<4> {
//     let (s, c) = angle.sin_cos();
//     Matrix::new(&[
//         [c, -s, 0.0, 0.0],
//         [s, c, 0.0, 0.0],
//         [0.0, 0.0, 1.0, 0.0],
//         [0.0, 0.0, 0.0, 1.0],
//     ])
// }

#[cfg(test)]
mod tests {
    use super::{rotate_x, scaling, scaling_constant, translation};
    use crate::{Float, FloatExt, Tuple};

    #[test]
    fn test_translation() {
        assert_eq!(
            translation(5.0, -3.0, 2.0) * Tuple::new_point(-3.0, 4.0, 5.0),
            Tuple::new_point(2.0, 1.0, 7.0)
        );
        assert!(
            (translation(5.0, -3.0, 2.0).inverse() * Tuple::new_point(-3.0, 4.0, 5.0))
                .is_close(&Tuple::new_point(-8.0, 7.0, 3.0))
        );
        assert_eq!(
            translation(0.5, -3.25, 7.0) * Tuple::new_point(0.0, 0.0, 0.0),
            Tuple::new_point(0.5, -3.25, 7.0)
        );
        assert_eq!(
            translation(0.0, 0.0, 0.0) * Tuple::new_point(-3.125, 4.0, 5.0),
            Tuple::new_point(-3.125, 4.0, 5.0)
        );
        let v = Tuple::new_vector(-3.0, 4.0, 5.0); // not affected by translation
        assert_eq!(translation(5.0, -3.0, 2.0) * v.clone(), v);
    }

    #[test]
    fn test_scaling() {
        assert_eq!(
            scaling(2.0, 3.0, 4.0) * Tuple::new_point(-4.0, 6.0, 8.0),
            Tuple::new_point(-8.0, 18.0, 32.0)
        );
        assert_eq!(
            scaling(2.0, 3.0, 4.0) * Tuple::new_vector(-4.0, 6.0, 8.0),
            Tuple::new_vector(-8.0, 18.0, 32.0)
        );
        assert_eq!(
            scaling(2.0, 3.0, 4.0).inverse() * Tuple::new_point(-4.0, 6.0, 8.0),
            Tuple::new_point(-2.0, 2.0, 2.0)
        );
        assert_eq!(
            scaling(-1.0, 1.0, 1.0) * Tuple::new_point(-4.0, 6.0, 8.0),
            Tuple::new_point(4.0, 6.0, 8.0)
        );
    }

    #[test]
    fn test_scaling_constant() {
        assert_eq!(
            scaling_constant(0.5) * Tuple::new_point(-3.0, 4.0, 5.0),
            Tuple::new_point(-1.5, 2.0, 2.5)
        );
        assert_eq!(
            scaling_constant(-2.5) * Tuple::new_point(-3.0, 4.0, 5.0),
            Tuple::new_point(7.5, -10.0, -12.5)
        );
        assert_eq!(
            scaling_constant(1.0) * Tuple::new_point(-3.0, 4.0, 5.0),
            Tuple::new_point(-3.0, 4.0, 5.0)
        );
        assert_eq!(scaling_constant(4.2) * Tuple::zero(), Tuple::zero());
    }

    #[test]
    fn test_rotate_x() {
        let p = Tuple::new_point(3.0, 1.0, 0.0);
        assert!((rotate_x(Float::TAU) * p.clone()).is_close(&p.clone()));
        assert!((rotate_x(Float::PI) * p.clone()).is_close(&Tuple::new_point(3.0, -1.0, 0.0)));
        assert!((rotate_x(Float::PI / 2.0) * p.clone()).is_close(&Tuple::new_point(3.0, 0.0, 1.0)));
        assert!(
            (rotate_x(Float::PI / 4.0) * p.clone()).is_close(&Tuple::new_point(
                3.0,
                (0.5 as Float).sqrt(),
                (0.5 as Float).sqrt()
            ))
        );
        assert!(
            (rotate_x(Float::PI * 1.5) * Tuple::new_point(7.0, -2.0, 4.5))
                .is_close(&Tuple::new_point(7.0, 4.5, 2.0))
        );
    }
}
