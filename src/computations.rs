use crate::{
    objects::{Intersection, Object},
    Float, Ray, Tuple,
};

struct Computations<'a> {
    t: Float,
    object: &'a dyn Object,
    point: Tuple,
    eyev: Tuple,
    normalv: Tuple,
}

impl<'a> Computations<'a> {
    pub fn prepare(intersection: &'a Intersection, ray: &Ray) -> Self {
        let point = ray.position(intersection.t);
        let normalv = intersection.object.normal_at(&point);
        Self {
            t: intersection.t,
            object: intersection.object,
            point,
            eyev: -ray.direction.clone(),
            normalv,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Computations;
    use crate::{
        objects::{Intersection, Sphere},
        Ray, Tuple,
    };

    #[test]
    fn test_prepare_computations() {
        let sphere = Sphere::default();
        let comps = Computations::prepare(
            &Intersection::new(&sphere, 4.0), // TODO: Rc
            &Ray::new(
                Tuple::new_point(0.0, 0.0, -5.0),
                Tuple::new_vector(0.0, 0.0, 1.0),
            ),
        );
        assert_eq!(comps.t, 4.0);
        assert!(std::ptr::addr_eq(&comps.object, &sphere));
        assert_eq!(comps.point, Tuple::new_point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, Tuple::new_vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Tuple::new_vector(0.0, 0.0, -1.0));
    }
}
