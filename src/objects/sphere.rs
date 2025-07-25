// use {
//     super::Intersection,
//     crate::{matrix::Matrix, objects::Object, ray::Ray, tuple::Tuple},
// };

// #[derive(Debug)]
// pub struct Sphere {
//     pub inverse_transform: Matrix,
// }

// // TODO: make constructors part of Object trait
// // TODO: accept inverse transform directly
// // TODO: impl Default
// impl Sphere {
//     pub fn new(transform: Matrix) -> Self {
//         Self {
//             inverse_transform: transform.inverse(),
//         }
//     }
// }

// impl Object for Sphere {
//     fn local_intersect(&self, object_ray: &Ray) -> Vec<Intersection> {
//         let a = object_ray.direction.magnitude_squared();
//         let b = 2. * object_ray.direction.dot(&object_ray.origin);
//         let c = object_ray.origin.magnitude_squared() - 1.;
//         let discriminant = b * b - 4. * a * c;
//         if discriminant < 0. {
//             return vec![];
//         }
//         let neg_b = -b;
//         let sqrt_discriminant = discriminant.sqrt();
//         let factor = 0.5 / a;
//         vec![
//             Intersection::new(self, (neg_b - sqrt_discriminant) * factor),
//             Intersection::new(self, (neg_b + sqrt_discriminant) * factor),
//         ]
//     }

//     fn local_normal_at(&self, object_point: &Tuple) -> Tuple {
//         Tuple::new_vector(object_point.x, object_point.y, object_point.z)
//     }

//     fn get_inverse_transform(&self) -> &Matrix {
//         &self.inverse_transform
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
// }
