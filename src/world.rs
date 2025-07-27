use {
    crate::{
        color::Color,
        computations::Computations,
        lighting::{PointLight, is_shadowed, lighting},
        material::Material,
        math::transform::scale_constant,
        objects::{Object, Sphere},
        ray::Ray,
    },
    std::sync::LazyLock,
};

#[derive(Debug)]
pub struct World {
    // TODO: ambient_color: Color
    pub objects: Vec<Box<dyn Object>>,
    pub lights: Vec<PointLight>,
}

impl World {
    pub fn new(objects: Vec<Box<dyn Object>>, lights: Vec<PointLight>) -> Self {
        Self { objects, lights }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<(&Box<dyn Object>, f64)> {
        const MIN_HIT_DISTANCE: f64 = 1e-6;

        let mut hit_object = None;
        let mut hit_distance = std::f64::INFINITY;

        for object in &self.objects {
            for t in object.intersect(ray) {
                if t >= MIN_HIT_DISTANCE && t < hit_distance {
                    hit_distance = t;
                    hit_object = Some(object);
                }
            }
        }

        hit_object.map(|object| (object, hit_distance))
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        match self.intersect(ray) {
            None => Color::black(), // TODO: ambient color instead
            Some((object, hit_distance)) => {
                self.shade_hit(&Computations::prepare(&**object, hit_distance, ray))
            }
        }
    }

    fn shade_hit(&self, comps: &Computations) -> Color {
        lighting(
            comps.object,
            &self.lights[0], // TODO: all the lights
            &comps.point,
            &comps.eyev,
            &comps.normalv,
            is_shadowed(self, &comps.over_point),
        )
    }
}

#[cfg(test)]
pub const TESTING_WORLD: LazyLock<World> = LazyLock::new(|| World {
    objects: vec![
        Box::new(Sphere::unit(Material {
            pattern: Box::new(crate::patterns::Solid::new(Color::new(0.8, 1., 0.6))),
            diffuse: 0.7,
            specular: 0.2,
            ..Material::default()
        })),
        Box::new(Sphere::plastic(scale_constant(0.5))),
    ],
    lights: vec![PointLight::new(Color::white(), point![-10., 10., -10.])],
});

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            material::Material,
            math::{
                Tuple,
                transform::{scale_constant, translate, translate_z},
            },
            objects::Sphere,
        },
        std::sync::LazyLock,
    };

    #[test]
    fn test_intersect_all_positive() {
        let sphere = Sphere::default();
        let world = World::new(vec![Box::new(sphere)], vec![]);
        let intersection = world.intersect(&Ray::new(point![0., 0., -5.], vector![0., 0., 1.]));
        assert!(intersection.is_some());
        let (object, t) = intersection.unwrap();
        assert_eq!(t, 4.);
        assert!(std::ptr::addr_eq(object, &world.objects[0]));
    }

    #[test]
    fn test_intersect_one_positive() {
        let sphere = Sphere::default();
        let world = World::new(vec![Box::new(sphere)], vec![]);
        let intersection = world.intersect(&Ray::new(point![0., 0., 0.], vector![0., 0., 1.]));
        assert!(intersection.is_some());
        let (object, t) = intersection.unwrap();
        assert_eq!(t, 1.);
        assert!(std::ptr::addr_eq(object, &world.objects[0]));
    }

    #[test]
    fn test_intersect_all_negative() {
        let sphere = Sphere::default();
        let world = World::new(vec![Box::new(sphere)], vec![]);
        let intersection = world.intersect(&Ray::new(point![0., 0., 5.], vector![0., 0., 1.]));
        assert!(intersection.is_none());
    }

    #[test]
    fn test_intersect_more() {
        let big_sphere_around = Sphere::plastic(scale_constant(5.));
        let small_sphere_ahead = Sphere::plastic(translate(0., 0., 1.5));
        let world = World::new(
            vec![Box::new(big_sphere_around), Box::new(small_sphere_ahead)],
            vec![],
        );
        let intersection = world.intersect(&Ray::new(point![0., 0., 0.], vector![0., 0., 1.]));
        assert!(intersection.is_some());
        let (object, t) = intersection.unwrap();
        assert_eq!(t, 0.5);
        assert!(std::ptr::addr_eq(object, &world.objects[1]));
    }

    #[test]
    fn test_shade_hit_not_in_shadow() {
        let ray = Ray::new(point![0., 0., -5.], vector![0., 0., 1.]);
        let object = &TESTING_WORLD.objects[0];
        let comps = Computations::prepare(&**object, 4.0, &ray);
        assert!(
            TESTING_WORLD
                .shade_hit(&comps)
                .is_close(&Color::new(0.3806612, 0.47582647, 0.2854959))
        );
    }

    // TODO: test shade_hit inside an object

    #[test]
    fn test_shade_hit_in_shadow() {
        let world = World::new(
            vec![
                Box::new(Sphere::default()),
                Box::new(Sphere::plastic(translate_z(10.))),
            ],
            vec![PointLight::new(Color::white(), point![0., 0., -10.])],
        );
        let ray = Ray::new(point![0., 0., 5.], vector![0., 0., 1.]);
        let comps = Computations::prepare(&*world.objects[1], 4., &ray);
        assert!(world.shade_hit(&comps).is_close(&Color::new(0.1, 0.1, 0.1)));
    }

    #[test]
    fn test_color_at_void() {
        let ray = Ray::new(point![0., 0., -5.], Tuple::up());
        assert_eq!(TESTING_WORLD.color_at(&ray), Color::black());
    }

    #[test]
    fn test_color_at_sphere() {
        let ray = Ray::new(point![0., 0., -5.], vector![0., 0., 1.]);
        assert!(
            TESTING_WORLD
                .color_at(&ray)
                .is_close(&Color::new(0.3806612, 0.47582647, 0.2854959))
        );
    }

    #[test]
    fn test_color_at_between() {
        let ray = Ray::new(point![0., 0., 0.75], vector![0., 0., -1.]);
        let expected = Color::white() * TESTING_WORLD.objects[1].get_material().ambient;
        assert!(TESTING_WORLD.color_at(&ray).is_close(&expected));
    }
}
