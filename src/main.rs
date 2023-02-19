#![no_std]
#![no_main]

// Some panic handler needs to be included. This one halts the processor on panic.
extern crate panic_halt;

use cortex_m_rt::entry;

use stm32_usbd::UsbBus;
use stm32f0xx_hal::{pac, prelude::*, usb};
use usb_device::prelude::*;

use usbd_human_interface_device::device::keyboard::NKROBootKeyboardInterface;
use usbd_human_interface_device::page::Keyboard;
use usbd_human_interface_device::prelude::UsbHidClassBuilder;

#[entry]
fn main() -> ! {
    let mut dp = pac::Peripherals::take().unwrap();

    // Reset and clock control
    let mut rcc = dp
        .RCC
        .configure()
        .hsi48()
        .enable_crs(dp.CRS) // Clock Recovery System
        .sysclk(48.mhz())
        .pclk(24.mhz())
        .freeze(&mut dp.FLASH);

    let gpioa = dp.GPIOA.split(&mut rcc);

    let usb_bus = UsbBus::new(usb::Peripheral {
        usb: dp.USB,
        pin_dm: gpioa.pa11,
        pin_dp: gpioa.pa12,
    });

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x1209, 0x0001))
        .manufacturer("usbd-human-interface-device")
        .product("NKRO Keyboard")
        .serial_number("TEST")
        .build();

    //let mut keyboard = UsbHidClassBuilder::new()
    //    .add_interface(NKROBootKeyboardInterface::default_config())
    //    .build(&usb_dev);

    //let timer = Timer::new(pac.TIMER, &mut pac.RESETS);
    //let mut input_count_down = timer.count_down();
    //input_count_down.start(10.millis());

    //let mut tick_count_down = timer.count_down();
    //tick_count_down.start(1.millis());

    loop {
        //let keys = &[Keyboard::A];

        //keyboard.interface().write_report(keys).ok();

        //tick once per ms
        //if tick_count_down.wait().is_ok() {
        //    keyboard.interface().tick().unwrap();
        //}

        //if usb_dev.poll(&mut [&mut keyboard]) {
        //    match keyboard.interface().read_report() {
        //        Ok(l) => {}
        //        _ => {}
        //    }
        //}
    }
}
