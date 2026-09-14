use display_interface::WriteOnlyDataCommand;
use embedded_graphics_core::{
    Pixel, draw_target::DrawTarget, geometry::{OriginDimensions, Point, Size}, pixelcolor::BinaryColor, primitives::Rectangle,
};
use embedded_hal::digital::OutputPin;

use crate::{Ch1115, DisplaySize};

impl<DI, RST, SIZE> OriginDimensions for Ch1115<DI, RST, SIZE>
where
    DI: WriteOnlyDataCommand,
    RST: OutputPin,
    SIZE: DisplaySize,
{
    fn size(&self) -> Size {
        Size::new(SIZE::WIDTH.into(), SIZE::HEIGHT.into())
    }
}

impl<DI, RST, SIZE> DrawTarget for Ch1115<DI, RST, SIZE>
where
    DI: WriteOnlyDataCommand,
    RST: OutputPin,
    SIZE: DisplaySize,
{
    type Color = BinaryColor;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let width: usize = SIZE::WIDTH.into();
        let visible = Rectangle::new(Point::zero(), self.size());
        let buffer_ref = self.buffer.as_mut();

        for Pixel(point, color) in pixels {
            if !visible.contains(point) {
                continue;
            }

            let x = point.x as usize;
            let y = point.y as usize;

            let page = y / 8;
            let bit_offset = y % 8;
            let byte_index = x + (page * width);

            match color {
                BinaryColor::On => buffer_ref[byte_index] |= 1 << bit_offset,
                BinaryColor::Off => buffer_ref[byte_index] &= !(1 << bit_offset),
            }
        }

        Ok(())
    }
}
