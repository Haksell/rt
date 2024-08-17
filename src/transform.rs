use super::Matrix;
use crate::Float;

// TODO: implement methods directly on Tuple when not chaining matrices?

pub fn translate(x: Float, y: Float, z: Float) -> Matrix<4> {
    Matrix::new([
        [1., 0., 0., x],
        [0., 1., 0., y],
        [0., 0., 1., z],
        [0., 0., 0., 1.],
    ])
}

pub fn scale(x: Float, y: Float, z: Float) -> Matrix<4> {
    Matrix::new([
        [x, 0., 0., 0.],
        [0., y, 0., 0.],
        [0., 0., z, 0.],
        [0., 0., 0., 1.],
    ])
}

pub fn scale_constant(s: Float) -> Matrix<4> {
    scale(s, s, s)
}

pub fn rotate_x(angle: Float) -> Matrix<4> {
    let (s, c) = angle.sin_cos();
    Matrix::new([
        [1., 0., 0., 0.],
        [0., c, -s, 0.],
        [0., s, c, 0.],
        [0., 0., 0., 1.],
    ])
}

pub fn rotate_y(angle: Float) -> Matrix<4> {
    let (s, c) = angle.sin_cos();
    Matrix::new([
        [c, 0., s, 0.],
        [0., 1., 0., 0.],
        [-s, 0., c, 0.],
        [0., 0., 0., 1.],
    ])
}

pub fn rotate_z(angle: Float) -> Matrix<4> {
    let (s, c) = angle.sin_cos();
    Matrix::new([
        [c, -s, 0., 0.],
        [s, c, 0., 0.],
        [0., 0., 1., 0.],
        [0., 0., 0., 1.],
    ])
}

// TODO: remove if unused
pub fn shear(xy: Float, xz: Float, yx: Float, yz: Float, zx: Float, zy: Float) -> Matrix<4> {
    Matrix::new([
        [1., xy, xz, 0.],
        [yx, 1., yz, 0.],
        [zx, zy, 1., 0.],
        [0., 0., 0., 1.],
    ])
}

#[cfg(test)]
mod tests {
    use super::{rotate_x, rotate_y, rotate_z, scale, scale_constant, translate};
    use crate::{transform::shear, Float, FloatExt, Tuple};

    #[test]
    fn test_translate() {
        assert_eq!(
            translate(5., -3., 2.) * Tuple::new_point(-3., 4., 5.),
            Tuple::new_point(2., 1., 7.)
        );
        assert!(
            (translate(5., -3., 2.).inverse() * Tuple::new_point(-3., 4., 5.))
                .is_close(&Tuple::new_point(-8., 7., 3.))
        );
        assert_eq!(
            translate(0.5, -3.25, 7.) * Tuple::zero_point(),
            Tuple::new_point(0.5, -3.25, 7.)
        );
        assert_eq!(
            translate(0., 0., 0.) * Tuple::new_point(-3.125, 4., 5.),
            Tuple::new_point(-3.125, 4., 5.)
        );
        let v = Tuple::new_vector(-3., 4., 5.); // not affected by translate
        assert_eq!(translate(5., -3., 2.) * v.clone(), v);
    }

    #[test]
    fn test_scale() {
        assert_eq!(
            scale(2., 3., 4.) * Tuple::new_point(-4., 6., 8.),
            Tuple::new_point(-8., 18., 32.)
        );
        assert_eq!(
            scale(2., 3., 4.) * Tuple::new_vector(-4., 6., 8.),
            Tuple::new_vector(-8., 18., 32.)
        );
        assert_eq!(
            scale(2., 3., 4.).inverse() * Tuple::new_point(-4., 6., 8.),
            Tuple::new_point(-2., 2., 2.)
        );
        assert_eq!(
            scale(-1., 1., 1.) * Tuple::new_point(-4., 6., 8.),
            Tuple::new_point(4., 6., 8.)
        );
    }

    #[test]
    fn test_scale_constant() {
        assert_eq!(
            scale_constant(0.5) * Tuple::new_point(-3., 4., 5.),
            Tuple::new_point(-1.5, 2., 2.5)
        );
        assert_eq!(
            scale_constant(-2.5) * Tuple::new_point(-3., 4., 5.),
            Tuple::new_point(7.5, -10., -12.5)
        );
        assert_eq!(
            scale_constant(1.) * Tuple::new_point(-3., 4., 5.),
            Tuple::new_point(-3., 4., 5.)
        );
        assert_eq!(
            scale_constant(4.2) * Tuple::zero_vector(),
            Tuple::zero_vector()
        );
    }

