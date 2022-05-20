#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![deny(unsafe_op_in_unsafe_fn)]

extern crate alloc;

use core::panic::PanicInfo;
use bootloader::{entry_point, BootInfo};
use blog_os::{allocator, println};
use alloc::{
    boxed::Box,
    vec,
    vec::Vec,
    rc::Rc,
};

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use x86_64::VirtAddr;
    use blog_os::memory::{self, BootInfoFrameAllocator};

    println!("Hello, World!");
    blog_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::new(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("Heap initialization failed");

    let heap_value = Box::new(42);
    println!("`heap_value` at: {:p}", heap_value);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("`vec` at: {:p}", vec.as_slice());

    let rc_vec = Rc::new(vec![1, 2, 3]);
    let cloned_rc_vec = Rc::clone(&rc_vec);

    println!("Current `rc_vec` reference count is: {}", Rc::strong_count(&cloned_rc_vec));
    drop(rc_vec);
    println!("`rc_vec` reference count is now: {}", Rc::strong_count(&cloned_rc_vec));

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
