mod gradient;
mod solid;
mod stripe;

pub use gradient::Gradient;
pub use solid::Solid;
pub use stripe::Stripe;

use crate::{color::Color, matrix::Matrix, tuple::Tuple};
use std::fmt::Debug;

pub trait Pattern: Debug {
    // TODO: find a way to return reference to avoid cloning in solid, stripe, checkerboard...
    fn color_at(&self, point: &Tuple) -> Color;
    fn get_inverse_transform(&self) -> &Matrix;
}
