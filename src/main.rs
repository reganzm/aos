// 禁用标准库
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(aos::test_runner)]
#![reexport_test_harness_main = "test_main"]
use aos::println;
use aos::serial_print;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
extern crate alloc;
use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use x86_64::VirtAddr;

use aos::task::{simple_executor::SimpleExecutor, Task};

async fn async_number() -> u32 {
    42
}
async fn example_task() {
    let number = async_number().await;
    println!("async number:{}", number);
}

//#[no_mangle]
//pub extern "C" fn _start(boot_info:&'static BootInfo) -> ! {
entry_point!(kernal_main);
fn kernal_main(boot_info: &'static BootInfo) -> ! {
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
    //println!("{:#?}", boot_info);
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
    //use x86_64::registers::control::Cr3;
    //let (level_4_page_table_addr, cr3_flag) = Cr3::read();
    //println!(
    //    "level 4 page table addr:{:#?} cr3 flag:{:#?}",
    //    level_4_page_table_addr, cr3_flag
    //);

    // 使用memory模块访问内存和页表
    //use aos::memory::active_level_4_table;
    //use x86_64::structures::paging::{Page, PageTable};
    //    use x86_64::VirtAddr;
    //let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    //let l4_table = unsafe { active_level_4_table(phys_mem_offset) };
    //for (i, entry) in l4_table.iter().enumerate() {
    //    if !entry.is_unused() {
    //        println!("L4 entry{}:{:?}", i, entry);
    //        let phys = entry.frame().unwrap().start_address();
    //        let virt = phys.as_u64() + boot_info.physical_memory_offset;
    //        let ptr = VirtAddr::new(virt).as_mut_ptr();
    //        let l3_table: &PageTable = unsafe { &*ptr };
    //        for (i, entry) in l3_table.iter().enumerate() {
    //            if !entry.is_unused() {
    //                println!(" L3 entry {}:{:?}", i, entry);
    //            }
    //        }
    //    }
    // }
    // 虚拟地址转物理地址
    // use aos::memory;
    // use aos::memory::translate_addr;
    // use aos::memory::BootInfoFrameAllocator;
    // use x86_64::structures::paging::Translate;

    //let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    //let mapper = unsafe { memory::init(phys_mem_offset) };
    //let addresses = [
    //    0xb8000,
    //    0x201008,
    //    0x0100_0020_1a10,
    //    boot_info.physical_memory_offset,
    //];

    //for &addr in &addresses {
    //    let virt = VirtAddr::new(addr);
    //let phys = unsafe { translate_addr(virt, phys_mem_offset) };
    //use x86_64 mappter translate_addr
    //    let phys = mapper.translate_addr(virt);
    //    println!("{:?}->{:?}", virt, phys);
    //}
    // 实现分页
    //let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    //let mut mapper = unsafe { memory::init(phys_mem_offset) };
    //let mut frame_allocator = memory::EmptyFrameAllocator;
    //let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    // map an unused page
    // let page = Page::containing_address(VirtAddr::new(0));
    // memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    // unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };
    // 堆内存分配
    use aos::allocator;
    use aos::memory::{self, BootInfoFrameAllocator};

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    let x = Box::new(33);
    println!("heap value as:{:p}", x);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at:{:p}", vec.as_slice());

    let rc = Rc::new(vec![2, 3, 4]);
    let cr = rc.clone();
    println!("current reference count is:{}", Rc::strong_count(&cr));
    core::mem::drop(rc);
    println!("current reference count is:{}", Rc::strong_count(&cr));
    // 异步任务
    let mut executor = SimpleExecutor::new();
    executor.spawn(Task::new(example_task()));
    executor.run();
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
