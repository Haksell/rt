use crate::{
    computations::Computations,
    lighting::shade_hit,
    objects::{hit, Intersection, Object, Sphere},
    transform::scale_constant,
    Color, Material, PointLight, Ray, Tuple,
};

#[derive(Debug)]
pub struct World {
    pub objects: Vec<Box<dyn Object>>,
    pub lights: Vec<PointLight>,
    // TODO: ambient_color: Color
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
                    color: Color::new(0.8, 1., 0.6),
                    diffuse: 0.7,
                    specular: 0.2,
                    ..Material::default()
                })),
                Box::new(Sphere::plastic(scale_constant(0.5))),
            ],
            lights: vec![PointLight::new(
                Color::white(),
                Tuple::new_point(-10., 10., -10.),
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

    pub fn color_at(&self, ray: &Ray) -> Color {
        match hit(&self.intersect(ray)) {
            None => Color::black(), // TODO: ambient color instead?
            Some(intersection) => shade_hit(self, &Computations::prepare(intersection, ray)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_intersect() {
        let world = World::default();
        let ray = Ray::new(Tuple::new_point(0., 0., -5.), Tuple::new_vector(0., 0., 1.));
        let intersections = world.intersect(&ray);
        assert_eq!(intersections.len(), 4);
        assert_eq!(intersections[0].t, 4.);
        assert_eq!(intersections[1].t, 4.5);
        assert_eq!(intersections[2].t, 5.5);
        assert_eq!(intersections[3].t, 6.);
    }

    #[test]
    fn test_color_at_void() {
        let world = World::default();
        let ray = Ray::new(Tuple::new_point(0., 0., -5.), Tuple::up());
        assert_eq!(world.color_at(&ray), Color::black());
    }

    #[test]
    fn test_color_at_sphere() {
        let world = World::default();
        let ray = Ray::new(Tuple::new_point(0., 0., -5.), Tuple::new_vector(0., 0., 1.));
        assert!(world
            .color_at(&ray)
            .is_close(&Color::new(0.3806612, 0.47582647, 0.2854959)));
    }

    #[test]
    fn test_color_at_between() {
        let world = World::default();
        let ray = Ray::new(
            Tuple::new_point(0., 0., 0.75),
            Tuple::new_vector(0., 0., -1.),
        );
        assert!(world.color_at(&ray).is_close(
            &(world.objects[1].get_material().color * world.objects[1].get_material().ambient)
        ));
    }
}
