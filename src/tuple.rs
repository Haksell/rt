use crate::floats::is_close;

#[macro_export]
macro_rules! point {
    ($x:expr, $y:expr, $z:expr $(,)?) => {
        crate::tuple::Tuple::new($x, $y, $z, 1.)
    };
}

#[macro_export]
macro_rules! vector {
    ($x:expr, $y:expr, $z:expr $(,)?) => {
        crate::tuple::Tuple::new($x, $y, $z, 0.)
    };
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    pub fn zero_point() -> Self {
        point![0., 0., 0.]
    }

    pub fn zero_vector() -> Self {
        vector![0., 0., 0.]
    }

    pub fn up() -> Self {
        vector![0., 1., 0.]
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.
    }

    pub fn is_normalized(&self) -> bool {
        debug_assert!(self.is_vector());
        is_close(self.magnitude_squared(), 1.)
    }

    pub fn is_close(&self, rhs: &Self) -> bool {
        debug_assert_eq!(self.w, rhs.w);
        is_close(self.x, rhs.x) && is_close(self.y, rhs.y) && is_close(self.z, rhs.z)
    }

    pub fn magnitude_squared(&self) -> f64 {
        debug_assert!(self.is_vector());
        self * self
    }

    pub fn magnitude(&self) -> f64 {
        debug_assert!(self.is_vector());
        self.magnitude_squared().sqrt()
    }

    pub fn normalize(&self) -> Self {
        debug_assert!(self.is_vector());
        let magnitude = self.magnitude();
        debug_assert_ne!(magnitude, 0.0);
        self / magnitude
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        debug_assert!(self.is_vector());
        debug_assert!(rhs.is_vector());
        vector![
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        ]
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        debug_assert!(normal.is_vector());
        debug_assert!(normal.is_normalized());
        self - normal * 2. * self * normal
    }
}

macro_rules! impl_tuple_tuple {
    ($lhs:ty, $rhs:ty) => {
        impl core::ops::Add<$rhs> for $lhs {
            type Output = Tuple;

            #[inline]
            fn add(self, rhs: $rhs) -> Self::Output {
                Self::Output {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                    z: self.z + rhs.z,
                    w: self.w + rhs.w,
                }
            }
        }

        impl core::ops::Sub<$rhs> for $lhs {
            type Output = Tuple;

            #[inline]
            fn sub(self, rhs: $rhs) -> Self::Output {
                Self::Output {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                    z: self.z - rhs.z,
                    w: self.w - rhs.w,
                }
            }
        }

        impl core::ops::Mul<$rhs> for $lhs {
            type Output = f64;

            #[inline]
            fn mul(self, rhs: $rhs) -> Self::Output {
                self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
            }
        }
    };
}
impl_tuple_tuple!(Tuple, Tuple);
impl_tuple_tuple!(Tuple, &Tuple);
impl_tuple_tuple!(&Tuple, Tuple);
impl_tuple_tuple!(&Tuple, &Tuple);

macro_rules! impl_tuple_float {
    ($lhs:ty, $rhs:ty) => {
        impl core::ops::Mul<$rhs> for $lhs {
            type Output = Tuple;

            #[inline]
            fn mul(self, scalar: $rhs) -> Self::Output {
                debug_assert!(self.is_vector());
                Self::Output {
                    x: self.x * scalar,
                    y: self.y * scalar,
                    z: self.z * scalar,
                    w: 0.,
                }
            }
        }

        impl core::ops::Mul<$lhs> for $rhs {
            type Output = Tuple;

            #[inline]
            fn mul(self, tuple: $lhs) -> Self::Output {
                debug_assert!(tuple.is_vector());
                tuple * self
            }
        }

        impl core::ops::Div<$rhs> for $lhs {
            type Output = Tuple;

            #[inline]
            fn div(self, divisor: $rhs) -> Self::Output {
                debug_assert!(self.is_vector());
                self * (1. / divisor)
            }
        }
    };
}
impl_tuple_float!(Tuple, f64);
impl_tuple_float!(&Tuple, f64);
impl_tuple_float!(Tuple, &f64);
impl_tuple_float!(&Tuple, &f64);

