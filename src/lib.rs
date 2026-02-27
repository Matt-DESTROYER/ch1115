#![no_std]

pub mod instructions;
use crate::instructions::Instruction;

use display_interface::{
	DataFormat::U8,
	WriteOnlyDataCommand
};
use embedded_hal::{
	delay::DelayNs,
	digital::{
		ErrorType,
		OutputPin
	}
};

use core::convert::Infallible;

#[cfg(feature = "async")]
use embedded_hal_async::delay::DelayNs as AsyncDelayNs;

#[cfg(feature = "graphics")]
mod graphics;

pub struct NoResetPin;
impl ErrorType for NoResetPin {
	type Error = Infallible;
}
impl OutputPin for NoResetPin {
	fn set_low(&mut self) -> core::result::Result<(), Self::Error> {
		Ok(())
	}

	fn set_high(&mut self) -> core::result::Result<(), Self::Error> {
		Ok(())
	}
}

#[derive(Debug)]
pub enum Error<PinError> {
	DisplayError,
	Pin(PinError)
}

pub type Result<T = (), PinError = Infallible> = core::result::Result<T, Error<PinError>>;

pub trait DisplaySize {
	const WIDTH: u8;
	const HEIGHT: u8;
	const OFFSET_X: u8;
	const OFFSET_Y: u8;

	type Buffer: AsMut<[u8]> + AsRef<[u8]>;

	fn create_buffer() -> Self::Buffer;
}

pub struct Size128x64;
impl DisplaySize for Size128x64 {
	const WIDTH: u8 = 128;
	const HEIGHT: u8 = 64;
	const OFFSET_X: u8 = 0;
	const OFFSET_Y: u8 = 0;

	type Buffer = [u8; 1024];
	fn create_buffer() -> Self::Buffer{ [0; 1024] }
}

pub struct Size88x48;
impl DisplaySize for Size88x48 {
	const WIDTH: u8 = 88;
	const HEIGHT: u8 = 48;
	const OFFSET_X: u8 = 20;
	const OFFSET_Y: u8 = 0;

	type Buffer = [u8; 528];
	fn create_buffer() -> Self::Buffer{ [0; 528] }
}

#[allow(dead_code, non_snake_case)]
pub mod BreathingLight {
	const ON: bool = true;
	const OFF: bool = false;

	#[repr(u8)]
	#[derive(Copy, Clone, Debug, PartialEq, Eq)]
	pub enum MaxBrightness {
		_256 = 0b00,
		_128 = 0b01,
		_64 = 0b10,
		_32 = 0b11
	}
}

#[allow(dead_code)]
pub struct Ch1115<DI, RST, SIZE: DisplaySize> {
	di: DI,
	rst: RST,
	size: SIZE,
	buffer: SIZE::Buffer
}

