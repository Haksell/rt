use {
    super::Object,
    crate::{
        material::Material,
        math::{Matrix, Tuple},
        ray::Ray,
    },
};

#[derive(Debug)]
pub struct Plane {
    pub inverse_transform: Matrix,
    pub material: Material,
}

// TODO: make constructors part of Object trait (or at least a macro)
// TODO: accept inverse transform directly
impl Plane {
    pub fn new(transform: Matrix, material: Material) -> Self {
        Self {
            inverse_transform: transform.inverse(),
            material,
        }
    }

    pub fn horizontal(material: Material) -> Self {
        Self {
            inverse_transform: Matrix::identity(),
            material,
        }
    }

    // Is it really plastic though?
    pub fn plastic(transform: Matrix) -> Self {
        Self {
            inverse_transform: transform.inverse(),
            material: Material::default(),
        }
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            inverse_transform: Matrix::identity(),
            material: Material::default(),
        }
    }
}

impl Object for Plane {
    fn get_inverse_transform(&self) -> &Matrix {
        &self.inverse_transform
    }

    fn get_material(&self) -> &Material {
        &self.material
    }

    fn local_intersect(&self, object_ray: &Ray) -> Vec<f64> {
        if object_ray.direction.y.abs() < 1e-9 {
            vec![]
        } else {
            vec![-object_ray.origin.y / object_ray.direction.y]
        }
    }

    fn local_normal_at(&self, _: &Tuple) -> Tuple {
        Tuple::up()
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{point, vector},
    };

    #[test]
    fn test_plane_local_intersect_parallel() {
        let plane = Plane::default();
        let ray = Ray::new(point![0., 10., 0.], vector![0., 0., 1.]);
        assert_eq!(plane.local_intersect(&ray), vec![]);
    }

    #[test]
    fn test_plane_local_intersect_coplanar() {
        let plane = Plane::default();
        let ray = Ray::new(point![0., 0., 0.], vector![0., 0., 1.]);
        assert_eq!(plane.local_intersect(&ray), vec![]);
    }

    #[test]
    fn test_plane_local_intersect_above() {
        let plane = Plane::default();
        let ray = Ray::new(point![0., 10., 0.], vector![0., -1., 0.]);
        assert_eq!(plane.local_intersect(&ray), vec![10.]);
    }

    #[test]
    fn test_plane_local_intersect_below() {
        let plane = Plane::default();
        let ray = Ray::new(point![0., -10., 0.], vector![0., 1., 0.]);
        assert_eq!(plane.local_intersect(&ray), vec![10.]);
    }

    #[test]
    fn test_plane_local_normal_at() {
        let plane = Plane::default();
        assert_eq!(plane.local_normal_at(&Tuple::zero_point()), Tuple::up());
        assert_eq!(plane.local_normal_at(&point![42., 0., -3.14]), Tuple::up())
    }
}
