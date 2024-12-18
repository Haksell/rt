mod plane;
mod sphere;

pub use plane::Plane;
pub use sphere::Sphere;

use crate::{color::Color, material::Material, world, Matrix, Ray, Tuple};
use std::fmt::Debug;

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
    fn get_inverse_transform(&self) -> &Matrix;
    fn get_material(&self) -> &Material;

    // TODO: return Vec<f64> instead?
    fn local_intersect(&self, object_ray: &Ray) -> Vec<Intersection>;
    fn local_normal_at(&self, object_point: &Tuple) -> Tuple;

    fn intersect(&self, world_ray: &Ray) -> Vec<Intersection> {
        let local_ray = world_ray.transform(&self.get_inverse_transform());
        self.local_intersect(&local_ray)
    }

    fn normal_at(&self, world_point: &Tuple) -> Tuple {
        let local_point = self.get_inverse_transform().clone() * world_point.clone();
        let local_normal = self.local_normal_at(&local_point);
        let mut world_normal = self.get_inverse_transform().transpose() * local_normal;
        world_normal.w = 0.;
        world_normal.normalize()
    }

    // TODO: find a way to avoid cloning the colors most of the time
    fn color_at(&self, world_point: &Tuple) -> Color {
        let object_point = self.get_inverse_transform().clone() * world_point.clone();
        let pattern = &self.get_material().pattern;
        let pattern_point = pattern.get_inverse_transform().clone() * object_point;
        pattern.color_at(&pattern_point)
    }
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
    use super::*;
    use crate::{
        patterns::Stripe,
        transform::{scale, scale_constant, translate},
    };

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

    #[test]
    fn test_color_at_object_transform() {
        assert_eq!(
            Sphere::new(
                scale_constant(2.0),
                Material {
                    pattern: Box::new(Stripe::default()),
                    ..Material::default()
                },
            )
            .color_at(&Tuple::new_point(1.5, 0.0, 0.0)),
            Color::white()
        );
    }

    #[test]
    fn test_color_at_pattern_transform() {
        assert_eq!(
            Sphere::unit(Material {
                pattern: Box::new(Stripe::new(
                    Color::white(),
                    Color::black(),
                    scale_constant(2.0),
                )),
                ..Material::default()
            })
            .color_at(&Tuple::new_point(1.5, 0.0, 0.0)),
            Color::white()
        );
    }

    #[test]
    fn test_color_at_both_transform() {
        assert_eq!(
            Sphere::new(
                scale_constant(2.0),
                Material {
                    pattern: Box::new(Stripe::new(
                        Color::white(),
                        Color::black(),
                        translate(0.5, 0.0, 0.0),
                    )),
                    ..Material::default()
                }
            )
            .color_at(&Tuple::new_point(2.5, 0.0, 0.0)),
            Color::white()
        );
    }
}
