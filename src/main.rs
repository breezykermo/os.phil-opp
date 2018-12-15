#![feature(exclusive_range_pattern)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)] // conditionally, so that tests can use main
#![cfg_attr(test, allow(unused_imports))]

extern crate volatile;
extern crate lazy_static;
extern crate spin;
mod vga_buffer;

use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    loop {}
}
