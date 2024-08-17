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

pub type Float = f32; // TODO: try f64

// TODO: find a better way to get Float::TAU

pub trait FloatExt {
    const TAU: Self;
}

impl FloatExt for Float {
    const TAU: Float = std::f32::consts::TAU;
}

fn is_close(f1: Float, f2: Float) -> bool {
    (f1 - f2).abs() < 1e-6
}

pub fn color_at(world: &World, ray: &Ray) -> Color {
    let mut intersections = vec![];
    for object in &world.objects {
        intersections.extend(object.intersect(ray));
    }
    match hit(&intersections) {
        None => Color::black(),
        Some(intersection) => {
            let comps = Computations::prepare(intersection, ray);
            shade_hit(world, &comps)
        }
    }
}
