mod canvas;
mod color;
mod matrix;
pub mod transformations;
mod tuple;

pub use canvas::Canvas;
pub use color::Color;
pub use matrix::Matrix;
pub use tuple::Tuple;

type Float = f32; // TODO: try f64

// TODO: find a better way to get TAU

pub trait FloatExt {
    const TAU: Self;
}

impl FloatExt for Float {
    const TAU: Float = std::f32::consts::TAU;
}

fn is_close(f1: Float, f2: Float) -> bool {
    const EPSILON: Float = 1e-6;
    (f1 - f2).abs() < EPSILON
}
