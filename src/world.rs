use crate::{
    objects::{Intersection, Object, Sphere},
    transform::scale_constant,
    Color, Material, PointLight, Ray, Tuple,
};

pub struct World {
    objects: Vec<Box<dyn Object>>,
    lights: Vec<PointLight>,
}

impl World {
    pub fn empty() -> Self {
        Self {
            objects: vec![],
            lights: vec![],
        }
    }

    pub fn new(objects: Vec<Box<dyn Object>>, lights: Vec<PointLight>) -> Self {
        Self { objects, lights }
    }

    // TODO: remove
    pub fn default() -> Self {
        Self {
            objects: vec![
                Box::new(Sphere::unit(Material {
                    color: Color::new(0.8, 1.0, 0.6),
                    diffuse: 0.7,
                    specular: 0.2,
                    ..Material::default()
                })),
                Box::new(Sphere::plastic(scale_constant(0.5))),
            ],
            lights: vec![PointLight::new(
                Color::white(),
                Tuple::new_point(-10.0, 10.0, -10.0),
            )],
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut intersections = vec![];
        for object in &self.objects {
            intersections.extend(object.intersect(ray));
        }
        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        intersections
    }
}

#[cfg(test)]
mod tests {
    use super::World;
    use crate::{Ray, Tuple};

    #[test]
    fn test_world_intersect() {
        let world = World::default();
        let ray = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let intersections = world.intersect(&ray);
        assert_eq!(intersections.len(), 4);
        assert_eq!(intersections[0].t, 4.0);
        assert_eq!(intersections[1].t, 4.5);
        assert_eq!(intersections[2].t, 5.5);
        assert_eq!(intersections[3].t, 6.0);
    }
}
