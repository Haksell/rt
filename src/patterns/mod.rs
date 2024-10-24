mod solid;
mod stripe;

pub use solid::Solid;
pub use stripe::Stripe;

use crate::{color::Color, tuple::Tuple};
use std::fmt::Debug;

pub trait Pattern: Debug {
    fn color_at(&self, point: &Tuple) -> &Color;
}
