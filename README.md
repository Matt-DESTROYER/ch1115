# ch1115

A `no_std` Rust driver for the CH1115 monochrome OLED/PLED controller.

This crate provides an interface to CH1115-based displays, built on top of `embedded-hal` 1.0.0 and `display-interface`. It features a highly optimized, generic `DisplaySize` architecture that perfectly sizes the internal zero-cost framebuffer to your specific physical display (e.g., 128x64, 88x48) to save precious SRAM.

## Features
 - `no_std`
 - `embedded-hal`
 - Async support through the `async` feature
 - `embedded-graphics` support through the `graphics` feature

## Examples

### Quickstart
```rust
use ch1115::{Ch1115, Size128x64, NoResetPin};
use embedded_hal::delay::DelayNs;

// Assuming you have your display interface (SPI/I2C) and Delay initialized from your HAL
// let di = ...; // e.g., display_interface_spi::SPIInterface
// let rst = ...; // e.g., esp_hal::gpio::Output (or ch1115::NoResetPin if none)
// let mut delay = ...;

// Initialize the display with the perfectly sized buffer for 128x64
let mut display = Ch1115::new(di, rst, Size128x64);

// Wake up the hardware and initialize the charge pump
display.init(&mut delay).unwrap();

// Clear the internal buffer and blast it to the screen (turns all pixels off)
display.clear().unwrap();

display.flush().unwrap();
```

### Drawing with embedded-graphics
Enable the `graphics` feature in your Cargo.toml.

```rs
use ch1115::{Ch1115, Size128x64};
use embedded_graphics::{
    pixelcolor::BinaryColor,
    primitives::{Circle, PrimitiveStyle},
    prelude::*
};

let mut display = Ch1115::new(di, rst, Size128x64);
display.init(&mut delay).unwrap();
display.clear().unwrap();

// draw a circle to the internal buffer
Circle::new(Point::new(64, 32), 20)
    .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 2))
    .draw(&mut display)
    .unwrap();

// send the updated buffer to the physical screen
display.flush().unwrap();
```