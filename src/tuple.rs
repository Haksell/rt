// Do we really need w everywhere?

use crate::is_close;
use std::ops::{Add, Div, Mul, Neg, Sub};

// TODO: SIMD
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
        Self::new(0., 0., 0., 1.)
    }

    pub fn zero_vector() -> Self {
        Self::new(0., 0., 0., 0.)
    }

    pub fn up() -> Self {
        Self::new_vector(0., 1., 0.)
    }

    pub fn is_point(&self) -> bool {
        is_close(self.w, 1.0)
    }

    pub fn is_vector(&self) -> bool {
        is_close(self.w, 0.0)
    }

    pub fn is_close(&self, rhs: &Self) -> bool {
        assert!(is_close(self.w, rhs.w));
        is_close(self.x, rhs.x) && is_close(self.y, rhs.y) && is_close(self.z, rhs.z)
    }

    pub fn magnitude_squared(&self) -> f64 {
        self.dot(self)
    }

    pub fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    pub fn normalize(&self) -> Self {
        // what if self.magnitude() == 0?
        let scalar = 1. / self.magnitude();
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w, // maybe assert_eq!(self.w, 0.)
        }
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        // maybe assert_eq!(self.w, rhs.w, 0.)
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        // maybe assert_eq!(self.w, rhs.w, 0.)
        Self::new_vector(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        // TODO: assert normal is normalized?
        self.clone() - normal.clone() * 2. * self.dot(normal)
    }
}

// TODO: implement operators for &Tuple, and remove .clone() from everywhere

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

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }
}

impl Mul<Tuple> for f64 {
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

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, divisor: f64) -> Self {
        self * (1. / divisor)
    }
}

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
            Tuple::new_point(3., 2., 1.) - Tuple::new_point(5., 6., 7.),
            Tuple::new_vector(-2., -4., -6.)
        );
        assert_eq!(
            Tuple::new_vector(3., 2., 1.) - Tuple::new_vector(5., 6., 7.),
            Tuple::new_vector(-2., -4., -6.)
        );
        assert_eq!(
            Tuple::new_point(3., 2., 1.) - Tuple::new_vector(5., 6., 7.),
            Tuple::new_point(-2., -4., -6.)
        );
    }

    #[test]
    fn test_negation() {
        assert_eq!(
            -Tuple::new_vector(3., 2., -1.),
            Tuple::new_vector(-3., -2., 1.)
        );
    }

    #[test]
    fn test_scaling() {
        assert_eq!(
            Tuple::new(1., -2., 3., -4.) * 3.5,
            Tuple::new(3.5, -7., 10.5, -14.),
        );
        assert_eq!(
            0.5 * Tuple::new(1., -2., 3., -4.),
            Tuple::new(0.5, -1., 1.5, -2.),
        );
    }

    #[test]
    fn test_division() {
        assert_eq!(
            Tuple::new_vector(1., -2.5, 3.25) / 2.,
            Tuple::new_vector(0.5, -1.25, 1.625),
        );
        assert_eq!(
            Tuple::new_point(1., -2.5, 3.25) / 2.,
            Tuple::new(0.5, -1.25, 1.625, 0.5,),
        );
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(Tuple::new_vector(1., 0., 0.).magnitude(), 1.);
        assert_eq!(Tuple::up().magnitude(), 1.);
        assert_eq!(Tuple::new_vector(0., 0., 1.).magnitude(), 1.);
        assert_eq!(Tuple::new_vector(0., 3., 4.).magnitude(), 5.);
        assert_eq!(
            Tuple::new_vector(1., 2., 3.).magnitude(),
            (14. as f64).sqrt()
        );
        assert_eq!(
            Tuple::new_vector(1., -2., -3.).magnitude(),
            (14. as f64).sqrt()
        );
    }

    #[test]
    fn test_normalize() {
        assert_eq!(
            Tuple::new_vector(3., 0., 0.).normalize(),
            Tuple::new_vector(1., 0., 0.)
        );
        assert!(Tuple::new_vector(3., 4., 0.)
            .normalize()
            .is_close(&Tuple::new_vector(0.6, 0.8, 0.)));
        let sqrt14 = (14. as f64).sqrt();
        assert!(Tuple::new_vector(1., -2., -3.)
            .normalize()
            .is_close(&Tuple::new_vector(1. / sqrt14, -2. / sqrt14, -3. / sqrt14)));
    }

    #[test]
    fn test_dot() {
        assert_eq!(
            Tuple::new_vector(1., 2., 3.).dot(&Tuple::new_vector(2., 3., 4.)),
            20.,
        );
        assert_eq!(Tuple::new_vector(1., 0., 0.).dot(&Tuple::up()), 0.,);
    }

    #[test]
    fn test_cross() {
        let x = Tuple::new_vector(1., 0., 0.);
        let y = Tuple::up();
        let z = Tuple::new_vector(0., 0., 1.);
        assert_eq!(x.cross(&y), z);
        assert_eq!(y.cross(&z), x);
        assert_eq!(z.cross(&x), y);
        assert_eq!(y.cross(&x), -z.clone());
        assert_eq!(z.cross(&y), -x.clone());
        assert_eq!(x.cross(&z), -y.clone());
        let a = Tuple::new_vector(1., 2., 3.);
        let b = Tuple::new_vector(2., 3., 4.);
        assert_eq!(a.cross(&b), Tuple::new_vector(-1., 2., -1.));
        assert_eq!(b.cross(&a), Tuple::new_vector(1., -2., 1.));
    }

    #[test]
    fn test_reflect() {
        assert!(Tuple::new_vector(1., -1., 0.)
            .reflect(&Tuple::up())
            .is_close(&Tuple::new_vector(1., 1., 0.)));
        let sqrt_half = std::f64::consts::FRAC_1_SQRT_2;
        assert!(Tuple::new_vector(0., -1., 0.)
            .reflect(&Tuple::new_vector(sqrt_half, sqrt_half, 0.))
            .is_close(&Tuple::new_vector(1., 0., 0.)));
        // TODO: test with unnormalized normals?
    }
}
