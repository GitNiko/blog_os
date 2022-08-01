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
    blog_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(_info);
    blog_os::hlt_loop();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print!("some numbers: {} {}", 29, 3.1415926);

    blog_os::init();

    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    blog_os::hlt_loop();
}