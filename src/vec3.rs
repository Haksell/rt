use std::{
    ops::{Add, Div, Mul, Neg, Sub},
    simd::{Simd, num::SimdFloat as _},
};

#[derive(Copy, Clone, Debug)]
pub struct Vec3(Simd<f32, 4>);

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Simd::from_array([x, y, z, 0.0]))
    }

    pub const fn splat(v: f32) -> Self {
        Self(Simd::splat(v))
    }

    pub fn x(self) -> f32 {
        self.0[0]
    }
    pub fn y(self) -> f32 {
        self.0[1]
    }
    pub fn z(self) -> f32 {
        self.0[2]
    }

    pub fn dot(self, other: Self) -> f32 {
        (self.0 * other.0).reduce_sum()
    }

    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normalize(self) -> Self {
        self / self.length()
    }

    pub fn cross(self, other: Self) -> Self {
        Self(Simd::from_array([
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
            0.0,
        ]))
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }
}

impl Div for Vec3 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Self(self.0 / rhs.0)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self(self.0 * Simd::splat(rhs))
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(Simd::splat(self) * rhs.0)
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Self(self.0 / Simd::splat(rhs))
    }
}
