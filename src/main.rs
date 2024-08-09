use minifb::{Key, Window, WindowOptions};
use rt::{Canvas, Color};

// TODO: args for window or PPM file or just keyboard shortcut?

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let color = Color::new(
                x as f32 / (WIDTH - 1) as f32,
                y as f32 / (HEIGHT - 1) as f32,
                0.5,
            );
            canvas.set_pixel(x, y, color);
        }
    }

    let mut window = Window::new("rt", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| panic!("{}", e));
    window.set_target_fps(6); // TODO: remove once incremental raytracing is implemented

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for (i, pixel) in buffer.iter_mut().enumerate() {
            *pixel = canvas.get_pixel(i % WIDTH, i / WIDTH).to_u32();
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
