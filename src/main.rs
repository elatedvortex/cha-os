#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
use cha_os::interrupts;
use core::panic::PanicInfo;
use cha_os::vga_buffer;
use cha_os::println;
use cha_os::init;
#[no_mangle]
pub extern "C" fn _start() -> ! {
   crate::interrupts::init_idt();
    println!("Hello test");
    //unsafe{
      //   *(0xdeadbeef as *mut u8) = 42;
    //};
    init();
    stack_overflow();
    x86_64::instructions::interrupts::int3();
    fn stack_overflow(){
        stack_overflow();   
    }
    println!("No crash!");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

