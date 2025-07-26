mod plane;
mod sphere;

pub use plane::Plane;
pub use sphere::Sphere;

use {
    crate::{color::Color, material::Material, math::Matrix, ray::Ray, math::Tuple},
    std::fmt::Debug,
};

pub trait Object: Debug {
    fn get_inverse_transform(&self) -> &Matrix;
    fn get_material(&self) -> &Material;

    fn local_intersect(&self, object_ray: &Ray) -> Vec<f64>;
    fn local_normal_at(&self, object_point: &Tuple) -> Tuple;

    fn intersect(&self, world_ray: &Ray) -> Vec<f64> {
        let local_ray = world_ray.transform(&self.get_inverse_transform());
        self.local_intersect(&local_ray)
    }

    fn normal_at(&self, world_point: &Tuple) -> Tuple {
        let local_point = self.get_inverse_transform() * world_point;
        let local_normal = self.local_normal_at(&local_point);
        let mut world_normal = local_normal * self.get_inverse_transform();
        world_normal.w = 0.;
        world_normal.normalize()
    }

    fn color_at(&self, world_point: &Tuple) -> Color {
        // let object_point = self.get_inverse_transform() * world_point;
        // let pattern = &self.get_material().pattern;
        // let pattern_point = pattern.get_inverse_transform() * object_point;
        // pattern.color_at(&pattern_point)
        self.get_material().color
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_color_at_object_transform() {
    //     assert_eq!(
    //         Sphere::new(
    //             scale_constant(2.0),
    //             Material {
    //                 pattern: Box::new(Stripe::default()),
    //                 ..Material::default()
    //             },
    //         )
    //         .color_at(&Tuple::new_point(1.5, 0.0, 0.0)),
    //         Color::white()
    //     );
    // }

    // #[test]
    // fn test_color_at_pattern_transform() {
    //     assert_eq!(
    //         Sphere::unit(Material {
    //             pattern: Box::new(Stripe::new(
    //                 Color::white(),
    //                 Color::black(),
    //                 scale_constant(2.0),
    //             )),
    //             ..Material::default()
    //         })
    //         .color_at(&Tuple::new_point(1.5, 0.0, 0.0)),
    //         Color::white()
    //     );
    // }

    // #[test]
    // fn test_color_at_both_transform() {
    //     assert_eq!(
    //         Sphere::new(
    //             scale_constant(2.0),
    //             Material {
    //                 pattern: Box::new(Stripe::new(
    //                     Color::white(),
    //                     Color::black(),
    //                     translate(0.5, 0.0, 0.0),
    //                 )),
    //                 ..Material::default()
    //             }
    //         )
    //         .color_at(&Tuple::new_point(2.5, 0.0, 0.0)),
    //         Color::white()
    //     );
    // }
}
