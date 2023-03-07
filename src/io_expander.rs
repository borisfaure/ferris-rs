use hal::gpio::{gpiob::PB10, gpiob::PB11, Alternate, AF1};
use hal::i2c::I2c;
use hal::pac::I2C1;
use hal::prelude::*;
use hal::rcc::Rcc;
use hal::stm32;
use mcp230xx::{Direction, Level, Mcp23017, Mcp230xx};
use stm32f0xx_hal as hal;

pub type Pins = (PB10<Alternate<AF1>>, PB11<Alternate<AF1>>);
pub type I2c1 = I2c<stm32::I2C1, PB10<Alternate<AF1>>, PB11<Alternate<AF1>>>;
pub struct IoExpander {
    mcp: Mcp230xx<I2c1, Mcp23017>,
    pub is_ok: bool,
}

/// Helper enum to handle Rows/Cols in code
#[derive(Debug, Copy, Clone)]
pub enum GpioPin {
    Row0,
    Row1,
    Row2,
    Row3,
    Col5,
    Col6,
    Col7,
    Col8,
    Col9,
}

/// Helper to convert GpioPin into pins on MCP23017
impl From<GpioPin> for Mcp23017 {
    fn from(x: GpioPin) -> Self {
        match x {
            GpioPin::Row0 => Self::B0,
            GpioPin::Row1 => Self::B1,
            GpioPin::Row2 => Self::B2,
            GpioPin::Row3 => Self::B3,
            GpioPin::Col5 => Self::A0,
            GpioPin::Col6 => Self::A1,
            GpioPin::Col7 => Self::A2,
            GpioPin::Col8 => Self::A3,
            GpioPin::Col9 => Self::A4,
        }
    }
}

/// Helper to get the Row pins
fn rows() -> [GpioPin; 4] {
    [GpioPin::Row0, GpioPin::Row1, GpioPin::Row2, GpioPin::Row3]
}

/// Helper to get the Column pins
fn cols() -> [GpioPin; 5] {
    [
        GpioPin::Col5,
        GpioPin::Col6,
        GpioPin::Col7,
        GpioPin::Col8,
        GpioPin::Col9,
    ]
}

impl IoExpander {
    /// Create a new IoExpander and initialize the pins
    pub fn new(i2c: I2C1, pins: Pins, rcc: &mut Rcc) -> Self {
        let mut i2c = I2c::i2c1(i2c, pins, 100.khz(), rcc);
        let mut data = [0x0u8, 0xffu8, 0xf0u8];
        let addr = 0x20;
        // this panics or makes the mcu panic
        //let is_ok = i2c.write(addr, &data).is_ok();
        let is_ok = false;
        let mcp = Mcp230xx::new_default(i2c).unwrap();
        let io_expander = Self { mcp, is_ok };
        /*
        for pin in rows() {
            io_expander
                .set_direction(pin.into(), Direction::Output)
                .unwrap();
            io_expander
                .set_gpio(pin.into(), mcp230xx::Level::Low)
                .unwrap();
        }
                */
        /*
        for pin in cols() {
            io_expander
                .set_direction(pin.into(), mcp230xx::Direction::Input)
                .unwrap();
            io_expander
                .set_gpio(pin.into(), mcp230xx::Level::High)
                .unwrap();
        }
        */
        io_expander
    }
}
