#![test_runner(sopeos::test_runner)]
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use sopeos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    sopeos::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    sopeos::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
