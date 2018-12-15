#![feature(exclusive_range_pattern)]
#![no_std]
#![no_main]

extern crate volatile;
extern crate lazy_static;
extern crate spin;
mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    panic!("some panic msg");

    loop {}
}
