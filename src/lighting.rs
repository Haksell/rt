use crate::{computations::Computations, ray::Ray, Color, Material, PointLight, Tuple, World};

fn lighting(
    material: &Material,
    light: &PointLight, // TODO: &[PointLight]
    point: &Tuple,
    eyev: &Tuple,
    normalv: &Tuple,
    in_shadow: bool, // TODO: not an argument
) -> Color {
    let effective_color = material.color * light.intensity;
    let ambient = effective_color * material.ambient; // TODO: shouldn't depend on light.intensity
    if in_shadow {
        return ambient;
    }
    let lightv = (light.position.clone() - point.clone()).normalize();
    let light_dot_normal = lightv.dot(normalv);
    let mut diffuse = Color::black();
    let mut specular = Color::black();
    if light_dot_normal > 0. {
        diffuse = effective_color * material.diffuse * light_dot_normal;
        let reflectv = (-lightv).reflect(normalv);
        let reflect_dot_eye = reflectv.dot(eyev);
        if reflect_dot_eye > 0. {
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = light.intensity * material.specular * factor;
        }
    }
    ambient + diffuse + specular
}

fn is_shadowed(world: &World, point: &Tuple) -> bool {
    // TODO: for all lights in world
    let v = world.lights[0].position.clone() - point.clone();
    let distance = v.magnitude();
    let direction = v.normalize();
    let ray = Ray::new(point.clone(), direction);
    let intersections = world.intersect(&ray);
    intersections.iter().any(|i| i.t > 0. && i.t < distance)
}

// TODO: in impl World
pub fn shade_hit(world: &World, comps: &Computations) -> Color {
    lighting(
        comps.object.get_material(),
        &world.lights[0], // TODO: all the lights
        &comps.point,
        &comps.eyev,
        &comps.normalv,
        is_shadowed(world, &comps.over_point),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        objects::{Intersection, Sphere},
        transform::translate,
    };

    #[test]
    fn test_lighting_light_behind_eye() {
        let material = Material::default();
        let position = Tuple::zero_point();
        let eyev = Tuple::new_vector(0., 0., -1.);
        let normalv = Tuple::new_vector(0., 0., -1.);
        let light = PointLight::new(Color::white(), Tuple::new_point(0., 0., -10.));
        assert!(
            lighting(&material, &light, &position, &eyev, &normalv, false)
                .is_close(&Color::new(1.9, 1.9, 1.9))
        )
    }

    #[test]
    fn test_lighting_eye_diagonal() {
        let material = Material::default();
        let position = Tuple::zero_point();
        let sqrt_half = std::f64::consts::FRAC_1_SQRT_2;
        let eyev = Tuple::new_vector(0., sqrt_half, -sqrt_half);
        let normalv = Tuple::new_vector(0., 0., -1.);
        let light = PointLight::new(Color::white(), Tuple::new_point(0., 0., -10.));
        // 0.9 + 0.1 = 1., specular has disappeared
        assert!(
            lighting(&material, &light, &position, &eyev, &normalv, false)
                .is_close(&Color::white())
        )
    }

    #[test]
    fn test_lighting_light_diagonal() {
        let material = Material::default();
        let position = Tuple::zero_point();
        let eyev = Tuple::new_vector(0., 0., -1.);
        let normalv = Tuple::new_vector(0., 0., -1.);
        let light = PointLight::new(Color::white(), Tuple::new_point(0., 10., -10.));
        assert!(
            lighting(&material, &light, &position, &eyev, &normalv, false)
                .is_close(&Color::new(0.73639613, 0.73639613, 0.73639613))
        )
    }

    #[test]
    fn test_lighting_both_diagonal_full_specular() {
        let material = Material::default();
        let position = Tuple::zero_point();
        let sqrt_half = std::f64::consts::FRAC_1_SQRT_2;
        let eyev = Tuple::new_vector(0., -sqrt_half, -sqrt_half);
        let normalv = Tuple::new_vector(0., 0., -1.);
        let light = PointLight::new(Color::white(), Tuple::new_point(0., 10., -10.));
        assert!(
            lighting(&material, &light, &position, &eyev, &normalv, false)
                .is_close(&Color::new(1.6363962, 1.6363962, 1.6363962))
        )
    }

    #[test]
    fn test_lighting_light_behind_surface() {
        let material = Material::default();
        let position = Tuple::zero_point();
        let eyev = Tuple::new_vector(0., 0., -1.);
        let normalv = Tuple::new_vector(0., 0., -1.);
        let light = PointLight::new(Color::white(), Tuple::new_point(0., 0., 10.));
        assert!(
            lighting(&material, &light, &position, &eyev, &normalv, false)
                .is_close(&Color::new(0.1, 0.1, 0.1))
        )
    }

    #[test]
    fn test_lighting_surface_in_shadow() {
        let material = Material::default();
        let position = Tuple::zero_point();
        let eyev = Tuple::new_vector(0., 0., -1.);
        let normalv = Tuple::new_vector(0., 0., -1.);
        let light = PointLight::new(Color::white(), Tuple::new_point(0., 0., -10.));
        assert!(
            lighting(&material, &light, &position, &eyev, &normalv, true)
                .is_close(&Color::new(0.1, 0.1, 0.1))
        )
    }

    #[test]
    fn test_shade_hit_not_in_shadow() {
        let world = World::default();
        let ray = Ray::new(Tuple::new_point(0., 0., -5.), Tuple::new_vector(0., 0., 1.));
        let intersection = Intersection::new(&*world.objects[0], 4.);
        let comps = Computations::prepare(&intersection, &ray);
        assert!(shade_hit(&world, &comps).is_close(&Color::new(0.3806612, 0.47582647, 0.2854959)));
    }

    #[test]
    fn test_shade_hit_in_shadow() {
        let world = World::new(
            vec![
                Box::new(Sphere::default()),
                Box::new(Sphere::plastic(translate(0., 0., 10.))),
            ],
            vec![PointLight::new(
                Color::white(),
                Tuple::new_point(0., 0., -10.),
            )],
        );
        let ray = Ray::new(Tuple::new_point(0., 0., 5.), Tuple::new_vector(0., 0., 1.));
        let intersection = Intersection::new(&*world.objects[1], 4.);
        let comps = Computations::prepare(&intersection, &ray);
        assert!(shade_hit(&world, &comps).is_close(&Color::new(0.1, 0.1, 0.1)));
    }

    #[test]
    fn test_is_shadowed_diagonal() {
        assert!(!is_shadowed(
            &World::default(),
            &Tuple::new_point(0., 10., 0.)
        ));
    }

    #[test]
    fn test_is_shadowed_sphere_middle() {
        assert!(is_shadowed(
            &World::default(),
            &Tuple::new_point(10., -10., 10.)
        ));
    }

    #[test]
    fn test_is_shadowed_light_middle() {
        assert!(!is_shadowed(
            &World::default(),
            &Tuple::new_point(-20., 20., -20.)
        ));
    }

    #[test]
    fn test_is_shadowed_point_middle() {
        assert!(!is_shadowed(
            &World::default(),
            &Tuple::new_point(-2., 2., -2.)
        ));
    }
}
