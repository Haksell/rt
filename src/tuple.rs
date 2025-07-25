use crate::floats::is_close;

#[derive(Debug, PartialEq, Clone)]
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

    pub fn new_point(x: f64, y: f64, z: f64) -> Self {
        Self::new(x, y, z, 1.)
    }

    pub fn new_vector(x: f64, y: f64, z: f64) -> Self {
        Self::new(x, y, z, 0.)
    }

    pub fn zero_point() -> Self {
        Self::new_point(0., 0., 0.)
    }

    pub fn zero_vector() -> Self {
        Self::new_vector(0., 0., 0.)
    }

    pub fn up() -> Self {
        Self::new_vector(0., 1., 0.)
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.
    }

    pub fn is_close(&self, rhs: &Self) -> bool {
        debug_assert!(is_close(self.w, rhs.w));
        is_close(self.x, rhs.x) && is_close(self.y, rhs.y) && is_close(self.z, rhs.z)
    }
}

macro_rules! impl_tuple_tuple {
    ($lhs:ty, $rhs:ty) => {
        impl core::ops::Add<$rhs> for $lhs {
            type Output = Tuple;

            #[inline]
            fn add(self, rhs: $rhs) -> Tuple {
                Tuple {
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
            fn sub(self, rhs: $rhs) -> Tuple {
                Tuple {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                    z: self.z - rhs.z,
                    w: self.w - rhs.w,
                }
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
                Tuple {
                    x: self.x * scalar,
                    y: self.y * scalar,
                    z: self.z * scalar,
                    w: self.w * scalar,
                }
            }
        }

        impl core::ops::Mul<$lhs> for $rhs {
            type Output = Tuple;

            #[inline]
            fn mul(self, tuple: $lhs) -> Tuple {
                Tuple {
                    x: self * tuple.x,
                    y: self * tuple.y,
                    z: self * tuple.z,
                    w: self * tuple.w,
                }
            }
        }

        impl core::ops::Div<$rhs> for $lhs {
            type Output = Tuple;

            #[inline]
            fn div(self, divisor: $rhs) -> Self::Output {
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
                Tuple {
                    x: -self.x,
                    y: -self.y,
                    z: -self.z,
                    w: -self.w,
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
        let point_constructor = Tuple::new_point(4.3, -4.2, 3.1);
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
        let vector_constructor = Tuple::new_vector(4.3, -4.2, 3.1);
        assert_eq!(vector_manual, vector_constructor);
        assert!(vector_constructor.is_vector());
        assert!(!vector_constructor.is_point());
    }

    #[test]
    fn test_addition() {
        assert_eq!(
            Tuple::new_point(3., -2., 5.) + Tuple::new_vector(-2., 3., 1.),
            Tuple::new_point(1., 1., 6.)
        );
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(
            Tuple::new_point(3., 2., 1.) - &Tuple::new_point(5., 6., 7.),
            Tuple::new_vector(-2., -4., -6.)
        );
        assert_eq!(
            &Tuple::new_vector(3., 2., 1.) - Tuple::new_vector(5., 6., 7.),
            Tuple::new_vector(-2., -4., -6.)
        );
        assert_eq!(
            &Tuple::new_point(3., 2., 1.) - &Tuple::new_vector(5., 6., 7.),
            Tuple::new_point(-2., -4., -6.)
        );
    }

    #[test]
    fn test_negation() {
        assert_eq!(
            -Tuple::new_vector(3., 2., -1.),
            Tuple::new_vector(-3., -2., 1.)
        );
        assert_eq!(-&Tuple::zero_vector(), Tuple::zero_vector());
    }

    #[test]
    fn test_scaling() {
        assert_eq!(
            &Tuple::new(1., -2., 3., -4.) * 3.5,
            Tuple::new(3.5, -7., 10.5, -14.),
        );
        assert_eq!(
            &0.5 * Tuple::new(1., -2., 3., -4.),
            Tuple::new(0.5, -1., 1.5, -2.),
        );
    }

    #[test]
    fn test_division() {
        assert_eq!(
            &Tuple::new_vector(1., -2.5, 3.25) / 2.,
            Tuple::new_vector(0.5, -1.25, 1.625),
        );
        assert_eq!(
            Tuple::new_point(1., -2.5, 3.25) / &2.,
            Tuple::new(0.5, -1.25, 1.625, 0.5),
        );
    }
}
