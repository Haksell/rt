use super::{Intersection, Object};
use crate::{material::Material, Matrix, Ray, Tuple};

#[derive(Debug)]
pub struct Plane {
    pub inverse_transform: Matrix,
    pub material: Material,
}

impl Plane {
    pub fn new(transform: Matrix, material: Material) -> Self {
        Self {
            inverse_transform: transform.inverse(),
            material,
        }
    }

    pub fn default() -> Self {
        Self {
            inverse_transform: Matrix::identity(),
            material: Material::default(),
        }
    }
}

impl Object for Plane {
    fn local_intersect(&self, object_ray: &Ray) -> Vec<Intersection> {
        // TODO: is_close with 0.0 instead?
        if object_ray.direction.y == 0. {
            vec![]
        } else {
            vec![Intersection::new(
                self,
                -object_ray.origin.y / object_ray.direction.y,
            )]
        }
    }

    fn local_normal_at(&self, _: &Tuple) -> Tuple {
        Tuple::up()
    }

    fn get_inverse_transform(&self) -> &Matrix {
        &self.inverse_transform
    }

    fn get_material(&self) -> &Material {
        &self.material
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plane_local_intersect_parallel() {
        let plane = Plane::default();
        let ray = Ray::new(Tuple::new_point(0., 10., 0.), Tuple::new_vector(0., 0., 1.));
        assert_eq!(plane.local_intersect(&ray), vec![]);
    }

    #[test]
    fn test_plane_local_intersect_coplanar() {
        let plane = Plane::default();
        let ray = Ray::new(Tuple::new_point(0., 0., 0.), Tuple::new_vector(0., 0., 1.));
        assert_eq!(plane.local_intersect(&ray), vec![]);
    }

    #[test]
    fn test_plane_local_intersect_above() {
        let plane = Plane::default();
        let ray = Ray::new(
            Tuple::new_point(0., 10., 0.),
            Tuple::new_vector(0., -1., 0.),
        );
        assert_eq!(
            plane.local_intersect(&ray),
            vec![Intersection::new(&plane, 10.)]
        );
    }

    #[test]
    fn test_plane_local_intersect_below() {
        let plane = Plane::default();
        let ray = Ray::new(
            Tuple::new_point(0., -10., 0.),
            Tuple::new_vector(0., 1., 0.),
        );
        assert_eq!(
            plane.local_intersect(&ray),
            vec![Intersection::new(&plane, 10.)]
        );
    }

    #[test]
    fn test_plane_local_normal_at() {
        let plane = Plane::default();
        assert_eq!(plane.local_normal_at(&Tuple::zero_point()), Tuple::up());
        assert_eq!(
            plane.local_normal_at(&Tuple::new_point(42., 0., -3.14)),
            Tuple::up()
        )
    }
}
