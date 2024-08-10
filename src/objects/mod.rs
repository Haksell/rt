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

pub fn hit<'a>(intersections: &'a [Intersection]) -> Option<&'a Intersection<'a>> {
    intersections
        .iter()
        .filter(|&i| i.t > 0.0)
        .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
}

#[cfg(test)]
mod tests {
    use super::{hit, Intersection, Sphere};

    #[test]
    fn test_hit_all_positive() {
        let sphere = Sphere::new();
        let intersections = vec![
            Intersection::new(&sphere, 4.0),
            Intersection::new(&sphere, 6.0),
        ];
        assert_eq!(hit(&intersections), Some(&Intersection::new(&sphere, 4.0)));
    }

    #[test]
    fn test_hit_one_positive() {
        let sphere = Sphere::new();
        let intersections = vec![
            Intersection::new(&sphere, -1.0),
            Intersection::new(&sphere, 1.0),
        ];
        assert_eq!(hit(&intersections), Some(&Intersection::new(&sphere, 1.0)));
    }

    #[test]
    fn test_hit_all_negative() {
        let sphere = Sphere::new();
        let intersections = vec![
            Intersection::new(&sphere, -6.0),
            Intersection::new(&sphere, -4.0),
        ];
        assert_eq!(hit(&intersections), None);
    }

    #[test]
    fn test_hit_more() {
        let sphere = Sphere::new();
        let intersections = vec![
            Intersection::new(&sphere, 5.0),
            Intersection::new(&sphere, 7.0),
            Intersection::new(&sphere, -3.0),
            Intersection::new(&sphere, 2.0),
        ];
        assert_eq!(hit(&intersections), Some(&Intersection::new(&sphere, 2.0)));
    }
}
