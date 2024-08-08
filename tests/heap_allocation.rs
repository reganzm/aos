#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(aos::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    use aos::allocator;
    use aos::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;
    aos::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    aos::test_panic_handler(info)
}

use alloc::boxed::Box;
#[test_case]
fn simple_allocation() {
    let heap_value1 = Box::new(10);
    let heap_value2 = Box::new(11);
    assert_eq!(*heap_value1, 10);
    assert_eq!(*heap_value2, 11);
}

use alloc::vec::Vec;
#[test_case]
fn large_vec() {
    let n = 10000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }

    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}

use aos::allocator::HEAP_SIZE;
#[test_case]
fn many_boxes() {
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}
