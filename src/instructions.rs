#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Instruction {
	SetEntireDisplayResume = 0xA4,
	SetEntireDisplayOn = 0xA5,
	SetDisplayNormal = 0xA6,
	SetDisplayInverse = 0xA7,
	SetDisplayOff = 0xAE,
	SetDisplayOn = 0xAF,
	ReadModifyWrite = 0xE0,
	End = 0xEE,
	Nop = 0xE3,
	SetBreathingLight = 0x23,
	AddHorizontalScrollSetup = 0x24,
	HorizontalScrollSetup = 0x26,
	SetScrollMode = 0x28,
	SetHorizontalScroll = 0x2E,
	SetContrastControl = 0x81,
	SetIrefResistor = 0x82,
	SetDcDcControl = 0xAD,
	SetSegPadsHardwareConfig = 0xA2,
	SetMultiplexRatio = 0xA8,
	SetDisplayOffset = 0xD3,
	SetDisplayClockDivide = 0xD5,
	SetPreChargePeriod = 0xD9,
	SetVcomDeselectLevel = 0xDB,
	SetRowNonOverlap = 0xDC,
	SetLowerColumnAddress = 0x00,
	SetHigherColumnAddress = 0x10,
	SetPumpVoltage = 0x30,
	SetDisplayStartLine = 0x40,
	SetPageAddress = 0xB0,
	SetSegmentRemap = 0xA0,
	SetCommonOutputScanDir = 0xC0,
	SetAdaptivePowerSave = 0xD6
}
impl From<Instruction> for u8 {
	fn from(instruction: Instruction) -> Self {
		instruction as u8
	}
}
