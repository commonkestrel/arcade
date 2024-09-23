use alloc::string::String;
use embedded_graphics::{mono_font::{ascii::{FONT_6X10, FONT_6X12}, MonoTextStyle, MonoTextStyleBuilder}, prelude::*, primitives::{Line, PrimitiveStyleBuilder, Rectangle}, text::{Text, TextStyleBuilder}};

use crate::dos::{graphics::{Color, Screen}, keyboard::Scancode, SystemTime};

use super::Element;

pub struct TextInput {
    content: String,
    width: usize,
    cursor: usize,
    selected: bool,
    position: Point,
    need_clear: bool,
}

impl TextInput {
    const TEXT_PIXEL_WIDTH: usize = 6;

    pub fn new(top_left: Point, width: usize) -> TextInput {
        Self {
            content: String::with_capacity(width / Self::TEXT_PIXEL_WIDTH),
            width,
            cursor: 0,
            selected: false,
            position: top_left,
            need_clear: false,
        }
    }

    fn insert_content(&mut self, content: char) {
        self.content.insert(self.cursor, content);
        self.cursor += 1;
    }


    fn content_full(&self) -> bool {
        return self.content.len() >= self.width / Self::TEXT_PIXEL_WIDTH;
    }
}

impl Element for TextInput {
    fn update(&mut self, sc: Scancode) {
        if self.content_full() && sc != Scancode::Backspace && sc != Scancode::Left && sc != Scancode::Right {
            return;
        }

        match sc {
            Scancode::A => self.insert_content('a'),
            Scancode::B => self.insert_content('b'),
            Scancode::C => self.insert_content('c'),
            Scancode::D => self.insert_content('d'),
            Scancode::E => self.insert_content('e'),
            Scancode::F => self.insert_content('f'),
            Scancode::G => self.insert_content('g'),
            Scancode::H => self.insert_content('h'),
            Scancode::I => self.insert_content('i'),
            Scancode::J => self.insert_content('j'),
            Scancode::K => self.insert_content('k'),
            Scancode::L => self.insert_content('l'),
            Scancode::M => self.insert_content('m'),
            Scancode::N => self.insert_content('n'),
            Scancode::O => self.insert_content('o'),
            Scancode::P => self.insert_content('p'),
            Scancode::Q => self.insert_content('q'),
            Scancode::R => self.insert_content('r'),
            Scancode::S => self.insert_content('s'),
            Scancode::T => self.insert_content('t'),
            Scancode::U => self.insert_content('u'),
            Scancode::V => self.insert_content('v'),
            Scancode::W => self.insert_content('w'),
            Scancode::X => self.insert_content('x'),
            Scancode::Y => self.insert_content('y'),
            Scancode::Z => self.insert_content('z'),
            Scancode::One => self.insert_content('1'),
            Scancode::Two => self.insert_content('2'),
            Scancode::Three => self.insert_content('3'),
            Scancode::Four => self.insert_content('4'),
            Scancode::Five => self.insert_content('5'),
            Scancode::Six => self.insert_content('6'),
            Scancode::Seven => self.insert_content('7'),
            Scancode::Eight => self.insert_content('8'),
            Scancode::Nine => self.insert_content('9'),
            Scancode::Zero => self.insert_content('0'),
            Scancode::Space => self.insert_content(' '),
            Scancode::Comma => self.insert_content(','),
            Scancode::Right => {
                // self.insert_content('>');
                self.cursor = (self.cursor + 1).min(self.content.len())
            },
            Scancode::Left => {
                self.need_clear = true;
                self.cursor = self.cursor.checked_sub(1).unwrap_or(0)
            },
            Scancode::Backspace => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    self.content.remove(self.cursor);
                    self.need_clear = true;
                }
            }
            _ => {}
        }
    }


    fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
        self.need_clear = true;
    }

    fn draw(&mut self, target: &mut Screen) {
        let character_style = MonoTextStyleBuilder::new()
            .text_color(Color(0x00))
            .background_color(Color(0x0F))
            .font(&FONT_6X10)
            .build();

        let text_position = Point::new(self.position.x + 2, self.position.y + 9);

        let next = Text::new(&self.content, text_position, character_style).draw(target).unwrap();

        if self.need_clear {
            self.need_clear = false;

            let start = Point::new(next.x, self.position.y + 1);

            let remaining_characters = self.width / Self::TEXT_PIXEL_WIDTH - self.content.len();
            let size = Size::new((remaining_characters*Self::TEXT_PIXEL_WIDTH) as u32, 11 as u32);

            let style = PrimitiveStyleBuilder::new()
                .fill_color(Color(0x0F))
                .build();

            let _ = Rectangle::new(start, size).into_styled(style).draw(target);
        }

        if self.selected && !self.content_full() {
            let start = Point::new(self.position.x + 2 + ((self.cursor*Self::TEXT_PIXEL_WIDTH) as i32), self.position.y + 10);
            let end = Point::new(start.x + (Self::TEXT_PIXEL_WIDTH as i32), start.y);
            let style = PrimitiveStyleBuilder::new()
                .stroke_color(Color(0x00))
                .stroke_width(1)
                .build();

            let _ = Line::new(start, end)
                .into_styled(style)
                .draw(target);
        }
    }

    fn redraw(&mut self, target: &mut Screen) {
        let style = PrimitiveStyleBuilder::new()
            .stroke_color(Color(0x00))
            .stroke_width(1)
            .fill_color(Color(0x0F))
            .build();

        let _ = Rectangle::new(self.position, Size::new((self.width + 4) as u32, 13))
            .into_styled(style)
            .draw(target);

        let style = MonoTextStyle::new(&FONT_6X10, Color(0x00));
        let mut text_position = self.position;
        text_position.x += 2;
        text_position.y += 9;

        let _ = Text::new(&self.content, text_position, style).draw(target);

        if self.selected && !self.content_full() {
            let start = Point::new(self.position.x + 2 + ((self.cursor*Self::TEXT_PIXEL_WIDTH) as i32), self.position.y + 10);
            let end = Point::new(start.x + (Self::TEXT_PIXEL_WIDTH as i32), start.y);
            let style = PrimitiveStyleBuilder::new()
                .stroke_color(Color(0x00))
                .stroke_width(1)
                .build();

            let _ = Line::new(start, end)
                .into_styled(style)
                .draw(target);
        }
    }

}
