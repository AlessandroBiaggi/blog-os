#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![deny(unsafe_op_in_unsafe_fn)]

use core::panic::PanicInfo;
use bootloader::{entry_point, BootInfo};
use blog_os::println;

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use x86_64::VirtAddr;
    use blog_os::memory::{self, BootInfoFrameAllocator};

    println!("Hello, World!");
    blog_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut _mapper = unsafe { memory::init(phys_mem_offset) };
    let mut _frame_allocator = unsafe {
        BootInfoFrameAllocator::new(&boot_info.memory_map)
    };

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    blog_os::hlt_loop()
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blog_os::hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}


#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
