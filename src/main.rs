#![no_std]
#![no_main]

// Some panic handler needs to be included. This one halts the processor on panic.
use panic_halt as _;

use core::convert::Infallible;
use hal::gpio::{Input, Output, Pin, PullUp, PushPull};
use hal::prelude::*;
use hal::usb;
use hal::{stm32, timers};
use keyberon::debounce::Debouncer;
use keyberon::key_code::KbHidReport;
use keyberon::layout::{CustomEvent, Event, Layout};
use keyberon::matrix::Matrix;
use rtic::app;
use stm32f0xx_hal as hal;
use usb_device::bus::UsbBusAllocator;
use usb_device::class::UsbClass as _;
use usb_device::device::UsbDeviceState;

mod io_expander;
mod layout;
mod right;
use io_expander::IoExpander;
use right::Right;

type UsbClass = keyberon::Class<'static, usb::UsbBusType, ()>;
type UsbDevice = usb_device::device::UsbDevice<'static, usb::UsbBusType>;

trait ResultExt<T> {
    fn get(self) -> T;
}
impl<T> ResultExt<T> for Result<T, Infallible> {
    fn get(self) -> T {
        match self {
            Ok(v) => v,
            Err(e) => match e {},
        }
    }
}

#[app(device = crate::hal::pac, peripherals = true, dispatchers = [CEC_CAN])]
mod app {
    use super::*;

    #[shared]
    struct Shared {
        usb_dev: UsbDevice,
        usb_class: UsbClass,
        #[lock_free]
        layout: Layout<10, 4, 1, ()>,
    }

    #[local]
    struct Local {
        matrix: Matrix<Pin<Input<PullUp>>, Pin<Output<PushPull>>, 5, 4>, // left
        right: Right,
        debouncer_left: Debouncer<[[bool; 5]; 4]>,
        debouncer_right: Debouncer<[[bool; 5]; 4]>,
        timer: timers::Timer<stm32::TIM3>,
    }

    #[init(local = [bus: Option<UsbBusAllocator<usb::UsbBusType>> = None])]
    fn init(mut c: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut rcc = c
            .device
            .RCC
            .configure()
            .hsi48()
            .enable_crs(c.device.CRS)
            .sysclk(48.mhz())
            .pclk(24.mhz())
            .freeze(&mut c.device.FLASH);

        let gpioa = c.device.GPIOA.split(&mut rcc);
        let gpiob = c.device.GPIOB.split(&mut rcc);

        let usb = usb::Peripheral {
            usb: c.device.USB,
            pin_dm: gpioa.pa11,
            pin_dp: gpioa.pa12,
        };
        *c.local.bus = Some(usb::UsbBusType::new(usb));
        let usb_bus = c.local.bus.as_ref().unwrap();

        let usb_class = keyberon::new_class(usb_bus, ());
        let usb_dev = keyberon::new_device(usb_bus);

        let mut timer = timers::Timer::tim3(c.device.TIM3, 1.khz(), &mut rcc);
        timer.listen(timers::Event::TimeOut);

        let pins = cortex_m::interrupt::free(move |cs| {
            (
                gpiob.pb10.into_alternate_af1(cs), // SCL
                gpiob.pb11.into_alternate_af1(cs), // SDA
            )
        });
        let io_expander = IoExpander::new(c.device.I2C2, pins, &mut rcc);
        let right = Right::new(io_expander);

        let matrix = cortex_m::interrupt::free(move |cs| {
            Matrix::new(
                [
                    // cols
                    gpiob.pb8.into_pull_up_input(cs).downgrade(),
                    gpiob.pb4.into_pull_up_input(cs).downgrade(),
                    gpiob.pb3.into_pull_up_input(cs).downgrade(),
                    gpioa.pa15.into_pull_up_input(cs).downgrade(),
                    gpioa.pa14.into_pull_up_input(cs).downgrade(),
                ],
                [
                    // rows
                    gpiob.pb7.into_push_pull_output(cs).downgrade(),
                    gpiob.pb6.into_push_pull_output(cs).downgrade(),
                    gpiob.pb5.into_push_pull_output(cs).downgrade(),
                    gpioa.pa2.into_push_pull_output(cs).downgrade(),
                ],
            )
        });

        (
            Shared {
                usb_dev,
                usb_class,
                layout: Layout::new(&crate::layout::LAYERS),
            },
            Local {
                matrix: matrix.get(),
                right,
                debouncer_left: Debouncer::new([[false; 5]; 4], [[false; 5]; 4], 5),
                debouncer_right: Debouncer::new([[false; 5]; 4], [[false; 5]; 4], 5),
                timer,
            },
            init::Monotonics(),
        )
    }

    #[task(binds = USB, priority = 3, shared = [usb_dev, usb_class])]
    fn usb_rx(c: usb_rx::Context) {
        (c.shared.usb_dev, c.shared.usb_class).lock(|usb_dev, usb_class| {
            if usb_dev.poll(&mut [usb_class]) {
                usb_class.poll();
            }
        });
    }

    #[task(priority = 2, capacity = 8, shared = [layout])]
    fn handle_event(c: handle_event::Context, event: Event) {
        c.shared.layout.event(event)
    }

    #[task(priority = 2, shared = [usb_dev, usb_class, layout])]
    fn tick_keyberon(mut c: tick_keyberon::Context) {
        let tick = c.shared.layout.tick();
        if c.shared.usb_dev.lock(|d| d.state()) != UsbDeviceState::Configured {
            return;
        }
        if let CustomEvent::Release(()) = tick {
            unsafe {
                cortex_m::asm::bootload(0x1FFFC800 as _);
            }
        }
        let report: KbHidReport = c.shared.layout.keycodes().collect();
        if !c
            .shared
            .usb_class
            .lock(|k| k.device_mut().set_keyboard_report(report.clone()))
        {
            return;
        }
        while let Ok(0) = c.shared.usb_class.lock(|k| k.write(report.as_bytes())) {}
    }

    #[task(
        binds = TIM3,
        priority = 1,
        local = [matrix, debouncer_left, debouncer_right, timer, right],
    )]
    fn tick(c: tick::Context) {
        c.local.timer.wait().ok();

        for event in c.local.debouncer_left.events(c.local.matrix.get().get()) {
            handle_event::spawn(event).unwrap();
        }
        for event in c
            .local
            .debouncer_right
            .events(c.local.right.get().get())
            .map(|e| e.transform(|i, j| (i, 5 + j)))
        {
            handle_event::spawn(event).unwrap();
        }
        tick_keyberon::spawn().unwrap();
    }
}
