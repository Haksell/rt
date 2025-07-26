#![allow(unused)] // TODO: remove before push

mod camera;
mod canvas;
#[macro_use]
mod color;
mod floats;
mod lighting;
mod material;
#[macro_use]
mod matrix;
mod objects;
mod ray;
mod transform;
#[macro_use]
mod tuple;
mod world;

use {
    crate::{
        camera::Camera,
        canvas::Canvas,
        color::Color,
        lighting::PointLight,
        material::Material,
        matrix::Matrix,
        objects::Sphere,
        transform::{rotate_z, scale, scale_constant, translate},
        world::World,
    },
    minifb::{Key, Window, WindowOptions},
    std::f64::consts::TAU,
};

fn main() {
    let camera = Camera::new(
        640,
        480,
        std::f64::consts::FRAC_PI_3,
        translate(0., 0., -3.),
    );
    let world = World {
        objects: vec![Box::new(Sphere::new(
            // rotate_z(TAU / 8.) * scale(0.5, 1., 1.),
            Matrix::identity(),
            Material {
                color: color![1., 0.2, 1.],
                ..Material::default()
            },
        ))],
        lights: vec![PointLight::new(Color::white(), point![-10., 10., -10.])],
    };
    let canvas = camera.render(&world);
    let buffer = canvas.to_buffer();

    let mut window =
        Window::new("rt", canvas.width, canvas.height, WindowOptions::default()).unwrap();
    window.set_target_fps(30);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&buffer, canvas.width, canvas.height)
            .unwrap();
    }
}
