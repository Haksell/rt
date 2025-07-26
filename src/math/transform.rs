use crate::math::{Matrix, Tuple};

// TODO: implement transformations directly on Tuple
// TODO: implement transformations directly on Matrix

#[inline]
pub fn translate(x: f64, y: f64, z: f64) -> Matrix {
    matrix![
        [1., 0., 0., x],
        [0., 1., 0., y],
        [0., 0., 1., z],
        [0., 0., 0., 1.],
    ]
}

#[inline]
pub fn scale(x: f64, y: f64, z: f64) -> Matrix {
    matrix![
        [x, 0., 0., 0.],
        [0., y, 0., 0.],
        [0., 0., z, 0.],
        [0., 0., 0., 1.],
    ]
}

#[inline]
pub fn scale_constant(s: f64) -> Matrix {
    matrix![
        [s, 0., 0., 0.],
        [0., s, 0., 0.],
        [0., 0., s, 0.],
        [0., 0., 0., 1.],
    ]
}

#[inline]
pub fn rotate_x(angle: f64) -> Matrix {
    let (s, c) = angle.sin_cos();
    matrix![
        [1., 0., 0., 0.],
        [0., c, -s, 0.],
        [0., s, c, 0.],
        [0., 0., 0., 1.],
    ]
}

#[inline]
pub fn rotate_y(angle: f64) -> Matrix {
    let (s, c) = angle.sin_cos();
    matrix![
        [c, 0., s, 0.],
        [0., 1., 0., 0.],
        [-s, 0., c, 0.],
        [0., 0., 0., 1.],
    ]
}

#[inline]
pub fn rotate_z(angle: f64) -> Matrix {
    let (s, c) = angle.sin_cos();
    matrix![
        [c, -s, 0., 0.],
        [s, c, 0., 0.],
        [0., 0., 1., 0.],
        [0., 0., 0., 1.],
    ]
}

#[inline]
pub fn shear(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
    matrix![
        [1., xy, xz, 0.],
        [yx, 1., yz, 0.],
        [zx, zy, 1., 0.],
        [0., 0., 0., 1.],
    ]
}

