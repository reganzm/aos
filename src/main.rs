// 禁用标准库
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(aos::test_runner)]
#![reexport_test_harness_main = "test_main"]
use aos::println;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    //let vga_buffer = 0xb8000 as *mut u8;
    //for (i, &byte) in HELLO.iter().enumerate() {
    //    unsafe {
    //        *vga_buffer.offset(i as isize * 2) = byte;
    //        *vga_buffer.offset(i as isize * 2 + 1) = 0xd9;
    //    }
    //}
    //vga_buffer::print_something();
    //use core::fmt::Write;
    //vga_buffer::WRITER.lock().write_str("Hello world").unwrap();
    //write!(
    //    vga_buffer::WRITER.lock(),
    //    "some numbers:{} {}",
    //    234,
    //    1. / 3.
    //)
    // .unwrap();

    println!("hello world! {}", "regan");
    //panic!("SOME panic message");

    aos::init();
    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("testing...");
    loop {}
}

// 自定义panic处理函数
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    aos::test_panic_handler(info);
}