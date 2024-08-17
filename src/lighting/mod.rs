mod point_light;

use crate::{computations::Computations, Color, Material, Tuple, World};
pub use point_light::PointLight;

pub fn lighting(
    material: &Material,
    light: &PointLight,
    point: &Tuple,
    eyev: &Tuple,
    normalv: &Tuple,
) -> Color {
    let effective_color = material.color * light.intensity;
    let ambient = effective_color * material.ambient;
    let lightv = (light.position.clone() - point.clone()).normalize();
    // TODO: loop on &[PointLight]
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

// TODO: put somewhere else
pub fn shade_hit(world: &World, comps: &Computations) -> Color {
    lighting(
        comps.object.get_material(),
        &world.lights[0], // TODO: all the lights
        &comps.point,
        &comps.eyev,
        &comps.normalv,
    )
}

#[cfg(test)]
mod tests {
    use super::{lighting, point_light::PointLight, shade_hit};
    use crate::{
        computations::Computations, objects::Intersection, Color, Material, Ray, Tuple, World,
    };

    #[test]
    fn test_lighting_light_behind_eye() {
        let material = Material::default();
        let position = Tuple::zero_point();
        let eyev = Tuple::new_vector(0., 0., -1.);
        let normalv = Tuple::new_vector(0., 0., -1.);
        let light = PointLight::new(Color::white(), Tuple::new_point(0., 0., -10.));
        assert!(lighting(&material, &light, &position, &eyev, &normalv)
            .is_close(&Color::new(1.9, 1.9, 1.9)))
    }

    #[test]
    fn test_lighting_eye_diagonal() {
        let material = Material::default();
        let position = Tuple::zero_point();
        let sqrt_half = 0.5f32.sqrt();
        let eyev = Tuple::new_vector(0., sqrt_half, -sqrt_half);
        let normalv = Tuple::new_vector(0., 0., -1.);
        let light = PointLight::new(Color::white(), Tuple::new_point(0., 0., -10.));
        // 0.9 + 0.1 = 1., specular has disappeared
        assert!(lighting(&material, &light, &position, &eyev, &normalv).is_close(&Color::white()))
    }

    #[test]
    fn test_lighting_light_diagonal() {
        let material = Material::default();
        let position = Tuple::zero_point();
        let eyev = Tuple::new_vector(0., 0., -1.);
        let normalv = Tuple::new_vector(0., 0., -1.);
        let light = PointLight::new(Color::white(), Tuple::new_point(0., 10., -10.));
        assert!(lighting(&material, &light, &position, &eyev, &normalv)
            .is_close(&Color::new(0.73639613, 0.73639613, 0.73639613)))
    }

    #[test]
    fn test_lighting_both_diagonal_full_specular() {
        let material = Material::default();
        let position = Tuple::zero_point();
        let sqrt_half = 0.5f32.sqrt();
        let eyev = Tuple::new_vector(0., -sqrt_half, -sqrt_half);
        let normalv = Tuple::new_vector(0., 0., -1.);
        let light = PointLight::new(Color::white(), Tuple::new_point(0., 10., -10.));
        assert!(lighting(&material, &light, &position, &eyev, &normalv)
            .is_close(&Color::new(1.6363962, 1.6363962, 1.6363962)))
    }

    #[test]
    fn test_lighting_light_behind_surface() {
        let material = Material::default();
        let position = Tuple::zero_point();
        let eyev = Tuple::new_vector(0., 0., -1.);
        let normalv = Tuple::new_vector(0., 0., -1.);
        let light = PointLight::new(Color::white(), Tuple::new_point(0., 0., 10.));
        assert!(lighting(&material, &light, &position, &eyev, &normalv)
            .is_close(&Color::new(0.1, 0.1, 0.1)))
    }

    #[test]
    fn test_shade_hit() {
        let world = World::default();
        let ray = Ray::new(Tuple::new_point(0., 0., -5.), Tuple::new_vector(0., 0., 1.));
        let intersection = Intersection::new(&*world.objects[0], 4.);
        let comps = Computations::prepare(&intersection, &ray);
        assert!(shade_hit(&world, &comps).is_close(&Color::new(0.3806612, 0.47582647, 0.2854959)));
    }
}
