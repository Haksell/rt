// TODO: reuse rt::Tuple or SIMD?

use crate::floats::is_close;
use std::ops::{Add, Mul, Sub};

// TODO: not Copy?
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn black() -> Self {
        Self::new(0., 0., 0.)
    }

    pub fn blue() -> Self {
        Self::new(0., 0., 1.)
    }

    pub fn green() -> Self {
        Self::new(0., 1., 0.)
    }

    pub fn cyan() -> Self {
        Self::new(0., 1., 1.)
    }

    pub fn red() -> Self {
        Self::new(1., 0., 0.)
    }

    pub fn magenta() -> Self {
        Self::new(1., 0., 1.)
    }

    pub fn yellow() -> Self {
        Self::new(1., 1., 0.)
    }

    pub fn white() -> Self {
        Self::new(1., 1., 1.)
    }

    pub fn is_close(&self, rhs: &Self) -> bool {
        is_close(self.r, rhs.r) && is_close(self.g, rhs.g) && is_close(self.b, rhs.b)
    }

    pub fn to_u32(&self) -> u32 {
        let r = (self.r.clamp(0., 1.) * 255.).round() as u32;
        let g = (self.g.clamp(0., 1.) * 255.).round() as u32;
        let b = (self.b.clamp(0., 1.) * 255.).round() as u32;
        (r << 16) | (g << 8) | b
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

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            r: self.r * scalar,
            g: self.g * scalar,
            b: self.b * scalar,
        }
    }
}

impl Mul<Color> for f64 {
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
    use super::*;

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
            .is_close(&Color::new(1.6, 0.7, 1.)));
    }

    #[test]
    fn test_subtraction() {
        assert!((Color::new(0.9, 0.6, 0.75) - Color::new(0.7, 0.1, 0.25))
            .is_close(&Color::new(0.2, 0.5, 0.5)));
    }

    #[test]
    fn test_scaling() {
        assert!((Color::new(0.9, 0.6, 0.75) * 0.5).is_close(&Color::new(0.45, 0.3, 0.375)));
        assert!((3. * Color::new(0.9, 0.6, 0.75)).is_close(&Color::new(2.7, 1.8, 2.25)));
    }

    #[test]
    fn test_multiplication() {
        assert!((Color::new(0.9, 0.6, 0.75) * Color::new(0.7, 0.1, 0.25))
            .is_close(&Color::new(0.63, 0.06, 0.1875)));
    }

    #[test]
    fn test_to_u32() {
        assert_eq!(Color::black().to_u32(), 0x000000);
        assert_eq!(Color::blue().to_u32(), 0x0000ff);
        assert_eq!(Color::green().to_u32(), 0x00ff00);
        assert_eq!(Color::cyan().to_u32(), 0x00ffff);
        assert_eq!(Color::red().to_u32(), 0xff0000);
        assert_eq!(Color::magenta().to_u32(), 0xff00ff);
        assert_eq!(Color::yellow().to_u32(), 0xffff00);
        assert_eq!(Color::white().to_u32(), 0xffffff);
        assert_eq!(
            Color::new(0.333, 0.667, 1.).to_u32(),
            85 << 16 | 170 << 8 | 255
        );
    }
}
