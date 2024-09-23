use embedded_graphics::prelude::DrawTarget;

use crate::dos::{graphics::{Color, Screen}, keyboard::Scancode};

pub mod input;
pub mod date;

pub trait Element {
    fn update(&mut self, sc: Scancode);
    fn set_selected(&mut self, selected: bool);
    /// Draw updates to the contents of the element
    fn draw(&mut self, target: &mut Screen);
    /// Redraw the entire element
    fn redraw(&mut self, target: &mut Screen);
}
