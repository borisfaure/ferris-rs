use hal::gpio::{gpiob::PB10, gpiob::PB11, Alternate, AF1};
use hal::i2c::I2c;
use hal::pac::I2C2;
use hal::prelude::*;
use hal::rcc::Rcc;
use hal::stm32;
use stm32f0xx_hal as hal;

pub type Pins = (PB10<Alternate<AF1>>, PB11<Alternate<AF1>>);
pub type I2c2 = I2c<stm32::I2C2, PB10<Alternate<AF1>>, PB11<Alternate<AF1>>>;
pub struct IoExpander {
    i2c: I2c2,
    pub is_ok: bool,
}

/// Helper enum to handle Rows/Cols in code
#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum GpioPin {
    Col5 = 0x0_u8,
    Col6 = 0x1_u8,
    Col7 = 0x2_u8,
    Col8 = 0x3_u8,
    Col9 = 0x4_u8,
    Row0 = 0x8_u8,
    Row1 = 0x9_u8,
    Row2 = 0xa_u8,
    Row3 = 0xb_u8,
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

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
enum Register {
    IODIR = 0x00, // i/o direction register
    GPPU = 0x0C,  // GPIO pull-up resistor register
    GPIOA = 0x12, // general purpose i/o port register (write modifies OLAT)
    GPIOB = 0x13, // general purpose i/o port register (write modifies OLAT)
    OLAT = 0x14,  // output latch register
}

const MCP_ADDR: u8 = 0x20;

impl IoExpander {
    fn init(&mut self) {
        // set pin direction
        // - input   : input  : 1
        // - driving : output : 0
        // This means: we will read all the bits on GPIOA
        // This means: we will write to the pins 0-4 on GPIOB (in select_rows)
        let data: [u8; 3] = [Register::IODIR as u8, 0b11111111, 0b11110000];
        let mut is_ok = self.i2c.write(MCP_ADDR, &data).is_ok();

        if !is_ok {
            /* Set Pull Up */
            let data: [u8; 3] = [Register::GPPU as u8, 0b11111111, 0b11110000];
            is_ok = self.i2c.write(MCP_ADDR, &data).is_ok();
        }
        self.is_ok = is_ok;
    }

    /// Create a new IoExpander and initialize the pins
    pub fn new(i2c: I2C2, pins: Pins, rcc: &mut Rcc) -> Self {
        let mut i2c = I2c::i2c2(i2c, pins, 100.khz(), rcc);
        let mut io_expander = Self { i2c, is_ok: true };
        io_expander.init();
        io_expander
    }

    /// I2C code to select one row
    ///
    /// Select the desired row only by writing a byte for the entire GPIOB bus
    /// where only the bit representing the row we want to select is
    /// a zero (write instruction) and every other bit is a one.
    fn select_row(&mut self, row: u8) {
        let row_selector: u8 = 0xff_u8 & !(1_u8 << row);
        let data: [u8; 2] = [Register::GPIOB as u8, row_selector];
        let mut is_ok = self.i2c.write(MCP_ADDR, &data).is_ok();
    }

    /// Get which keys are pressed on a specific row
    pub fn get_row(&mut self, row: u8) -> [bool; 5] {
        self.select_row(row);
        // TODO
        [false; 5]
    }
}
