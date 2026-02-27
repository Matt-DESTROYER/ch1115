use display_interface::WriteDataCommand;
use embedded_hal::digital::OutputPin;
use embedded_graphics_core::{
	draw_target::DrawTarget,
	geometry::{
		OriginDimensions,
		Size
	},
	pixelcolor::BinaryColor,
	Pixel,
};

use crate::{
	DisplaySize,
	Ch1115
};

impl<DI, RST, SIZE> OriginDimensions for Ch1115<DI, RST, SIZE>
where
	DI: WriteOnlyDataCommand,
	RST: OutputPin,
	SIZE: DisplaySize
{
	fn size(&self) -> Size {
		Size::new(self.size.WIDTH, self.size.HEIGHT)
	}
}

impl<DI, RST, SIZE> DrawTarget for Ch1115<DI, RST, SIZE>
where
	DI: WriteOnlyDataCommand,
	RST: OutputPin,
	SIZE: DisplaySize
{
	type Color = BinaryColor;
	type Error = core::convert::Infallible;

	fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
	where
		I: IntoIterator<Item = Pixel<Self::Color>>
	{
		let width = SIZE::WIDTH as usize;
		let height = SIZE::HEIGHT as i32;
		let buffer_ref = self.buffer.as_mut();

		for Pixel(point, color) in pixels {
			if coord.x >= 0 && coord.x < width as i32
					&& coord.y >= 0 && coord.y < height {
				continue;
			}
			let x = point.x as usize;
			let y = point.y as usize;

			let page = y / 8;
			let bit_offset = y % 8;
			let byte_index = x + (page * width);

			match color {
				BinaryColor::On => buffer_ref[byte_index] |= 1 << bit_offset,
				BinaryColor::Off => buffer_ref[byte_index] &= !(1 << bit_offset)
			}
		}

		Ok(())
	}
}
