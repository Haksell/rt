use minifb::{Key, Window, WindowOptions};
use rt::{
    color_at, objects::Sphere, Canvas, Color, Float, Material, PointLight, Ray, Tuple, World,
};

// TODO: args for window or PPM file or just keyboard shortcut?

const CANVAS_SIZE: usize = 400; // TODO: remove for non-square canvases
const WIDTH: usize = CANVAS_SIZE;
const HEIGHT: usize = CANVAS_SIZE;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let camera_pos = Tuple::new_point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 8.0;
    let pixel_size = wall_size / CANVAS_SIZE as Float;
    let half_wall_size = wall_size / 2.0;
    let object = Sphere::unit(Material::from_color(Color::new(1.0, 0.2, 1.0)));
    let light = PointLight::new(Color::white(), Tuple::new_point(-10.0, 10.0, -10.0));
    let world = World::new(vec![Box::new(object)], vec![light]);
    for y in 0..HEIGHT {
        let world_y = half_wall_size - pixel_size * y as Float;
        for x in 0..WIDTH {
            let world_x = -half_wall_size + pixel_size * x as Float;
            let point_on_the_wall = Tuple::new_point(world_x, world_y, wall_z);
            let ray = Ray::new(
                camera_pos.clone(),
                (point_on_the_wall - camera_pos.clone()).normalize(),
            );
            canvas.set_pixel(x, y, color_at(&world, &ray));
        }
    }

    let mut window = Window::new("rt", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| panic!("{}", e));
    window.set_target_fps(12); // TODO: remove once incremental raytracing is implemented

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for (i, pixel) in buffer.iter_mut().enumerate() {
            *pixel = canvas.get_pixel(i % WIDTH, i / WIDTH).to_u32();
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
