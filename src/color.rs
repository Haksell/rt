use std::{
    ops::{Add, Div, Mul},
    simd::Simd,
};

#[derive(Copy, Clone, Debug)]
pub struct Color(Simd<f32, 4>);

impl Color {
    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self(Simd::from_array([r, g, b, 0.0]))
    }

    pub const fn black() -> Self {
        Self(Simd::from_array([0.0, 0.0, 0.0, 0.0]))
    }

    pub const fn white() -> Self {
        Self(Simd::from_array([1.0, 1.0, 1.0, 0.0]))
    }

    pub fn r(self) -> f32 {
        self.0[0]
    }
    pub fn g(self) -> f32 {
        self.0[1]
    }
    pub fn b(self) -> f32 {
        self.0[2]
    }

    pub fn to_u32(self) -> u32 {
        let r = (self.r().clamp(0., 1.) * 255.).round() as u32;
        let g = (self.g().clamp(0., 1.) * 255.).round() as u32;
        let b = (self.b().clamp(0., 1.) * 255.).round() as u32;
        r << 16 | g << 8 | b
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl Mul<f32> for Color {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self(self.0 * Simd::splat(rhs))
    }
}

impl Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, rhs: Color) -> Color {
        Color(Simd::splat(self) * rhs.0)
    }
}

impl Div<f32> for Color {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Self(self.0 / Simd::splat(rhs))
    }
}
