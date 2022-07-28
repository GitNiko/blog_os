#![no_std]
#![no_main]


#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;
use blog_os::{println, print};


#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(_info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print!("some numbers: {} {}", 29, 3.1415926);

    #[cfg(test)]
    test_main();

    loop {}
}