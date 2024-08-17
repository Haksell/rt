mod camera;
mod canvas;
mod color;
mod computations; // TODO: find better name
mod lighting;
mod material;
mod matrix;
pub mod objects;
mod point_light;
mod ray;
pub mod transform;
mod tuple;
mod world;

// TODO: remove unused pub
pub use camera::Camera;
pub use canvas::Canvas;
pub use color::Color;
use computations::Computations;
use lighting::shade_hit;
pub use material::Material;
use matrix::Matrix;
use objects::hit;
pub use point_light::PointLight;
use ray::Ray;
pub use tuple::Tuple;
pub use world::World;

fn is_close(f1: f32, f2: f32) -> bool {
    (f1 - f2).abs() < 1e-6
}

// TODO: in impl World?
fn color_at(world: &World, ray: &Ray) -> Color {
    match hit(&world.intersect(ray)) {
        None => Color::black(), // TODO: ambient color instead?
        Some(intersection) => shade_hit(world, &Computations::prepare(intersection, ray)),
    }
}

pub fn render(camera: &Camera, world: &World) -> Canvas {
    let mut canvas = Canvas::new(camera.hsize, camera.vsize);
    for py in 0..camera.vsize {
        for px in 0..camera.hsize {
            canvas.set_pixel(px, py, color_at(world, &camera.ray_for_pixel(px, py)));
        }
    }
    canvas
}

#[cfg(test)]
mod tests {
    use super::{color_at, render};
    use crate::{transform::view_transform, Camera, Color, Ray, Tuple, World};

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
        assert!(color_at(&world, &ray).is_close(&Color::new(0.3806612, 0.47582647, 0.2854959)));
    }

    #[test]
    fn test_color_at_between() {
        let world = World::default();
        let ray = Ray::new(
            Tuple::new_point(0., 0., 0.75),
            Tuple::new_vector(0., 0., -1.),
        );
        assert!(color_at(&world, &ray).is_close(
            &(world.objects[1].get_material().color * world.objects[1].get_material().ambient)
        ));
    }

    #[test]
    fn test_render_center() {
        let camera = Camera::with_transform(
            11,
            11,
            std::f32::consts::FRAC_PI_2,
            view_transform(
                &Tuple::new_point(0., 0., -5.),
                &Tuple::zero_point(),
                &Tuple::up(),
            ),
        );
        let world = World::default();
        let canvas = render(&camera, &world);
        assert_eq!(canvas.height, 11);
        assert_eq!(canvas.width, 11);
        assert!(canvas
            .get_pixel(5, 5)
            .is_close(&Color::new(0.3806612, 0.47582647, 0.2854959)));
    }
}
