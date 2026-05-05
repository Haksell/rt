#![feature(portable_simd)]

mod canvas;
mod color;
mod interval;
mod ray;
mod vec3;

use {
    crate::{canvas::Canvas, color::Color},
    clap::Parser,
    minifb::{Key, Window, WindowOptions},
    rand::{RngExt as _, rng},
    std::io::{Write as _, stdout},
};

#[derive(Debug, Parser)]
struct Args {
    #[arg(long, default_value_t = 800)]
    width: usize,
    #[arg(long, default_value_t = 600)]
    height: usize,
    #[arg(long, default_value_t = 100)]
    samples: u32,
}

fn main() {
    let args = Args::parse();
    let mut sample = 0;

    let mut window = Window::new("rt", args.width, args.height, WindowOptions::default()).unwrap();
    window.set_target_fps(1000);

    let mut canvas = Canvas::new(args.width, args.height);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if sample == args.samples || !window.is_active() || !window.is_open() {
            window.update();
            continue;
        }

        render_frame(&mut canvas, sample);
        sample += 1;
        print!("\rSample: {}/{}", sample, args.samples);
        stdout().flush().unwrap();
        let buffer = canvas.to_buffer();
        window
            .update_with_buffer(&buffer, args.width, args.height)
            .unwrap();
        if sample == args.samples {
            println!("\nCompleted.");
        }
    }
}

fn render_frame(canvas: &mut Canvas, sample: u32) {
    let mut rng = rng();
    for y in 0..canvas.height() {
        for x in 0..canvas.width() {
            let sample_color = Color::new(rng.random(), rng.random(), rng.random());
            let mean_color = (canvas[(x, y)] * sample as f32 + sample_color) / (sample + 1) as f32;
            canvas[(x, y)] = mean_color;
        }
    }
}

// Vec3 ray_color(const Ray* ray, const World* world) {
//     Hit hit = {};
//     if (hit_world(world, ray, (Interval){0, FLT_MAX}, &hit))
//         return 0.5 * (hit.normal + 1);

//     // sky gradient
//     Vec3 unit_direction = vec3_unit(ray->direction);
//     f32 a = 0.5 * (unit_direction[Y] + 1);
//     Vec3 sky_color = (Vec3){0.5, 0.7, 1.0};
//     return vec3_lerp(WHITE, sky_color, a);
// }

// Vec3 get_ray_direction(const Camera* camera, usize x, usize y) {
//     f32 x_sample = (f32)x + random_f32();
//     f32 y_sample = (f32)y + random_f32();
//     f32 viewport_x = camera->viewport_width * (x_sample / (f32)WINDOW_WIDTH - 0.5);
//     f32 viewport_y = camera->viewport_height * (0.5 - y_sample / (f32)WINDOW_HEIGHT);
//     f32 viewport_z = -camera->focal_length;
//     return (Vec3){viewport_x, viewport_y, viewport_z};
// }
