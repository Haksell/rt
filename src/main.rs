#![feature(portable_simd)]

mod camera;
mod canvas;
mod color;
mod interval;
mod math;
mod ray;
mod vec3;

use {
    crate::{camera::Camera, canvas::Canvas, color::Color, math::lerp, ray::Ray, vec3::Vec3},
    clap::Parser,
    minifb::{Key, Window, WindowOptions},
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
    let camera = Camera::new(args.width, args.height);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if sample == args.samples || !window.is_active() || !window.is_open() {
            window.update();
            continue;
        }

        render_frame(&camera, &mut canvas, sample);
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

fn render_frame(camera: &Camera, canvas: &mut Canvas, sample: u32) {
    let mut ray = Ray::new(Vec3::zero(), Vec3::zero());
    for y in 0..canvas.height() {
        for x in 0..canvas.width() {
            ray.direction = camera.get_ray_direction(x, y);
            let sample_color = ray_color(&ray);
            let sum_color = canvas[(x, y)] * sample as f32 + sample_color;
            let mean_color = sum_color / (sample + 1) as f32;
            canvas[(x, y)] = mean_color;
        }
    }
}

fn ray_color(ray: &Ray) -> Color {
    // sky gradient
    let unit_direction = ray.direction.normalize();
    let a = 0.5 * (unit_direction.y() + 1.);
    let sky_color = Color::new(0.5, 0.7, 1.0);
    lerp(Color::white(), sky_color, a)
}
