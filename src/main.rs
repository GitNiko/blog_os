#![no_std]
#![no_main]


use core::panic::PanicInfo;
mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print!("some numbers: {} {}", 29, 3.1415926);
    loop {}
}
