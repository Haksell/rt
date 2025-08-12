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

#[cfg(test)]
impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::addr_eq(self.object, other.object) && self.distance == other.distance
    }
}
