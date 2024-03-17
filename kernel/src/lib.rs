#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;
use core::panic::PanicInfo;
use bootloader::{BootInfo};
use crate::vga_buffer::{Color, ColorCode};

pub mod allocator;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod serial;
pub mod task;
pub mod vga_buffer;

pub fn bootstrap(boot_info: &'static BootInfo) {
    use x86_64::VirtAddr;
    use memory::{BootInfoFrameAllocator};
    println!(ColorCode::new(Color::Blue, Color::Black), "[Bootstrap] Initializing Global Descriptor Table (GDT)");
    gdt::init();
    println!(ColorCode::new(Color::Blue, Color::Black), "[Bootstrap] GDT Initialized");
    println!(ColorCode::new(Color::Blue, Color::Black), "[Bootstrap] Initializing Interrupts.");
    interrupts::init_idt();
    println!(ColorCode::new(Color::Blue, Color::Black), "[Bootstrap] IDT Initialized");
    unsafe { interrupts::PICS.lock().initialize() };
    println!(ColorCode::new(Color::Blue, Color::Black), "[Bootstrap] PICs Initialized");
    x86_64::instructions::interrupts::enable();
    println!(ColorCode::new(Color::Blue, Color::Black), "[Bootstrap] Interrupts Loaded");
    println!(ColorCode::new(Color::Blue, Color::Black), "[Bootstrap] Initializing Memory Manager.");
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
    println!(ColorCode::new(Color::Blue, Color::Black), "[Bootstrap] Memory Manager Initialized");
    println!(ColorCode::new(Color::Blue, Color::Black), "[Bootstrap] System Bootstrapped");
}
pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(test)]
use bootloader::{entry_point, BootInfo};

#[cfg(test)]
entry_point!(test_kernel_main);

/// Entry point for `cargo xtest`
#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    init();
    test_main();
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