macro_rules! impl_tuple {
    ($ty:ty) => {
        impl core::ops::Neg for $ty {
            type Output = Tuple;

            #[inline]
            fn neg(self) -> Self::Output {
                debug_assert!(self.is_vector(), "{self:?}");
                Self::Output {
                    x: -self.x,
                    y: -self.y,
                    z: -self.z,
                    w: 0.,
                }
            }
        }
    };
}
impl_tuple!(Tuple);
impl_tuple!(&Tuple);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point() {
        let point_manual = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.,
        };
        let point_constructor = point![4.3, -4.2, 3.1];
        assert_eq!(point_manual, point_constructor);
        assert!(point_constructor.is_point());
        assert!(!point_constructor.is_vector());
    }

    #[test]
    fn test_vector() {
        let vector_manual = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.,
        };
        let vector_constructor = vector![4.3, -4.2, 3.1];
        assert_eq!(vector_manual, vector_constructor);
        assert!(vector_constructor.is_vector());
        assert!(!vector_constructor.is_point());
    }

    #[test]
    fn test_addition() {
        assert_eq!(
            point![3., -2., 5.] + vector![-2., 3., 1.],
            point![1., 1., 6.]
        );
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(
            point![3., 2., 1.] - &point![5., 6., 7.],
            vector![-2., -4., -6.]
        );
        assert_eq!(
            &vector![3., 2., 1.] - vector![5., 6., 7.],
            vector![-2., -4., -6.]
        );
        assert_eq!(
            &point![3., 2., 1.] - &vector![5., 6., 7.],
            point![-2., -4., -6.]
        );
    }

    #[test]
    fn test_negation() {
        assert_eq!(-vector![3., 2., -1.], vector![-3., -2., 1.]);
        assert_eq!(-&Tuple::zero_vector(), Tuple::zero_vector());
    }

    #[test]
    fn test_scaling() {
        assert_eq!(&vector![1., -2., 3.] * 3.5, vector![3.5, -7., 10.5],);
        assert_eq!(&0.5 * vector![1., -2., 3.], vector![0.5, -1., 1.5],);
    }

    #[test]
    fn test_division() {
        assert_eq!(&vector![1., -2.5, 3.25] / &2., vector![0.5, -1.25, 1.625],);
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(vector![1., 0., 0.].magnitude(), 1.);
        assert_eq!(Tuple::up().magnitude(), 1.);
        assert_eq!(vector![0., 0., 1.].magnitude(), 1.);
        assert_eq!(vector![0., 3., 4.].magnitude(), 5.);
        assert_eq!(vector![1., 2., 3.].magnitude(), 14.0f64.sqrt());
        assert_eq!(vector![1., -2., -3.].magnitude(), 14.0f64.sqrt());
    }

    #[test]
    fn test_is_normalized() {
        assert!(Tuple::up().is_normalized(),);
        assert!(
            vector!(
                core::f64::consts::FRAC_1_SQRT_2,
                core::f64::consts::FRAC_1_SQRT_2,
                0.
            )
            .is_normalized(),
        );
        assert!(!vector![0.5, 0.5, 0.].is_normalized(),);
    }

    #[test]
    fn test_normalize() {
        assert_eq!(vector![3., 0., 0.].normalize(), vector![1., 0., 0.]);
        assert!(
            vector![3., 4., 0.]
                .normalize()
                .is_close(&vector![0.6, 0.8, 0.])
        );
        let sqrt14 = 14.0f64.sqrt();
        assert!(vector![1., -2., -3.].normalize().is_close(&vector!(
            1. / sqrt14,
            -2. / sqrt14,
            -3. / sqrt14
        )));
    }

    #[test]
    fn test_dot() {
        assert_eq!(&vector![1., 2., 3.] * vector![2., 3., 4.], 20.,);
        assert_eq!(vector![1., 0., 0.] * &Tuple::up(), 0.);
    }

    #[test]
    fn test_cross() {
        let x = vector![1., 0., 0.];
        let y = Tuple::up();
        let z = vector![0., 0., 1.];
        assert_eq!(x.cross(&y), z);
        assert_eq!(y.cross(&z), x);
        assert_eq!(z.cross(&x), y);
        assert_eq!(y.cross(&x), -z);
        assert_eq!(z.cross(&y), -x);
        assert_eq!(x.cross(&z), -y);
        let a = vector![1., 2., 3.];
        let b = vector![2., 3., 4.];
        let a_cross_b = vector![-1., 2., -1.];
        assert_eq!(a.cross(&b), a_cross_b);
        assert_eq!(b.cross(&a), -a_cross_b);
    }

    #[test]
    fn test_reflect() {
        assert!(
            vector![1., -1., 0.]
                .reflect(&Tuple::up())
                .is_close(&vector![1., 1., 0.])
        );
        let sqrt_half = core::f64::consts::FRAC_1_SQRT_2;
        assert!(
            vector![0., -1., 0.]
                .reflect(&vector![sqrt_half, sqrt_half, 0.])
                .is_close(&vector![1., 0., 0.])
        );
    }
}
