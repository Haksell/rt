// Do we really need w everywhere?

use crate::{is_close, Float};
use std::ops::{Add, Div, Mul, Neg, Sub};

// TODO: SIMD
// TODO: Copy?
#[derive(Debug, PartialEq, Clone)]
pub struct Tuple {
    x: Float,
    y: Float,
    z: Float,
    w: Float,
}

impl Tuple {
    pub fn new_point(x: Float, y: Float, z: Float) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    pub fn new_vector(x: Float, y: Float, z: Float) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    pub fn is_point(&self) -> bool {
        match self.w {
            0.0 => false,
            1.0 => true,
            _ => panic!("Tuple::w is invalid: {}", self.w),
        }
    }

    pub fn is_vector(&self) -> bool {
        match self.w {
            0.0 => true,
            1.0 => false,
            _ => panic!("Tuple::w is invalid: {}", self.w),
        }
    }

    pub fn is_close(&self, rhs: &Self) -> bool {
        is_close(self.x, rhs.x)
            && is_close(self.y, rhs.y)
            && is_close(self.z, rhs.z)
            && is_close(self.w, rhs.w) // Use == for w?
    }

    pub fn magnitude(&self) -> Float {
        (self.dot(self)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        // What if self.magnitude() == 0?
        let scalar = 1.0 / self.magnitude();
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w, // ???
        }
    }

    pub fn dot(&self, rhs: &Self) -> Float {
        // Once again, wtf am I supposed to do with w?
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        // Let's just completely ignore w
        Self::new_vector(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

// TODO: implement operators for &Tuple

impl Add for Tuple {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

// What if I want to negate a point?
impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<Float> for Tuple {
    type Output = Self;

    fn mul(self, scalar: Float) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }
}

impl Mul<Tuple> for Float {
    type Output = Tuple;

    fn mul(self, tuple: Tuple) -> Tuple {
        Tuple {
            x: self * tuple.x,
            y: self * tuple.y,
            z: self * tuple.z,
            w: self * tuple.w,
        }
    }
}

impl Div<Float> for Tuple {
    type Output = Self;

    fn div(self, divisor: Float) -> Self {
        self * (1.0 / divisor)
    }
}

#[cfg(test)]
mod tests {
    use crate::Float;

    use super::Tuple;

    #[test]
    fn test_point() {
        let point_manual = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
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
            w: 0.0,
        };
        let vector_constructor = Tuple::new_vector(4.3, -4.2, 3.1);
        assert_eq!(vector_manual, vector_constructor);
        assert!(vector_constructor.is_vector());
        assert!(!vector_constructor.is_point());
    }

    #[test]
    fn test_addition() {
        assert_eq!(
            Tuple::new_point(3.0, -2.0, 5.0) + Tuple::new_vector(-2.0, 3.0, 1.0),
            Tuple::new_point(1.0, 1.0, 6.0)
        );
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(
            Tuple::new_point(3.0, 2.0, 1.0) - Tuple::new_point(5.0, 6.0, 7.0),
            Tuple::new_vector(-2.0, -4.0, -6.0)
        );
        assert_eq!(
            Tuple::new_vector(3.0, 2.0, 1.0) - Tuple::new_vector(5.0, 6.0, 7.0),
            Tuple::new_vector(-2.0, -4.0, -6.0)
        );
        assert_eq!(
            Tuple::new_point(3.0, 2.0, 1.0) - Tuple::new_vector(5.0, 6.0, 7.0),
            Tuple::new_point(-2.0, -4.0, -6.0)
        );
    }

    #[test]
    fn test_negation() {
        assert_eq!(
            -Tuple::new_vector(3.0, 2.0, -1.0),
            Tuple::new_vector(-3.0, -2.0, 1.0)
        );
    }

    #[test]
    fn test_scaling() {
        assert_eq!(
            Tuple {
                x: 1.0,
                y: -2.0,
                z: 3.0,
                w: -4.0
            } * 3.5,
            Tuple {
                x: 3.5,
                y: -7.0,
                z: 10.5,
                w: -14.0
            },
        );
        assert_eq!(
            0.5 * Tuple {
                x: 1.0,
                y: -2.0,
                z: 3.0,
                w: -4.0
            },
            Tuple {
                x: 0.5,
                y: -1.0,
                z: 1.5,
                w: -2.0
            },
        );
    }

    #[test]
    fn test_division() {
        assert_eq!(
            Tuple::new_vector(1.0, -2.5, 3.25) / 2.0,
            Tuple::new_vector(0.5, -1.25, 1.625),
        );
        assert_eq!(
            Tuple::new_point(1.0, -2.5, 3.25) / 2.0,
            Tuple {
                x: 0.5,
                y: -1.25,
                z: 1.625,
                w: 0.5,
            },
        );
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(Tuple::new_vector(1.0, 0.0, 0.0).magnitude(), 1.0);
        assert_eq!(Tuple::new_vector(0.0, 1.0, 0.0).magnitude(), 1.0);
        assert_eq!(Tuple::new_vector(0.0, 0.0, 1.0).magnitude(), 1.0);
        assert_eq!(Tuple::new_vector(0.0, 3.0, 4.0).magnitude(), 5.0);
        assert_eq!(
            Tuple::new_vector(1.0, 2.0, 3.0).magnitude(),
            (14.0 as Float).sqrt()
        );
        assert_eq!(
            Tuple::new_vector(1.0, -2.0, -3.0).magnitude(),
            (14.0 as Float).sqrt()
        );
    }

    #[test]
    fn test_normalize() {
        assert_eq!(
            Tuple::new_vector(3.0, 0.0, 0.0).normalize(),
            Tuple::new_vector(1.0, 0.0, 0.0)
        );
        assert!(Tuple::new_vector(3.0, 4.0, 0.0)
            .normalize()
            .is_close(&Tuple::new_vector(0.6, 0.8, 0.0)));
        let sqrt14 = (14.0 as Float).sqrt();
        assert!(Tuple::new_vector(1.0, -2.0, -3.0)
            .normalize()
            .is_close(&Tuple::new_vector(
                1.0 / sqrt14,
                -2.0 / sqrt14,
                -3.0 / sqrt14
            )));
    }

    #[test]
    fn test_dot() {
        assert_eq!(
            Tuple::new_vector(1.0, 2.0, 3.0).dot(&Tuple::new_vector(2.0, 3.0, 4.0)),
            20.0,
        );
        assert_eq!(
            Tuple::new_vector(1.0, 0.0, 0.0).dot(&Tuple::new_vector(0.0, 1.0, 0.0)),
            0.0,
        );
    }

    #[test]
    fn test_cross() {
        let x = Tuple::new_vector(1.0, 0.0, 0.0);
        let y = Tuple::new_vector(0.0, 1.0, 0.0);
        let z = Tuple::new_vector(0.0, 0.0, 1.0);
        assert_eq!(x.cross(&y), z);
        assert_eq!(y.cross(&z), x);
        assert_eq!(z.cross(&x), y);
        assert_eq!(y.cross(&x), -z.clone());
        assert_eq!(z.cross(&y), -x.clone());
        assert_eq!(x.cross(&z), -y.clone());
        let a = Tuple::new_vector(1.0, 2.0, 3.0);
        let b = Tuple::new_vector(2.0, 3.0, 4.0);
        assert_eq!(a.cross(&b), Tuple::new_vector(-1.0, 2.0, -1.0));
        assert_eq!(b.cross(&a), Tuple::new_vector(1.0, -2.0, 1.0));
    }
}