// TODO: put in mod camera and not pub?
pub fn view_transform(from: &Tuple, to: &Tuple, up: &Tuple) -> Matrix {
    let forward = (to - from).normalize();
    let left = forward.cross(&up.normalize());
    let true_up = left.cross(&forward);
    matrix![
        [left.x, left.y, left.z, -from.dot(&left)],
        [true_up.x, true_up.y, true_up.z, -from.dot(&true_up)],
        [-forward.x, -forward.y, -forward.z, from.dot(&forward)],
        [0., 0., 0., 1.],
    ]
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{point, vector},
        std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2, FRAC_PI_4, PI, TAU},
    };

    #[test]
    fn test_translate() {
        assert_eq!(
            translate(5., -3., 2.) * point![-3., 4., 5.],
            point![2., 1., 7.]
        );
        assert!(
            (translate(5., -3., 2.).inverse() * point![-3., 4., 5.]).is_close(&point![-8., 7., 3.])
        );
        assert_eq!(
            translate(0.5, -3.25, 7.) * Tuple::zero_point(),
            point![0.5, -3.25, 7.]
        );
        assert_eq!(
            translate(0., 0., 0.) * point![-3.125, 4., 5.],
            point![-3.125, 4., 5.]
        );
        let v = vector![-3., 4., 5.]; // not affected by translate
        assert_eq!(translate(5., -3., 2.) * v, v);
    }

    #[test]
    fn test_scale() {
        assert_eq!(
            scale(2., 3., 4.) * point![-4., 6., 8.],
            point![-8., 18., 32.]
        );
        assert_eq!(
            scale(2., 3., 4.) * vector![-4., 6., 8.],
            vector![-8., 18., 32.]
        );
        assert_eq!(
            scale(2., 3., 4.).inverse() * point![-4., 6., 8.],
            point![-2., 2., 2.]
        );
        assert_eq!(scale(-1., 1., 1.) * point![-4., 6., 8.], point![4., 6., 8.]);
    }

    #[test]
    fn test_scale_constant() {
        assert_eq!(
            scale_constant(0.5) * point![-3., 4., 5.],
            point![-1.5, 2., 2.5]
        );
        assert_eq!(
            scale_constant(-2.5) * point![-3., 4., 5.],
            point![7.5, -10., -12.5]
        );
        assert_eq!(
            scale_constant(1.) * point![-3., 4., 5.],
            point![-3., 4., 5.]
        );
        assert_eq!(
            scale_constant(4.2) * Tuple::zero_vector(),
            Tuple::zero_vector()
        );
    }

    #[test]
    fn test_rotate_x() {
        assert!((rotate_x(TAU)).is_close(&Matrix::identity()));
        let p = point![3., 1., 0.];
        assert!((rotate_x(TAU) * p).is_close(&p));
        assert!((rotate_x(PI) * p).is_close(&point![3., -1., 0.]));
        assert!((rotate_x(FRAC_PI_2) * p).is_close(&point![3., 0., 1.]));
        assert!((rotate_x(FRAC_PI_4) * p).is_close(&point![3., FRAC_1_SQRT_2, FRAC_1_SQRT_2]));
        assert!((rotate_x(TAU * 0.75) * point![7., -2., 4.5]).is_close(&point![7., 4.5, 2.]));
    }

    #[test]
    fn test_rotate_y() {
        assert!((rotate_y(TAU)).is_close(&Matrix::identity()));
        let p = point![0., 4.2, 1.];
        assert!((rotate_y(TAU) * p).is_close(&p));
        assert!((rotate_y(TAU / 2.) * p).is_close(&point![0., 4.2, -1.]));
        assert!((rotate_y(TAU / 4.) * p).is_close(&point![1., 4.2, 0.]));
        assert!((rotate_y(TAU / 8.) * p).is_close(&point![FRAC_1_SQRT_2, 4.2, FRAC_1_SQRT_2,]));
    }

    #[test]
    fn test_rotate_z() {
        assert!((rotate_z(TAU)).is_close(&Matrix::identity()));
        let p = point![0., 2., -1.];
        assert!((rotate_z(TAU) * p).is_close(&p));
        assert!((rotate_z(TAU / 2.) * p).is_close(&point![0., -2., -1.]));
        assert!((rotate_z(TAU / 4.) * p).is_close(&point![-2., 0., -1.]));
        assert!((rotate_z(TAU / 8.) * p).is_close(&point![
            -(2. as f64).sqrt(),
            (2. as f64).sqrt(),
            -1.
        ]));
    }

    #[test]
    fn test_shear() {
        assert_eq!(
            shear(0., 0., 0., 0., 0., 0.) * point![2., 3., 4.],
            point![2., 3., 4.]
        );
        assert_eq!(
            shear(1., 0., 0., 0., 0., 0.) * point![2., 3., 4.],
            point![5., 3., 4.]
        );
        assert_eq!(
            shear(0., 1., 0., 0., 0., 0.) * point![2., 3., 4.],
            point![6., 3., 4.]
        );
        assert_eq!(
            shear(0., 0., 1., 0., 0., 0.) * point![2., 3., 4.],
            point![2., 5., 4.]
        );
        assert_eq!(
            shear(0., 0., 0., 1., 0., 0.) * point![2., 3., 4.],
            point![2., 7., 4.]
        );
        assert_eq!(
            shear(0., 0., 0., 0., 1., 0.) * point![2., 3., 4.],
            point![2., 3., 6.]
        );
        assert_eq!(
            shear(0., 0., 0., 0., 0., 1.) * point![2., 3., 4.],
            point![2., 3., 7.]
        );
        assert_eq!(
            shear(1., 1., 0., 0., 0., 0.) * point![2., 3., 4.],
            point![9., 3., 4.]
        );
    }

    #[test]
    fn test_mixed_transforms() {
        let p = point![1., 0., 1.];
        let a = rotate_x(TAU / 4.);
        let b = scale_constant(5.);
        let c = translate(10., 5., 7.);
        assert_eq!(&c * (&b * (&a * p)), point![15., 0., 7.]);
        assert_eq!(&c * ((&b * &a) * p), point![15., 0., 7.]);
        assert_eq!((((&c * &b) * &a) * p), point![15., 0., 7.]);
    }

    #[test]
    fn test_view_transform_identity() {
        let from = Tuple::zero_point();
        let to = point![0., 0., -1.];
        let up = Tuple::up();
        assert_eq!(view_transform(&from, &to, &up), Matrix::identity());
    }

    #[test]
    fn test_view_transform_behind() {
        let from = Tuple::zero_point();
        let to = point![0., 0., 1.];
        let up = Tuple::up();
        assert_eq!(
            view_transform(&from, &to, &up),
            matrix![
                [-1., 0., 0., 0.],
                [0., 1., 0., 0.],
                [0., 0., -1., 0.],
                [0., 0., 0., 1.],
            ]
        );
    }

    #[test]
    fn test_view_transform_move_world_not_eye() {
        let from = point![0., 0., 8.];
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
        let from = point![1., 3., 2.];
        let to = point![4., -2., 8.];
        let up = vector![1., 1., 0.];
        assert!(view_transform(&from, &to, &up).is_close(&Matrix::new([
            [-0.50709254, 0.50709254, 0.6761234, -2.366432],
            [0.76771593, 0.6060915, 0.121218294, -2.828427],
            [-0.35856858, 0.5976143, -0.71713716, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ])));
    }
}
