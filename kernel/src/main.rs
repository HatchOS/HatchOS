#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(hatch_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use hatch_os::println;
use hatch_os::task::{executor::Executor, keyboard, Task};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use hatch_os::vga_buffer::{Color, ColorCode};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {

    println!(ColorCode::new(Color::Green, Color::Black), "[Kernel] hatchOS kernel 0.1.0");
    println!(ColorCode::new(Color::Green, Color::Black), "[Kernel] Created by PieyIsAPie and TacoDark");
    println!(ColorCode::new(Color::Green, Color::Black), "[Kernel] Bootstrapping hatchOS...");
    hatch_os::bootstrap(&boot_info);
    println!(ColorCode::new(Color::Green, Color::Black), "[Kernel] Bootstrapped");

    #[cfg(test)]
    test_main();

    let mut executor = Executor::new();
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!(ColorCode::new(Color::Red, Color::Black), "PANIC: {}", info);
    hatch_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    hatch_os::test_panic_handler(info)
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!(ColorCode::new(Color::Yellow, Color::Black), "async number: {}", number);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
