#![no_std]
#![no_main]

mod dos;

use dos::graphics::{Color, Screen};
use embedded_graphics::{prelude::{Dimensions, Primitive}, primitives::{Circle, PrimitiveStyle}, Drawable};
// Mostly included for the allocator
use rust_dos::{entry, println, print};

entry!(main);

fn main() {
    println!("Hello world!");

    let mut screen = Screen::init();

    for y in 0..200 {
        for x in 0..320 {
            let color = 0x50;

            dos::graphics::plot_pixel(x, y, color);
        }
    }

    let bounding_box = screen.bounding_box();
    let _ = Circle::with_center(bounding_box.center(), 100)
        .into_styled(PrimitiveStyle::with_stroke(Color(0x01), 10))
        .draw(&mut screen);

    loop {
        let code = dos::get_keyboard_input();
        if code != 0 { break; }
    }
}
