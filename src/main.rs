#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(sopeos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use sopeos::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    sopeos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    sopeos::test_panic_handler(info)
}

use bootloader::{entry_point, BootInfo};
entry_point!(kernel_main);

// #[no_mangle]
// pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("你好，世界. Hello, World{}", "!");

    sopeos::init();
    // 测试 触发断点异常
    // x86_64::instructions::interrupts::int3();
    // trigger a page fault
    // 内存页异常触发
    // let ptr = 0x2055e6 as *mut u32;
    // unsafe {
    //     let x = *ptr;
    //     println!("x:{}", x)
    // } // 读取只读内存区域
    // println!("read worked");
    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // };
    // 触发循环栈溢出
    // fn stack_overflow() {
    //     stack_overflow(); // for each recursion, the return address is pushed
    // }
    // // trigger a stack overflow
    // stack_overflow();

    use sopeos::memory;
    use x86_64::{structures::paging::Page, VirtAddr};

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = memory::EmptyFrameAllocator;
    let page = Page::containing_address(VirtAddr::new(0));

    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    sopeos::hlt_loop();
    // loop {
    //     // sopeos::print!("-"); // 测试 与中断控制器形成死锁
    // }
}
