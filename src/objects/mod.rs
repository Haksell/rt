// mod sphere;

// pub use sphere::Sphere;

// use {
//     crate::{color::Color, matrix::Matrix, ray::Ray, tuple::Tuple},
//     std::fmt::Debug,
// };

// // TODO: automate Intersection.object
// // TODO: no lifetime?
// #[derive(Debug)]
// pub struct Intersection<'a> {
//     pub object: &'a dyn Object,
//     pub t: f64,
// }

// impl<'a> Intersection<'a> {
//     pub fn new(object: &'a dyn Object, t: f64) -> Self {
//         Self { object, t }
//     }
// }

// // turbo sus
// impl<'a> PartialEq for Intersection<'a> {
//     fn eq(&self, other: &Self) -> bool {
//         self.t == other.t && std::ptr::addr_eq(self.object, other.object)
//     }
// }

// pub trait Object: Debug {
//     fn get_inverse_transform(&self) -> &Matrix;

//     // TODO: return Vec<f64> instead?
//     fn local_intersect(&self, object_ray: &Ray) -> Vec<Intersection>;
//     fn local_normal_at(&self, object_point: &Tuple) -> Tuple;

//     fn intersect(&self, world_ray: &Ray) -> Vec<Intersection> {
//         let local_ray = world_ray.transform(&self.get_inverse_transform());
//         self.local_intersect(&local_ray)
//     }

//     fn normal_at(&self, world_point: &Tuple) -> Tuple {
//         let local_point = self.get_inverse_transform().clone() * world_point.clone();
//         let local_normal = self.local_normal_at(&local_point);
//         let mut world_normal = self.get_inverse_transform().transpose() * local_normal;
//         world_normal.w = 0.;
//         world_normal.normalize()
//     }

//     // TODO: find a way to avoid cloning the colors most of the time
//     fn color_at(&self, world_point: &Tuple) -> Color {
//         let object_point = self.get_inverse_transform().clone() * world_point.clone();
//         let pattern = &self.get_material().pattern;
//         let pattern_point = pattern.get_inverse_transform().clone() * object_point;
//         pattern.color_at(&pattern_point)
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
// }
