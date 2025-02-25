#![no_std]
#![feature(abi_x86_interrupt)]

pub mod vga_buffer;
pub mod interrupts;

use x86_64::structures::idt::InterruptDescriptorTable;
pub fn init() {
    interrupts::init_idt();
}


