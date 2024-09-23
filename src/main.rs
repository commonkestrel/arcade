#![no_std]
#![no_main]

mod dos;
mod gui;

extern crate alloc;

use dos::{graphics::{Color, Screen}, keyboard::Scancode, SystemTime};
use embedded_graphics::{mono_font::{ascii::FONT_6X9, MonoTextStyle, MonoTextStyleBuilder}, prelude::{Dimensions, DrawTarget, Point, Primitive}, primitives::{Circle, PrimitiveStyle}, text::{Text, TextStyleBuilder}, Drawable};
use gui::{input::TextInput, Element};
// Mostly included for the allocator
use rust_dos::{entry, println, print};      

use alloc::{boxed::Box, format, string::ToString};
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

    let _ = screen.clear(Color::BACKGROUND_COLOR);

    let input = TextInput::new(Point::new(320/2 - 60, 50), 120);
    let input2 = TextInput::new(Point::new(320/2 - 60, 100), 100);

    let mut elements: [Box<dyn Element>; 2] = [
        Box::new(input),
        Box::new(input2),
    ];

    let mut selected = 0;
    elements[selected].set_selected(true);
    for element in elements.as_mut() {
        element.redraw(&mut screen);
    }
    

    loop {
        let code: Result<Scancode, ()> = dos::get_keyboard_input().try_into();

        if let Ok(code) = code {
            match code {
                Scancode::Escape => break,
                Scancode::Tab => {
                    let previous = selected;
                    selected = (selected + 1).min(elements.len() - 1);
                    if previous != selected {
                        elements[previous].set_selected(false);
                        elements[previous].draw(&mut screen);

                        elements[selected].set_selected(true);
                        elements[selected].draw(&mut screen);
                    }
                }
                Scancode::Tilde => {
                    let previous = selected;
                    selected = selected.checked_sub(1).unwrap_or(0);
                    if previous != selected {
                        elements[previous].set_selected(false);
                        elements[previous].draw(&mut screen);

                        elements[selected].set_selected(true);
                        elements[selected].draw(&mut screen);
                    }
                }
                sc => {
                    // we only have to update&redraw the selected element
                    elements[selected].update(sc);
                    elements[selected].draw(&mut screen);
                }
            }
        }
    }
}

struct Clock;
