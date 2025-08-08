use crate::objects::Object;

#[derive(Debug)]
pub struct Intersection<'a> {
    pub object: &'a Box<dyn Object>,
    pub distance: f64,
}
impl<'a> Intersection<'a> {
    pub fn new(object: &'a Box<dyn Object>, distance: f64) -> Self {
        Self { object, distance }
    }
}
