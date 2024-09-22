#![no_std]
#![no_main]

mod dos;
mod gui {
    pub mod input;
}

extern crate alloc;

use dos::{graphics::{Color, Screen}, keyboard::Scancode, SystemTime};
use embedded_graphics::{mono_font::{ascii::FONT_6X9, MonoTextStyle, MonoTextStyleBuilder}, prelude::{Dimensions, DrawTarget, Point, Primitive}, primitives::{Circle, PrimitiveStyle}, text::{Text, TextStyleBuilder}, Drawable};
use gui::input::TextInput;
// Mostly included for the allocator
use rust_dos::{entry, println, print};      

use alloc::{format, string::ToString};
use core::fmt::Write;

entry!(main);

struct Foo<D>
where 
    D: DrawTarget
{
    target: D,
}

fn main() {
    let mut screen = Screen::init();

    for y in 0..200 {   
        for x in 0..320 {
            let color = 0x00;

            dos::graphics::plot_pixel(x, y, color);
        }
    }

    let _ = screen.clear(Color::BACKGROUND_COLOR);

    let mut input = TextInput::new(Point::new(320/2 - 60, 50), 120);

    let character_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X9)
        .text_color(Color(0x0F))
        .background_color(Color(0x08))
        .build();

    // let _ = Text::new("Scancode", screen.bounding_box().center(), character_style).draw(&mut screen);


    input.set_selected(true);
    input.draw(&mut screen);

    loop {
        let code: Result<Scancode, ()> = dos::get_keyboard_input().try_into();


        if let Ok(Scancode::Escape) = code {
            break;
        }

        if let Ok(code) = code {
            // Text::new("Scancode", screen.bounding_box().center(), character_style).draw(&mut screen).unwrap();
            // let bounding_box = screen.bounding_box();
            // let _ = Circle::with_center(bounding_box.center(), 100)
            //     .into_styled(PrimitiveStyle::with_stroke(Color(0x01), 10))
            //     .draw(&mut screen);

            input.update(code);
            input.draw(&mut screen);
        }
    }
}

struct Clock;