impl<DI, RST, SIZE, PinError> Ch1115<DI, RST, SIZE>
where
	DI: WriteOnlyDataCommand,
	RST: OutputPin<Error = PinError>,
	SIZE: DisplaySize
{
	pub fn new(interface: DI, reset: RST, size: SIZE) -> Self {
		Self {
			di: interface,
			rst: reset,
			size,
			buffer: SIZE::create_buffer()
		}
	}

	pub fn init(&mut self, delay_source: &mut impl DelayNs) -> Result<(), PinError> {
		self.hard_reset(delay_source)?;

		self.write_command(Instruction::SetDisplayOff)?;
		self.write_command(Instruction::SetDisplayClockDivide)?;
		self.write_command(0x80)?;

		self.write_command(Instruction::SetMultiplexRatio)?;
		self.write_command(SIZE::HEIGHT - 1)?;

		self.write_command(Instruction::SetDisplayOffset)?;
		self.write_command(0x00)?; // No offset

		self.write_command(Instruction::SetDisplayStartLine as u8 | 0x00)?;

		// Internal charge pump configuration
		self.write_command(Instruction::SetDcDcControl)?;
		self.write_command(0x8B)?; // Built-in DC-DC on

		self.write_command(Instruction::SetPumpVoltage as u8 | 0x02)?; // 7.4V to 8.0V

		self.write_command(Instruction::SetSegmentRemap as u8 | 0x01)?; // Reverse
		self.write_command(Instruction::SetCommonOutputScanDir as u8 | 0x08)?; // Reverse

		self.write_command(Instruction::SetSegPadsHardwareConfig)?;

		self.write_command(Instruction::SetContrastControl)?;
		self.write_command(0x80)?; // Mid contrast

		self.write_command(Instruction::SetPreChargePeriod)?;
		self.write_command(0x22)?; // Default

		self.write_command(Instruction::SetVcomDeselectLevel)?;
		self.write_command(0x35)?; // Default

		self.clear()?;

		self.write_command(Instruction::SetEntireDisplayResume)?; // Resume from RAM
		self.write_command(Instruction::SetDisplayNormal)?; // Normal, not inverted
		self.write_command(Instruction::SetDisplayOn)?;

		Ok(())
	}

	#[cfg(feature = "async")]
	pub async fn init_async(&mut self, delay_source: &mut impl AsyncDelayNs) -> Result<(), PinError> {
		self.hard_reset_async(delay_source).await?;

		self.write_command(Instruction::SetDisplayOff)?;
		self.write_command(Instruction::SetDisplayClockDivide)?;
		self.write_command(0x80)?;

		self.write_command(Instruction::SetMultiplexRatio)?;
		self.write_command(SIZE::HEIGHT - 1)?;

		self.write_command(Instruction::SetDisplayOffset)?;
		self.write_command(0x00)?; // No offset

		self.write_command(Instruction::SetDisplayStartLine as u8 | 0x00)?;

		// Internal charge pump configuration
		self.write_command(Instruction::SetDcDcControl)?;
		self.write_command(0x8B)?; // Built-in DC-DC on

		self.write_command(Instruction::SetPumpVoltage as u8 | 0x02)?; // 7.4V to 8.0V

		self.write_command(Instruction::SetSegmentRemap as u8 | 0x01)?; // Reverse
		self.write_command(Instruction::SetCommonOutputScanDir as u8 | 0x08)?; // Reverse

		self.write_command(Instruction::SetSegPadsHardwareConfig)?;

		self.write_command(Instruction::SetContrastControl)?;
		self.write_command(0x80)?; // Mid contrast

		self.write_command(Instruction::SetPreChargePeriod)?;
		self.write_command(0x22)?; // Default

		self.write_command(Instruction::SetVcomDeselectLevel)?;
		self.write_command(0x35)?; // Default

		self.clear()?;

		self.write_command(Instruction::SetEntireDisplayResume)?; // Resume from RAM
		self.write_command(Instruction::SetDisplayNormal)?; // Normal, not inverted
		self.write_command(Instruction::SetDisplayOn)?;

		Ok(())
	}

	pub fn hard_reset(&mut self, delay_source: &mut impl DelayNs) -> Result<(), PinError> {
		self.rst.set_high()
			.map_err(Error::Pin)?;
		delay_source.delay_ms(1);

		self.rst.set_low()
			.map_err(Error::Pin)?;
		// > 10 us
		delay_source.delay_us(50);

		self.rst.set_high()
			.map_err(Error::Pin)?;
		// > 2 us
		delay_source.delay_ms(1);

		Ok(())
	}

	#[cfg(feature = "async")]
	pub async fn hard_reset_async(&mut self, delay_source: &mut impl AsyncDelayNs) -> Result<(), PinError> {
		self.rst.set_high()
			.map_err(Error::Pin)?;
		delay_source.delay_ms(1).await;

		self.rst.set_low()
			.map_err(Error::Pin)?;
		// > 10 us
		delay_source.delay_us(50).await;

		self.rst.set_high()
			.map_err(Error::Pin)?;
		// > 2 us
		delay_source.delay_ms(1).await;

		Ok(())
	}

	pub fn flush(&mut self) -> Result<(), PinError> {
		let pages = SIZE::HEIGHT / 8;
		let width = SIZE::WIDTH as usize;

		for page in 0..pages {
			self.set_page(page)?;
			self.set_column_address(SIZE::OFFSET_X)?;

			let start = (page as usize) * width;
			let end = start + width;

			self.di
				.send_data(U8(&self.buffer.as_ref()[start..end]))
				.map_err(|_| Error::DisplayError)?;
		}

		Ok(())
	}

	pub fn set_column_address(&mut self, column: u8) -> Result<(), PinError> {
		if column > 127 {
			return  Ok(());
		}

		let lower_nibble = column & 0x0F;
		let higher_nibble = (column >> 4) & 0x0F;

		self.write_command(Instruction::SetLowerColumnAddress as u8 | lower_nibble)?;
		self.write_command(Instruction::SetHigherColumnAddress as u8 | higher_nibble)?;

		Ok(())
	}

	pub fn set_breathing_effect(&mut self, on: bool, max_brightness: BreathingLight::MaxBrightness, frames: u8) -> Result<(), PinError> {
		if frames == 0 || frames > 8 {
			return Ok(());
		}

		self.write_command(Instruction::SetBreathingLight)?;

		let mut instruction = 0;
		if on {
			instruction |= 1 << 7;
		}
		instruction |= (max_brightness as u8) << 3;
		instruction |= frames - 1;

		self.write_command(instruction)?;

		Ok(())
	}

	pub fn set_page(&mut self, page: u8) -> Result<(), PinError> {
		if page > 7 {
			return Ok(());
		}
		
		self.write_command(Instruction::SetPageAddress as u8 | page)?;
		Ok(())
	}

	pub fn clear(&mut self) -> Result<(), PinError> {
		self.buffer.as_mut().fill(0);
		self.flush()?;

		Ok(())
	}

	fn write_command(&mut self, command: impl Into<u8>) -> Result<(), PinError> {
		self.di
			.send_commands(U8(&[command.into()]))
			.map_err(|_| Error::DisplayError)?;
		Ok(())
	}

	/*fn write_data(&mut self, data: &[u8]) -> Result<(), PinError> {
		self.di
			.send_data(U8(data))
			.map_err(|_| Error::DisplayError)?;
		Ok(())
	}*/
}
