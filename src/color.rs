use crate::floats::is_close;

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

    pub fn red() -> Self {
        Self::new(1., 0., 0.)
    }

    pub fn green() -> Self {
        Self::new(0., 1., 0.)
    }

    pub fn blue() -> Self {
        Self::new(0., 0., 1.)
    }

    pub fn cyan() -> Self {
        Self::new(0., 1., 1.)
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

macro_rules! impl_color_color {
    ($lhs:ty, $rhs:ty) => {
        impl core::ops::Add<$rhs> for $lhs {
            type Output = Color;

            #[inline]
            fn add(self, rhs: $rhs) -> Self::Output {
                Self::Output {
                    r: self.r + rhs.r,
                    g: self.g + rhs.g,
                    b: self.b + rhs.b,
                }
            }
        }

        impl core::ops::Sub<$rhs> for $lhs {
            type Output = Color;

            #[inline]
            fn sub(self, rhs: $rhs) -> Self::Output {
                Self::Output {
                    r: self.r - rhs.r,
                    g: self.g - rhs.g,
                    b: self.b - rhs.b,
                }
            }
        }

        impl core::ops::Mul<$rhs> for $lhs {
            type Output = Color;

            #[inline]
            fn mul(self, rhs: $rhs) -> Self::Output {
                Self::Output {
                    r: self.r * rhs.r,
                    g: self.g * rhs.g,
                    b: self.b * rhs.b,
                }
            }
        }
    };
}
impl_color_color!(Color, Color);
impl_color_color!(Color, &Color);
impl_color_color!(&Color, Color);
impl_color_color!(&Color, &Color);

macro_rules! impl_color_float {
    ($lhs:ty, $rhs:ty) => {
        impl core::ops::Mul<$rhs> for $lhs {
            type Output = Color;

            #[inline]
            fn mul(self, scalar: $rhs) -> Self::Output {
                Self::Output {
                    r: self.r * scalar,
                    g: self.g * scalar,
                    b: self.b * scalar,
                }
            }
        }

        impl core::ops::Mul<$lhs> for $rhs {
            type Output = Color;

            #[inline]
            fn mul(self, color: $lhs) -> Self::Output {
                color * self
            }
        }

        impl core::ops::Div<$rhs> for $lhs {
            type Output = Color;

            #[inline]
            fn div(self, divisor: $rhs) -> Self::Output {
                self * (1. / divisor)
            }
        }
    };
}
impl_color_float!(Color, f64);
impl_color_float!(&Color, f64);
impl_color_float!(Color, &f64);
impl_color_float!(&Color, &f64);

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
        assert_eq!(
            Color::red() + &Color::green() + Color::blue(),
            Color::white()
        );
        assert!(
            (Color::new(0.9, 0.6, 0.75) + Color::new(0.7, 0.1, 0.25))
                .is_close(&Color::new(1.6, 0.7, 1.))
        );
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(
            &Color::white() - Color::green() - Color::blue(),
            Color::red()
        );
        assert!(
            (&Color::new(0.9, 0.6, 0.75) - Color::new(0.7, 0.1, 0.25))
                .is_close(&Color::new(0.2, 0.5, 0.5))
        );
    }

    #[test]
    fn test_scaling() {
        assert!((Color::new(0.9, 0.6, 0.75) * &0.5).is_close(&Color::new(0.45, 0.3, 0.375)));
        assert!((3. * Color::new(0.9, 0.6, 0.75)).is_close(&Color::new(2.7, 1.8, 2.25)));
    }

    #[test]
    fn test_division() {
        assert!((&Color::new(0.9, 0.6, 0.75) / &2.).is_close(&Color::new(0.45, 0.3, 0.375)));
    }

    #[test]
    fn test_multiplication() {
        assert!(
            (Color::new(0.9, 0.6, 0.75) * Color::new(0.7, 0.1, 0.25))
                .is_close(&Color::new(0.63, 0.06, 0.1875))
        );
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
