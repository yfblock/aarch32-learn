#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[macro_use]
pub mod subsystem;

pub mod arch;

fn main() {
    println!("Hello World!");
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {
        unsafe { core::arch::asm!("wfi") }
    }
}