    #[test]
    fn test_rotate_x() {
        let p = Tuple::new_point(3., 1., 0.);
        assert!((rotate_x(Float::TAU) * p.clone()).is_close(&p.clone()));
        assert!((rotate_x(Float::TAU / 2.) * p.clone()).is_close(&Tuple::new_point(3., -1., 0.)));
        assert!((rotate_x(Float::TAU / 4.) * p.clone()).is_close(&Tuple::new_point(3., 0., 1.)));
        assert!(
            (rotate_x(Float::TAU / 8.) * p.clone()).is_close(&Tuple::new_point(
                3.,
                (0.5 as Float).sqrt(),
                (0.5 as Float).sqrt()
            ))
        );
        assert!(
            (rotate_x(Float::TAU * 0.75) * Tuple::new_point(7., -2., 4.5))
                .is_close(&Tuple::new_point(7., 4.5, 2.))
        );
    }

    #[test]
    fn test_rotate_y() {
        let p = Tuple::new_point(0., 4.2, 1.);
        assert!((rotate_y(Float::TAU) * p.clone()).is_close(&p.clone()));
        assert!((rotate_y(Float::TAU / 2.) * p.clone()).is_close(&Tuple::new_point(0., 4.2, -1.)));
        assert!((rotate_y(Float::TAU / 4.) * p.clone()).is_close(&Tuple::new_point(1., 4.2, 0.)));
        assert!(
            (rotate_y(Float::TAU / 8.) * p.clone()).is_close(&Tuple::new_point(
                (0.5 as Float).sqrt(),
                4.2,
                (0.5 as Float).sqrt()
            ))
        );
    }

    #[test]
    fn test_rotate_z() {
        let p = Tuple::new_point(0., 2., -1.);
        assert!((rotate_z(Float::TAU) * p.clone()).is_close(&p.clone()));
        assert!((rotate_z(Float::TAU / 2.) * p.clone()).is_close(&Tuple::new_point(0., -2., -1.)));
        assert!((rotate_z(Float::TAU / 4.) * p.clone()).is_close(&Tuple::new_point(-2., 0., -1.)));
        assert!(
            (rotate_z(Float::TAU / 8.) * p.clone()).is_close(&Tuple::new_point(
                -(2. as Float).sqrt(),
                (2. as Float).sqrt(),
                -1.
            ))
        );
    }

    #[test]
    fn test_shear() {
        assert_eq!(
            shear(0., 0., 0., 0., 0., 0.) * Tuple::new_point(2., 3., 4.),
            Tuple::new_point(2., 3., 4.)
        );
        assert_eq!(
            shear(1., 0., 0., 0., 0., 0.) * Tuple::new_point(2., 3., 4.),
            Tuple::new_point(5., 3., 4.)
        );
        assert_eq!(
            shear(0., 1., 0., 0., 0., 0.) * Tuple::new_point(2., 3., 4.),
            Tuple::new_point(6., 3., 4.)
        );
        assert_eq!(
            shear(0., 0., 1., 0., 0., 0.) * Tuple::new_point(2., 3., 4.),
            Tuple::new_point(2., 5., 4.)
        );
        assert_eq!(
            shear(0., 0., 0., 1., 0., 0.) * Tuple::new_point(2., 3., 4.),
            Tuple::new_point(2., 7., 4.)
        );
        assert_eq!(
            shear(0., 0., 0., 0., 1., 0.) * Tuple::new_point(2., 3., 4.),
            Tuple::new_point(2., 3., 6.)
        );
        assert_eq!(
            shear(0., 0., 0., 0., 0., 1.) * Tuple::new_point(2., 3., 4.),
            Tuple::new_point(2., 3., 7.)
        );
    }

    #[test]
    fn test_mixed_transforms() {
        let p1 = Tuple::new_point(1., 0., 1.);
        let a = rotate_x(Float::TAU / 4.);
        let b = scale_constant(5.);
        let c = translate(10., 5., 7.);
        let p2 = a.clone() * p1.clone();
        let p3 = b.clone() * p2;
        let p4 = c.clone() * p3;
        assert_eq!(p4, Tuple::new_point(15., 0., 7.));
        assert_eq!((c * b * a) * p1, Tuple::new_point(15., 0., 7.));
    }
}
