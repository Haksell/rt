use minifb::{Key, Window, WindowOptions};
use rt::{
    objects::Sphere,
    render,
    transform::{rotate_x, rotate_y, scale, scale_constant, translate, view_transform},
    Camera, Color, Material, PointLight, Tuple, World,
};

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

    let canvas = render(&camera, &build_world());
    let mut window = Window::new("rt", canvas.width, canvas.height, WindowOptions::default())
        .unwrap_or_else(|e| panic!("{}", e));
    window.set_target_fps(12); // TODO: remove once incremental raytracing is implemented
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
    let floor = Sphere::new(
        scale(10., 0.01, 10.),
        Material {
            color: Color::new(1., 0.9, 0.9),
            specular: 0.,
            ..Material::default()
        },
    );
    let left_wall = Sphere::new(
        translate(0., 0., 5.)
            * rotate_y(-std::f64::consts::FRAC_PI_4)
            * rotate_x(std::f64::consts::FRAC_PI_2)
            * scale(10., 0.01, 10.),
        floor.material.clone(),
    );
    let right_wall = Sphere::new(
        translate(0., 0., 5.)
            * rotate_y(std::f64::consts::FRAC_PI_4)
            * rotate_x(std::f64::consts::FRAC_PI_2)
            * scale(10., 0.01, 10.),
        floor.material.clone(),
    );
    let middle = Sphere::new(
        translate(-0.5, 1., 0.5),
        Material {
            color: Color::new(0.1, 1., 0.5),
            diffuse: 0.7,
            specular: 0.3,
            ..Material::default()
        },
    );
    let right = Sphere::new(
        translate(1.5, 0.5, -0.5) * scale_constant(0.5),
        Material {
            color: Color::new(0.5, 1., 0.1),
            ..middle.material
        },
    );
    let left = Sphere::new(
        translate(-1.5, 0.33, -0.75) * scale_constant(0.33),
        Material {
            color: Color::new(1., 0.8, 0.1),
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
