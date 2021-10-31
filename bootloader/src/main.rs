#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

// Halt on panic
use panic_semihosting as _; // panic handler

mod blinky_arr;

use blinky_arr::BYTES;
use cortex_m_rt::entry;
use hal::{flash::FlashExt, pac, prelude::*};
use stm32f4xx_hal as hal;

pub const FLASH_USER: u32 = 0x0800_8000;

fn boot(scb: &mut cortex_m::peripheral::SCB) {
    unsafe {
        let rv: usize = *((FLASH_USER + 4) as *const usize);
        scb.vtor.write(FLASH_USER);
        let function = core::mem::transmute::<usize, extern "C" fn() -> !>(rv);
        function();
    }
}

#[entry]
fn main() -> ! {
    if let Some(mut cp) = cortex_m::peripheral::Peripherals::take() {
        let mut dp = pac::Peripherals::take().unwrap();

        let mut flash = dp.FLASH.unlocked();
        // sector 2 is from 0x0800_8000 to 0x0800_BFFF
        // erase more sectors if program is bigger than 16kb
        if let Err(err) = flash.erase(2) {
            let _err = err;
        }

        match flash.program(0x8000, BYTES.iter()) {
            Ok(()) => {
                drop(flash);
                // boot user program
                boot(&mut cp.SCB);
            }
            Err(err) => {
                // FIXME: handle error
                let _err = err;
            }
        }
    }

    loop {}
}
