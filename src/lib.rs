mod canvas;
mod color;
mod computations; // TODO: find better name
mod lighting;
mod material;
mod matrix;
pub mod objects;
mod ray;
pub mod transform;
mod tuple;
mod view_transform;
mod world;

// TODO: remove unused pub
pub use canvas::Canvas;
pub use color::Color;
use computations::Computations;
use lighting::shade_hit;
pub use lighting::PointLight;
pub use material::Material;
use matrix::Matrix;
use objects::hit;
pub use ray::Ray;
pub use tuple::Tuple;
pub use world::World;

fn is_close(f1: f32, f2: f32) -> bool {
    (f1 - f2).abs() < 1e-6
}

pub fn color_at(world: &World, ray: &Ray) -> Color {
    match hit(&world.intersect(ray)) {
        None => Color::black(), // TODO: ambient color instead?
        Some(intersection) => shade_hit(world, &Computations::prepare(intersection, ray)),
    }
}

#[cfg(test)]
mod tests {
    use crate::{color_at, Color, Ray, Tuple, World};

    #[test]
    fn test_color_at_void() {
        let world = World::default();
        let ray = Ray::new(Tuple::new_point(0., 0., -5.), Tuple::up());
        assert_eq!(color_at(&world, &ray), Color::black());
    }

    #[test]
    fn test_color_at_sphere() {
        let world = World::default();
        let ray = Ray::new(Tuple::new_point(0., 0., -5.), Tuple::new_vector(0., 0., 1.));
        println!("{:?}", color_at(&world, &ray));
        assert!(color_at(&world, &ray).is_close(&Color::new(0.3806612, 0.47582647, 0.2854959)));
    }

    #[test]
    fn test_color_at_between() {
        let world = World::default();
        let ray = Ray::new(
            Tuple::new_point(0., 0., 0.75),
            Tuple::new_vector(0., 0., -1.),
        );
        println!("{:?}", color_at(&world, &ray));
        println!("{:?}", world.objects[1].get_material().color);
        assert!(color_at(&world, &ray).is_close(
            &(world.objects[1].get_material().color * world.objects[1].get_material().ambient)
        ));
    }
}
