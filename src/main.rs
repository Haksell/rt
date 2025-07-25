#![allow(unused)] // TODO: remove before push

mod canvas;
mod color;
mod floats;
mod tuple;

use {
    crate::canvas::Canvas,
    minifb::{Key, Window, WindowOptions},
};

fn main() {
    let canvas = Canvas::new(600, 400);
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
