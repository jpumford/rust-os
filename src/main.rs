// Don't use std library
#![no_std]
// Don't use the normal entry point chain since we don't have the normal rust runtime
#![no_main]

use core::panic::PanicInfo;
mod vga_buffer;

// static HELLO: &[u8] = b"Hello World!";

// use our own start function for the linker
// don't mangle the function name so the _main label is used in the output .so
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // let vga_buffer = 0xb8000 as *mut u8;

    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }
    vga_buffer::print_stuff();

    loop {}
}

// Define our own panic handler since we don't have the std lib panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}