#![allow(unused)] // TODO: remove before push

mod camera;
mod canvas;
mod color;
mod computations; // TODO: find better name
mod floats;
mod lighting;
mod material;
mod matrix;
mod objects;
mod patterns;
mod point_light;
mod ray;
mod transform;
mod tuple;
mod world;

use camera::Camera;
use color::Color;
use material::Material;
use matrix::Matrix;
use minifb::{Key, Window, WindowOptions};
use objects::{Plane, Sphere};
use patterns::{Solid, Stripe};
use point_light::PointLight;
use ray::Ray;
use transform::{rotate_x, rotate_y, scale_constant, translate, view_transform};
use tuple::Tuple;
use world::World;

// TODO: args for window or PPM file or just keyboard shortcut?

fn main() {
    let camera = Camera::with_transform(
        600,
        400,
        std::f64::consts::FRAC_PI_3,
        view_transform(
            &Tuple::new_point(0., 1.5, -5.),
            &Tuple::new_point(0., 1., 0.),
            &Tuple::new_vector(0., 1., 0.),
        ),
    );
    let world = build_world();
    let canvas = camera.render(&world);

    let mut window = Window::new("rt", canvas.width, canvas.height, WindowOptions::default())
        .unwrap_or_else(|e| panic!("{}", e));
    window.set_target_fps(10); // TODO: set to 0 once incremental raytracing is implemented

    let mut buffer: Vec<u32> = vec![0; canvas.width * canvas.height];
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for (i, pixel) in buffer.iter_mut().enumerate() {
            *pixel = canvas
                .get_pixel(i % canvas.width, i / canvas.width)
                .to_u32();
        }
        window
            .update_with_buffer(&buffer, canvas.width, canvas.height)
            .unwrap();
    }
}

fn build_world() -> World {
    let floor = Plane::new(
        Matrix::identity(),
        Material {
            pattern: Box::new(Solid::new(Color::new(1., 0.9, 0.9))),
            specular: 0.,
            ..Material::default()
        },
    );
    let left_wall = Plane::new(
        translate(0., 0., 5.)
            * rotate_y(-std::f64::consts::FRAC_PI_4)
            * rotate_x(std::f64::consts::FRAC_PI_2),
        Material {
            pattern: Box::new(Stripe::new(Color::black(), Color::red())),
            specular: 0.,
            ..Material::default()
        },
    );
    let right_wall = Plane::new(
        translate(0., 0., 5.)
            * rotate_y(std::f64::consts::FRAC_PI_4)
            * rotate_x(std::f64::consts::FRAC_PI_2),
        Material {
            pattern: Box::new(Solid::new(Color::new(1., 0.9, 0.9))),
            specular: 0.,
            ..Material::default()
        },
    );
    let middle = Sphere::new(
        translate(-0.5, 1., 0.5),
        Material {
            pattern: Box::new(Stripe::new(Color::black(), Color::red())),
            diffuse: 0.7,
            specular: 0.3,
            ..Material::default()
        },
    );
    let right = Sphere::new(
        translate(1.5, 0.5, -0.5) * scale_constant(0.5),
        Material {
            pattern: Box::new(Solid::new(Color::new(0.5, 1., 0.1))),
            ..middle.material
        },
    );
    let left = Sphere::new(
        translate(-1.5, 0.33, -0.75) * scale_constant(0.33),
        Material {
            pattern: Box::new(Solid::new(Color::new(1., 0.8, 0.1))),
            ..middle.material
        },
    );
    World::new(
        vec![
            Box::new(floor),
            Box::new(left_wall),
            Box::new(right_wall),
            Box::new(middle),
            Box::new(right),
            Box::new(left),
        ],
        vec![PointLight::new(
            Color::white(),
            Tuple::new_point(-10., 10., -10.),
        )],
    )
}
