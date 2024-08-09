use crate::{Float, Ray, Tuple};

pub struct Sphere {
    pub center: Tuple,
    pub radius: Float,
}

impl Sphere {
    // TODO: accept arguments
    pub fn new() -> Self {
        Self {
            center: Tuple::new_point(0.0, 0.0, 0.0),
            radius: 1.0,
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Float> {
        let sphere_to_ray = ray.origin.clone() - self.center.clone();
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            vec![]
        } else {
            let neg_b = -b;
            let sqrt_discriminant = discriminant.sqrt();
            let factor = 0.5 / a;
            vec![
                (neg_b - sqrt_discriminant) * factor,
                (neg_b + sqrt_discriminant) * factor,
            ]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Sphere;
    use crate::{Ray, Tuple};

    #[test]
    fn test_sphere_intersect() {
        let sphere = Sphere::new();
        let direction = Tuple::new_vector(0.0, 0.0, 1.0);
        let ray = Ray::new(&Tuple::new_point(0.0, 0.0, -5.0), &direction.clone());
        assert_eq!(sphere.intersect(&ray), vec![4.0, 6.0]);
        let ray = Ray::new(&Tuple::new_point(0.0, 1.0, -5.0), &direction.clone());
        assert_eq!(sphere.intersect(&ray), vec![5.0, 5.0]); // at least for now
        let ray = Ray::new(&Tuple::new_point(0.0, 2.0, -5.0), &direction.clone());
        assert_eq!(sphere.intersect(&ray), vec![]);
        let ray = Ray::new(&Tuple::new_point(0.0, 0.0, 0.0), &direction.clone());
        assert_eq!(sphere.intersect(&ray), vec![-1.0, 1.0]);
        let ray = Ray::new(&Tuple::new_point(0.0, 0.0, 5.0), &direction.clone());
        assert_eq!(sphere.intersect(&ray), vec![-6.0, -4.0]);
    }
}
