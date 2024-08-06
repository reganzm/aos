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

    //println!("hello world! {}", "regan");
    //panic!("SOME panic message");

    aos::init();
    //x86_64::instructions::interrupts::int3();

    //unsafe {
    //    *(0xdeadbeef as *mut u8) = 33;
    //}
    //fn stack_overflow() {
    //    stack_overflow();
    //}
    //stack_overflow();

    //loop {
    //    // 引发死锁
    //    use aos::print;
    //    for _ in 0..100000{}
    //    print!("——");
    //}

    //let ptr = 0x2031b2 as *mut u8;
    //unsafe {
    //    let x = *ptr;
    //};
    //println!("read ok");

    //unsafe {
    //    *ptr = 42;
    //}
    // 访问页表
    use x86_64::registers::control::Cr3;
    let (level_4_page_table_addr, cr3_flag) = Cr3::read();
    println!(
        "level 4 page table addr:{:?} cr3 flag:{:#?}",
        level_4_page_table_addr, cr3_flag
    );

    #[cfg(test)]
    test_main();

    println!("not crash!");
    aos::hlt_loop();
}

// 自定义panic处理函数
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    aos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    aos::test_panic_handler(info);
}
