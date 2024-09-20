#![no_std]
#![no_main]

mod dos;

use dos::graphics::{Color, Screen};
use embedded_graphics::{prelude::{Dimensions, DrawTarget, Primitive}, primitives::{Circle, PrimitiveStyle}, Drawable};
use embedded_gui::{self as egui, widgets::{background::Background, label::Label, layouts::linear::Column}};
use backend_embedded_graphics::{self as egui_backend, themes::Theme, widgets::label::ascii::LabelConstructor, EgCanvas};
use egui::Window;
// Mostly included for the allocator
use rust_dos::{entry, println, print};

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

    // let bounding_box = screen.bounding_box();
    // let _ = Circle::with_center(bounding_box.center(), 100)
    //     .into_styled(PrimitiveStyle::with_stroke(Color(0x01), 10))
    //     .draw(&mut screen);

    let mut gui = Window::new(
        egui_backend::EgCanvas::new(screen),
        Column::new()
            .add(Label::new("Hello, world!"))
    );

    gui
        .canvas
        .target
        .clear(Color::BACKGROUND_COLOR)
        .unwrap();

    gui.update();
    gui.measure();
    gui.arrange();
    gui.draw().unwrap();

    loop {
        let code = dos::get_keyboard_input();
        if code != 0 { break; }
    }
}
