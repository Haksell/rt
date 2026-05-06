use std::ops::{Add, Mul};

pub fn lerp<A: Mul<f32, Output = A> + Add<A, Output = A>>(a: A, b: A, t: f32) -> A {
    a * (1. - t) + b * t
}
