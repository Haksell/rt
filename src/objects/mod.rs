mod sphere;

use std::fmt::Debug;

use crate::{Float, Ray};
pub use sphere::Sphere;

// TODO: automate Intersection.object
// TODO: no lifetime?
#[derive(Debug)]
pub struct Intersection<'a> {
    pub object: &'a dyn Object,
    pub t: Float,
}

impl<'a> Intersection<'a> {
    fn new(object: &'a dyn Object, t: Float) -> Self {
        Self { object, t }
    }
}

// turbo sus
impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && std::ptr::addr_eq(self.object, other.object)
    }
}

pub trait Object: Debug {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection>;
}
