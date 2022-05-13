#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop { x86_64::instructions::hlt() }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop { x86_64::instructions::hlt() }
}

#[test_case]
fn test_println() {
    println!("Hello, World!");
}
