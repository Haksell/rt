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

#[cfg(test)]
mod tests {
    use super::translation;
    use crate::Tuple;

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
}
