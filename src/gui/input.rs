use alloc::string::String;
use embedded_graphics::{mono_font::{ascii::FONT_6X12, MonoTextStyle}, prelude::*, primitives::{Line, PrimitiveStyleBuilder, Rectangle}, text::Text};

use crate::dos::{graphics::Color, keyboard::Scancode, SystemTime};

pub struct TextInput {
    content: String,
    width: u32,
    cursor: usize,
    selected: bool,
    position: Point,
}

impl TextInput {
    const TEXT_PIXEL_WIDTH: usize = 6;

    pub fn new(top_left: Point, width: u32) -> TextInput {
        Self {
            content: String::new(),
            width,
            cursor: 0,
            selected: false,
            position: top_left,
        }
    }

    pub fn update(&mut self, sc: Scancode) {
        if self.content_full() {
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
                // self.insert_content('<');
                self.cursor = self.cursor.checked_sub(1).unwrap_or(0)
            },
            Scancode::Backspace => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    self.content.remove(self.cursor);
                }
            }
            _ => {}
        }
    }

    pub fn insert_content(&mut self, content: char) {
        self.content.insert(self.cursor, content);
        self.cursor += 1;
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    pub fn draw(&self, target: &mut impl DrawTarget<Color = Color>) {
        let blink = SystemTime::now().second() % 2 == 0;

        let style = PrimitiveStyleBuilder::new()
            .stroke_color(Color(0x00))
            .stroke_width(1)
            .fill_color(Color(0x0F))
            .build();

        let _ = Rectangle::new(self.position, Size::new(self.width + 4, 13))
            .into_styled(style)
            .draw(target);

        let style = MonoTextStyle::new(&FONT_6X12, Color(0x00));
        let mut text_position = self.position;
        text_position.x += 2;
        text_position.y += 9;

        let _ = Text::new(&self.content, text_position, style).draw(target);

        // if blink {
        //     let start = Point::new(self.position.x + 2 + ((self.cursor*Self::TEXT_PIXEL_WIDTH) as i32), self.position.y + 10);
        //     let end = Point::new(start.x + (Self::TEXT_PIXEL_WIDTH as i32), start.y);
        //     let style = PrimitiveStyleBuilder::new()
        //         .stroke_color(Color(0x00))
        //         .build();

        //     let _ = Line::new(start, end)
        //         .into_styled(style)
        //         .draw(target);
        // }
    }

    fn content_full(&self) -> bool {
        return (self.content.len() as u32) >= self.width / (Self::TEXT_PIXEL_WIDTH as u32);
    }
}
