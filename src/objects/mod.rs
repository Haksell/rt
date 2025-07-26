mod sphere;

pub use sphere::Sphere;

use {
    crate::{color::Color, material::Material, matrix::Matrix, ray::Ray, tuple::Tuple},
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

    // fn color_at(&self, world_point: &Tuple) -> Color {
    //     let object_point = self.get_inverse_transform().clone() * world_point.clone();
    //     let pattern = &self.get_material().pattern;
    //     let pattern_point = pattern.get_inverse_transform().clone() * object_point;
    //     pattern.color_at(&pattern_point)
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: tests (at least the same ones as old branch)
}
