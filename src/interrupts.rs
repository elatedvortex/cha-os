#![feature(abi_x86_interrupt)]  // ✅ Required for `x86-interrupt`

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::vga_buffer;
use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

use crate::println;

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

