#![no_std]
#![no_main]
#![feature(asm)]

use core::panic::PanicInfo;

mod vga_buffer;

static HELLO: &[u8] = b"Hello ouesu!";
#[no_mangle] // bootloader
pub extern "C" fn _start() -> ! {
    // let vga_buffer= 0xb8000 as *mut u8;
    //
    // for(i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }
    vga_buffer::print_something();
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
