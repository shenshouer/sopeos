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

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    sopeos::init();
    // 测试 触发断点异常
    // x86_64::instructions::interrupts::int3();
    // trigger a page fault
    // 内存页异常触发
    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // };
    // 触发循环栈溢出
    // fn stack_overflow() {
    //     stack_overflow(); // for each recursion, the return address is pushed
    // }
    // // trigger a stack overflow
    // stack_overflow();

    #[cfg(test)]
    test_main();

    sopeos::hlt_loop();
    // loop {
    //     // sopeos::print!("-"); // 测试 与中断控制器形成死锁
    // }
}
