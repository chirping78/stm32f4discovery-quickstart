#![no_main]
#![no_std]

use panic_halt as _;

#[allow(unused_imports)]
use cortex_m::{iprintln, Peripherals};
use cortex_m_rt::entry;

use stm32f4xx_hal as hal;

use hal::{prelude::*, stm32};

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

    let gpiod = dp.GPIOD.split();

    gpiod.pd12.into_push_pull_output();
    gpiod.pd13.into_push_pull_output();
    gpiod.pd14.into_push_pull_output();
    gpiod.pd15.into_push_pull_output();

    // port d
    // 12, 13, 14, 15
    const GPIOD_BSRR: u32 = 0x4002_0C00 + 0x18;

    unsafe {
        core::ptr::write_volatile(GPIOD_BSRR as *mut u32, 1 << 12);
        core::ptr::write_volatile(GPIOD_BSRR as *mut u32, 1 << 13);
        core::ptr::write_volatile(GPIOD_BSRR as *mut u32, 1 << 14);
        core::ptr::write_volatile(GPIOD_BSRR as *mut u32, 1 << 15);
        core::ptr::write_volatile(GPIOD_BSRR as *mut u32, 1 << (12 + 16));
        core::ptr::write_volatile(GPIOD_BSRR as *mut u32, 1 << (13 + 16));
        core::ptr::write_volatile(GPIOD_BSRR as *mut u32, 1 << (14 + 16));
        core::ptr::write_volatile(GPIOD_BSRR as *mut u32, 1 << (15 + 16));
    }

    loop {}
}
