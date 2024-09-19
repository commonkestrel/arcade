use core::arch::asm;

use embedded_graphics::prelude::{DrawTarget, OriginDimensions, PixelColor, Size};

pub struct Screen {
    _inner: (),
}

impl Screen {
    pub fn init() -> Screen {
        super::set_video_mode(0x13);
        return Screen { _inner: () };
    }
}

impl DrawTarget for Screen {
    type Color = Color;
    type Error = ();

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
        where
            I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>> {
        
        for pixel in pixels {
            // only plot this pixel if the x and y coordinates fit in a u16
            let (x, y): (u16, u16) = match (pixel.0.x.try_into(), pixel.0.y.try_into()) {
                (Ok(x), Ok(y)) => (x, y),
                _ => continue,
            };

            plot_pixel(x, y, pixel.1.0);
        }

        Ok(())
    }
}

impl OriginDimensions for Screen {
    fn size(&self) -> embedded_graphics::prelude::Size {
        return Size::new(320, 200);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color(pub u8);

impl PixelColor for Color {
    type Raw = ();
}

pub fn plot_pixel(x: u16, y: u16, color: u8) {
    if x < 320 && y < 200 {
        unsafe {
            asm!(
                "int 10h",
                in("ax") (0x0C00u16) | (color as u16),
                in("bh") 0u8,
                in("cx") x,
                in("dx") y,
            )
        }
    }
}