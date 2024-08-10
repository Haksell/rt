mod canvas;
mod color;
mod lighting;
mod material;
mod matrix;
pub mod objects;
mod ray;
pub mod transform;
mod tuple;

// TODO: remove unused pub
pub use canvas::Canvas;
pub use color::Color;
pub use lighting::{lighting, PointLight};
pub use material::Material;
use matrix::Matrix;
pub use ray::Ray;
pub use tuple::Tuple;

pub type Float = f32; // TODO: try f64

// TODO: find a better way to get Float::TAU

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
