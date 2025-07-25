use crate::{
    matrix::{self, Matrix},
    tuple::Tuple,
};

// TODO: implement methods directly on Tuple when not chaining matrices?

pub fn translate(x: f64, y: f64, z: f64) -> Matrix {
    Matrix::new([
        [1., 0., 0., x],
        [0., 1., 0., y],
        [0., 0., 1., z],
        [0., 0., 0., 1.],
    ])
}

pub fn scale(x: f64, y: f64, z: f64) -> Matrix {
    Matrix::new([
        [x, 0., 0., 0.],
        [0., y, 0., 0.],
        [0., 0., z, 0.],
        [0., 0., 0., 1.],
    ])
}

pub fn scale_constant(s: f64) -> Matrix {
    scale(s, s, s)
}

pub fn rotate_x(angle: f64) -> Matrix {
    let (s, c) = angle.sin_cos();
    Matrix::new([
        [1., 0., 0., 0.],
        [0., c, -s, 0.],
        [0., s, c, 0.],
        [0., 0., 0., 1.],
    ])
}

pub fn rotate_y(angle: f64) -> Matrix {
    let (s, c) = angle.sin_cos();
    Matrix::new([
        [c, 0., s, 0.],
        [0., 1., 0., 0.],
        [-s, 0., c, 0.],
        [0., 0., 0., 1.],
    ])
}

pub fn rotate_z(angle: f64) -> Matrix {
    let (s, c) = angle.sin_cos();
    Matrix::new([
        [c, -s, 0., 0.],
        [s, c, 0., 0.],
        [0., 0., 1., 0.],
        [0., 0., 0., 1.],
    ])
}

pub fn shear(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
    matrix![
        [1., xy, xz, 0.],
        [yx, 1., yz, 0.],
        [zx, zy, 1., 0.],
        [0., 0., 0., 1.],
    ]
}

pub fn view_transform(from: &Tuple, to: &Tuple, up: &Tuple) -> Matrix {
    let forward = (to.clone() - from.clone()).normalize();
    let left = forward.cross(&up.normalize());
    let true_up = left.cross(&forward);
    matrix![
        [left.x, left.y, left.z, -from * left],
        [true_up.x, true_up.y, true_up.z, -from * &true_up],
        [-forward.x, -forward.y, -forward.z, from * &forward],
        [0., 0., 0., 1.],
    ]
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{point, vector},
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
        assert_eq!(translate(5., -3., 2.) * v.clone(), v);
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
        let p = point![3., 1., 0.];
        assert!((rotate_x(std::f64::consts::TAU) * p.clone()).is_close(&p.clone()));
        assert!((rotate_x(std::f64::consts::PI) * p.clone()).is_close(&point![3., -1., 0.]));
        assert!((rotate_x(std::f64::consts::FRAC_PI_2) * p.clone()).is_close(&point![3., 0., 1.]));
        assert!(
            (rotate_x(std::f64::consts::FRAC_PI_4) * p.clone()).is_close(&point![
                3.,
                std::f64::consts::FRAC_1_SQRT_2,
                std::f64::consts::FRAC_1_SQRT_2
            ])
        );
        assert!(
            (rotate_x(std::f64::consts::TAU * 0.75) * point![7., -2., 4.5])
                .is_close(&point![7., 4.5, 2.])
        );
    }

    #[test]
    fn test_rotate_y() {
        let p = point![0., 4.2, 1.];
        assert!((rotate_y(std::f64::consts::TAU) * p.clone()).is_close(&p.clone()));
        assert!((rotate_y(std::f64::consts::TAU / 2.) * p.clone()).is_close(&point![0., 4.2, -1.]));
        assert!((rotate_y(std::f64::consts::TAU / 4.) * p.clone()).is_close(&point![1., 4.2, 0.]));
        assert!(
            (rotate_y(std::f64::consts::TAU / 8.) * p.clone()).is_close(&point![
                std::f64::consts::FRAC_1_SQRT_2,
                4.2,
                std::f64::consts::FRAC_1_SQRT_2,
            ])
        );
    }

    #[test]
    fn test_rotate_z() {
        let p = point![0., 2., -1.];
        assert!((rotate_z(std::f64::consts::TAU) * p.clone()).is_close(&p.clone()));
        assert!((rotate_z(std::f64::consts::TAU / 2.) * p.clone()).is_close(&point![0., -2., -1.]));
        assert!((rotate_z(std::f64::consts::TAU / 4.) * p.clone()).is_close(&point![-2., 0., -1.]));
        assert!(
            (rotate_z(std::f64::consts::TAU / 8.) * p.clone()).is_close(&point![
                -(2. as f64).sqrt(),
                (2. as f64).sqrt(),
                -1.
            ])
        );
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
    }

    #[test]
    fn test_mixed_transforms() {
        let p1 = point![1., 0., 1.];
        let a = rotate_x(std::f64::consts::TAU / 4.);
        let b = scale_constant(5.);
        let c = translate(10., 5., 7.);
        let p2 = a.clone() * p1.clone();
        let p3 = b.clone() * p2;
        let p4 = c.clone() * p3;
        assert_eq!(p4, point![15., 0., 7.]);
        assert_eq!((c * b * a) * p1, point![15., 0., 7.]);
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
