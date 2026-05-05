#![feature(portable_simd)]

mod color;
mod ray;
mod vec3;

use {
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
    samples: usize,
}

fn main() {
    let args = Args::parse();
    let mut sample = 0;

    let mut window = Window::new("rt", args.width, args.height, WindowOptions::default()).unwrap();
    window.set_target_fps(1000);

    let buffer = create_buffer(&args);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if sample == args.samples || !window.is_active() || !window.is_open() {
            window.update();
        } else {
            window
                .update_with_buffer(&buffer, args.width, args.height)
                .unwrap();
            sample += 1;
            print!("\rSample: {}/{}", sample, args.samples);
            stdout().flush().unwrap();
            if sample == args.samples {
                println!("\nCompleted.");
            }
        }
    }
}

fn create_buffer(args: &Args) -> Vec<u32> {
    let mut buffer = vec![0; args.width * args.height];

    for y in 0..args.height {
        for x in 0..args.width {
            let r = (y as f32 / args.height as f32 * 255.999) as u32;
            let g = (x as f32 / args.width as f32 * 255.999) as u32;
            buffer[y * args.width + x] = r << 16 | g << 8;
        }
    }

    buffer
}
