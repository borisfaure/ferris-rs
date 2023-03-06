use enum_iterator::Sequence;
use hal::gpio::gpiob;
use hal::gpio::{gpiob::PB10, gpiob::PB11, Alternate, Input, Output, Pin, PullUp, PushPull, AF1};
use hal::i2c::{I2c, SclPin, SdaPin};
use hal::pac::I2C1;
use hal::prelude::*;
use hal::rcc::Rcc;
use hal::stm32;
use mcp230xx;
use stm32f0xx_hal as hal;

pub type Pins = (PB10<Alternate<AF1>>, PB11<Alternate<AF1>>);
pub type I2c1 = I2c<stm32::I2C1, PB10<Alternate<AF1>>, PB11<Alternate<AF1>>>;
pub struct IoExpander(mcp230xx::Mcp230xx<I2c1, mcp230xx::Mcp23017>);

#[derive(Debug, Copy, Clone, Sequence)]
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

impl From<GpioPin> for mcp230xx::Mcp23017 {
    fn from(x: GpioPin) -> Self {
        match x {
            GpioPin::Row0 => Self::B0,
            GpioPin::Row1 => Self::B0,
            GpioPin::Row2 => Self::B0,
            GpioPin::Row3 => Self::B0,
            GpioPin::Col5 => Self::A0,
            GpioPin::Col6 => Self::A1,
            GpioPin::Col7 => Self::A2,
            GpioPin::Col8 => Self::A3,
            GpioPin::Col9 => Self::A4,
        }
    }
}

impl IoExpander {
    pub fn new(i2c: I2C1, pins: Pins, rcc: &mut Rcc) -> Self {
        let mut i2c = I2c::i2c1(i2c, pins, 100.khz(), rcc);
        let io_expander = mcp230xx::Mcp230xx::new_default(i2c).unwrap();
        Self(io_expander)
    }
}
