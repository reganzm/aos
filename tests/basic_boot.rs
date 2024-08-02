#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(aos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use aos::println;
use core::panic::PanicInfo;
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[test_case]
fn test_println() {
    println!("test pritnln output");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    aos::test_panic_handler(info)
}
