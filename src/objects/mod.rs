mod sphere;

use std::fmt::Debug;

use crate::{material::Material, Matrix, Ray, Tuple};
pub use sphere::Sphere;

// TODO: automate Intersection.object
// TODO: no lifetime?
#[derive(Debug)]
pub struct Intersection<'a> {
    pub object: &'a dyn Object,
    pub t: f64,
}

impl<'a> Intersection<'a> {
    pub fn new(object: &'a dyn Object, t: f64) -> Self {
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
    fn normal_at(&self, point: &Tuple) -> Tuple;
    fn get_transform(&self) -> &Matrix<4>;
    fn get_material(&self) -> &Material;
}

// TODO: assume intersections is sorted and do a binary search
// TODO: put in world.rs
pub fn hit<'a>(intersections: &'a [Intersection]) -> Option<&'a Intersection<'a>> {
    intersections
        .iter()
        .filter(|&i| i.t > 0.)
        .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
}

#[cfg(test)]
mod tests {
    use super::{hit, Intersection, Sphere};

    #[test]
    fn test_hit_all_positive() {
        let sphere = Sphere::default();
        let intersections = vec![
            Intersection::new(&sphere, 4.),
            Intersection::new(&sphere, 6.),
        ];
        assert_eq!(hit(&intersections), Some(&Intersection::new(&sphere, 4.)));
    }

    #[test]
    fn test_hit_one_positive() {
        let sphere = Sphere::default();
        let intersections = vec![
            Intersection::new(&sphere, -1.),
            Intersection::new(&sphere, 1.),
        ];
        assert_eq!(hit(&intersections), Some(&Intersection::new(&sphere, 1.)));
    }

    #[test]
    fn test_hit_all_negative() {
        let sphere = Sphere::default();
        let intersections = vec![
            Intersection::new(&sphere, -6.),
            Intersection::new(&sphere, -4.),
        ];
        assert_eq!(hit(&intersections), None);
    }

    #[test]
    fn test_hit_more() {
        let sphere = Sphere::default();
        let intersections = vec![
            Intersection::new(&sphere, 5.),
            Intersection::new(&sphere, 7.),
            Intersection::new(&sphere, -3.),
            Intersection::new(&sphere, 2.),
        ];
        assert_eq!(hit(&intersections), Some(&Intersection::new(&sphere, 2.)));
    }
}
