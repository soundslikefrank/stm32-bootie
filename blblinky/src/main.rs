//! Demonstrate the use of a blocking `Delay` using TIM5 general-purpose timer.

#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

// Halt on panic
use panic_semihosting as _; // panic handler

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*};

/* pub const FLASH_USER: u32 = 0x0808_0000;
fn boot(scb: &mut cortex_m::peripheral::SCB) {
    unsafe {
        // let sp: u32 = *(FLASH_USER as *const u32);
        let rv: usize = *((FLASH_USER + 4) as *const usize);
        scb.vtor.write(FLASH_USER);
        // cortex_m::register::msp::write(sp);
        let function = core::mem::transmute::<usize, extern "C" fn() -> !>(rv);
        function();
    }
} */

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(_cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the LED. On the Mini-F4 it's connected to pin PC13.
        let gpioa = dp.GPIOA.split();
        let mut led = gpioa.pa5.into_push_pull_output();

        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        // Create a delay abstraction based on general-pupose 32-bit timer TIM5
        let mut delay = hal::delay::Delay::tim5(dp.TIM5, &clocks);

        loop {
            // On for 1s, off for 1s.
            led.set_high();
            delay.delay_ms(1_000_u32);
            led.set_low();
            delay.delay_us(1_000_000_u32);
        }
    }

    loop {}
}
