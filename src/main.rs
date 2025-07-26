#![allow(unused)] // TODO: remove before push

mod camera;
mod canvas;
mod color;
mod computations;
mod lighting;
mod material;
#[macro_use]
mod math;
mod objects;
mod patterns;
mod ray;
mod world;

use {
    crate::{
        camera::Camera,
        canvas::Canvas,
        color::Color,
        lighting::PointLight,
        material::Material,
        math::{
            Matrix,
            transform::{
                rotate_x, rotate_y, rotate_z, scale, scale_constant, translate, view_transform,
            },
        },
        objects::{Object, Plane, Sphere},
        patterns::{Solid, Stripe},
        world::World,
    },
    minifb::{Key, Window, WindowOptions},
    std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, TAU},
};

fn main() {
    let camera = Camera::new(
        800,
        600,
        FRAC_PI_3,
        view_transform(
            &point![0.0, 1.5, -5.0],
            &point![0.0, 1.0, 0.0],
            &vector![0.0, 1.0, 0.0],
        ),
    );
    let world = build_world();
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

fn wall_material() -> Material {
    Material {
        pattern: Box::new(Solid::new(Color::new(1.0, 0.9, 0.9))),
        specular: 0.0,
        ..Default::default()
    }
}

fn build_world() -> World {
    let wall_scale = scale(10.0, 0.01, 10.0);

    let floor = Plane::new(Matrix::identity(), wall_material());
    let left_wall = Plane::new(
        translate(0., 0., 5.) * rotate_y(-FRAC_PI_4) * rotate_x(FRAC_PI_2),
        Material {
            pattern: Box::new(Stripe::new(
                Color::red(),
                Color::white(),
                translate(0., 0., 5.) * rotate_y(-FRAC_PI_4) * rotate_x(FRAC_PI_2),
            )),
            ..wall_material()
        },
    );
    let right_wall = Plane::new(
        translate(0., 0., 5.) * rotate_y(FRAC_PI_4) * rotate_x(FRAC_PI_2),
        wall_material(),
    );

    let middle = Sphere::new(
        translate(-0.5, 1., 0.5),
        Material {
            pattern: Box::new(Stripe::default()),
            diffuse: 0.7,
            specular: 0.3,
            ..Default::default()
        },
    );
    let right = Sphere::new(
        translate(1.5, 0.5, -0.5) * scale_constant(0.5),
        Material {
            pattern: Box::new(Solid::new(Color::new(0.5, 1.0, 0.1))),
            ..*middle.get_material()
        },
    );
    let left = Sphere::new(
        translate(-1.5, 0.33, -0.75) * scale_constant(0.33),
        Material {
            pattern: Box::new(Solid::new(Color::new(1.0, 0.8, 0.1))),
            ..*middle.get_material()
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
        vec![PointLight::new(Color::white(), point![-10., 10., -10.])],
    )
}
