#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(hatch_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use hatch_os::println;
use core::panic::PanicInfo;
use hatch_os::vga_buffer::{Color, ColorCode};

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    hatch_os::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!(ColorCode::new(Color::Blue, Color::Black), "test_println output");
}
