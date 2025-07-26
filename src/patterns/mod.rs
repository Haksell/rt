mod gradient;
mod ring;
mod solid;
mod stripe;

pub use {gradient::Gradient, ring::Ring, solid::Solid, stripe::Stripe};

use {
    crate::{
        color::Color,
        math::{Matrix, Tuple},
    },
    std::fmt::Debug,
};

pub trait Pattern: Debug {
    fn color_at(&self, point: &Tuple) -> Color;
    fn get_inverse_transform(&self) -> &Matrix;
}
