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
}

/**
 * Rows are on GPIOB: PB0 to PB3
 * Cols are on GPIOA: PA0 to PA4
 */

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
    fn reset(&mut self) {
        // set pin direction
        // - input   : input  : 1
        // - driving : output : 0
        // This means: we will read all the bits on GPIOA
        // This means: we will write to the pins 0-4 on GPIOB (in select_rows)
        let data: [u8; 3] = [Register::IODIR as u8, 0b11111111, 0b11110000];

        if self.i2c.write(MCP_ADDR, &data).is_err() {
            /* Set Pull Up */
            let data: [u8; 3] = [Register::GPPU as u8, 0b11111111, 0b11110000];
            self.i2c.write(MCP_ADDR, &data).unwrap();
        }
    }

    /// Create a new IoExpander and initialize the pins
    pub fn new(i2c: I2C2, pins: Pins, rcc: &mut Rcc) -> Self {
        let i2c = I2c::i2c2(i2c, pins, 100.khz(), rcc);
        let mut io_expander = Self { i2c };
        io_expander.reset();
        io_expander
    }

    /// I2C code to select one row
    ///
    /// Select the desired row only by writing a byte for the entire GPIOB bus
    /// where only the bit representing the row we want to select is
    /// a zero (write instruction) and every other bit is a one.
    fn select_row(&mut self, row: u8) {
        let row_selector: u8 = !(1_u8 << row);
        let data: [u8; 2] = [Register::GPIOB as u8, row_selector];
        if self.i2c.write(MCP_ADDR, &data).is_err() {
            self.reset();
            self.i2c.write(MCP_ADDR, &data).unwrap();
        }
    }

    /// Get which keys are pressed on a specific row
    pub fn get_row(&mut self, row: u8) -> [bool; 5] {
        self.select_row(row);

        // Read all the pins on GPIOA
        let mut data: [u8; 1] = [0];
        if self.i2c.write(MCP_ADDR, &[Register::GPIOA as u8]).is_err() {
            self.reset();
            self.select_row(row);
            self.i2c.write(MCP_ADDR, &[Register::GPIOA as u8]).unwrap();
        }
        let mut cols = [false; 5];
        if self.i2c.read(MCP_ADDR, &mut data).is_ok() {
            // The return value is a row as represented in the generic matrix
            // code were the rightmost bits represent the lower columns (on
            // the left) and zeroes represent pressed keys while ones
            // represent depressed keys.
            for (i, col) in cols.iter_mut().enumerate() {
                if (data[0] & (1_u8 << i)) == 0_u8 {
                    *col = true;
                }
            }
        }
        cols
    }
}
