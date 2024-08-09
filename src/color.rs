// TODO: reuse rt::Tuple or SIMD?

use std::ops::{Add, Mul, Sub};

use crate::{is_close, Float};

// TODO: Copy?
#[derive(Debug, PartialEq, Clone)]
pub struct Color {
    r: Float,
    g: Float,
    b: Float,
}

impl Color {
    pub fn new(r: Float, g: Float, b: Float) -> Self {
        Self { r, g, b }
    }

    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn is_close(&self, rhs: &Self) -> bool {
        is_close(self.r, rhs.r) && is_close(self.g, rhs.g) && is_close(self.b, rhs.b)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl Mul<Float> for Color {
    type Output = Self;

    fn mul(self, scalar: Float) -> Self {
        Self {
            r: self.r * scalar,
            g: self.g * scalar,
            b: self.b * scalar,
        }
    }
}

impl Mul<Color> for Float {
    type Output = Color;

    fn mul(self, color: Color) -> Color {
        Color {
            r: self * color.r,
            g: self * color.g,
            b: self * color.b,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn test_new() {
        let c = Color::new(-0.5, 0.4, 1.7);
        assert_eq!(c.r, -0.5);
        assert_eq!(c.g, 0.4);
        assert_eq!(c.b, 1.7);
    }

    #[test]
    fn test_addition() {
        assert!((Color::new(0.9, 0.6, 0.75) + Color::new(0.7, 0.1, 0.25))
            .is_close(&Color::new(1.6, 0.7, 1.0)));
    }

    #[test]
    fn test_subtraction() {
        assert!((Color::new(0.9, 0.6, 0.75) - Color::new(0.7, 0.1, 0.25))
            .is_close(&Color::new(0.2, 0.5, 0.5)));
    }

    #[test]
    fn test_scaling() {
        assert!((Color::new(0.9, 0.6, 0.75) * 0.5).is_close(&Color::new(0.45, 0.3, 0.375)));
        assert!((3.0 * Color::new(0.9, 0.6, 0.75)).is_close(&Color::new(2.7, 1.8, 2.25)));
    }

    #[test]
    fn test_multiplication() {
        assert!((Color::new(0.9, 0.6, 0.75) * Color::new(0.7, 0.1, 0.25))
            .is_close(&Color::new(0.63, 0.06, 0.1875)));
    }
}
