#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;

use stm32f4xx_hal as hal;

use hal::{prelude::*, stm32};
use volatile::Volatile;

#[entry]
fn main() -> ! {
    let (cp, dp) = (
        cortex_m::peripheral::Peripherals::take().unwrap(),
        stm32::Peripherals::take().unwrap()
    );

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(168.mhz()).freeze();
    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

    let gpiod = dp.GPIOD.split();
    let mut led = (
        gpiod.pd12.into_push_pull_output(),
        gpiod.pd13.into_push_pull_output(),
        gpiod.pd14.into_push_pull_output(),
        gpiod.pd15.into_push_pull_output(),
    );

    let mut half_period = 500_u16;
    let volatile = Volatile::new(&mut half_period);
    loop {
        led.0.set_high().unwrap();
        delay.delay_ms(volatile.read());

        led.0.set_low().unwrap();
        delay.delay_ms(volatile.read());
    }
}
