//! Demonstrate the use of a blocking `Delay` using TIM5 general-purpose timer.

#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

// Halt on panic
use panic_semihosting as _; // panic handler

use stm32f4xx_hal as _hal;

use cortex_m_rt::entry;

pub const FLASH_USER: u32 = 0x0800_8000;

fn boot(scb: &mut cortex_m::peripheral::SCB) {
    unsafe {
        // let sp: u32 = *(FLASH_USER as *const u32);
        let rv: usize = *((FLASH_USER + 4) as *const usize);
        scb.vtor.write(FLASH_USER);
        // cortex_m::register::msp::write(sp);
        let function = core::mem::transmute::<usize, extern "C" fn() -> !>(rv);
        function();
    }
}

#[entry]
fn main() -> ! {
    if let Some(mut cp) = cortex_m::peripheral::Peripherals::take() {
        boot(&mut cp.SCB);
    }

    loop {}
}
