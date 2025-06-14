#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(alloc_error_handler)]

extern crate alloc;

use core::panic::PanicInfo;
use alloc::vec::Vec;
use alloc::rc::Rc;
use alloc::boxed::Box;
use bootloader::{BootInfo, entry_point};
use x86_64::{VirtAddr, structures::paging::Page};

use cha_os::{
    println,
    interrupts,
    gdt::init,
    memory::{self, BootInfoFrameAllocator},
    hlt_loop,
    allocator,
};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello test{}","!");
    init();
    interrupts::init_idt();

    // Memory mapping
    let phys_mem_offset=VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper=unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator=unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // Heap initialization
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap init failed");

    // Allocating 69 on heap
    let heap_value=Box::new(69);
    println!("Heap value at {:p}", heap_value);

    // Vector
    let mut vec=Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // Reference counted vector
    let reference_counted=Rc::new(vec![1, 2, 3]);
    let cloned_reference=reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));

    // Page mapping example
    let page=Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // Writing into mapped page
    let page_ptr: *mut u64=page.start_address().as_mut_ptr();
    unsafe {
        page_ptr.offset(400).write_volatile(0xf021_f077_f065_f04e);
    }

    println!("No crash!");

    //print entire page table structure (debug heavy)
    /*
    use x86_64::structures::paging::PageTable;

    let l4_table=unsafe { memory::active_level_4_table(phys_mem_offset) };
    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);

            let phys=entry.frame().unwrap().start_address();
            let virt=phys.as_u64()+boot_info.physical_memory_offset;
            let l3_table: &PageTable=unsafe { &*(VirtAddr::new(virt).as_mut_ptr()) };

            for (j, entry) in l3_table.iter().enumerate() {
                if !entry.is_unused() {
                    println!("L3 Entry {}: {:?}", j, entry);

                    let phys=entry.frame().unwrap().start_address();
                    let virt=phys.as_u64()+boot_info.physical_memory_offset;
                    let l2_table: &PageTable=unsafe { &*(VirtAddr::new(virt).as_mut_ptr()) };

                    for (k, entry) in l2_table.iter().enumerate() {
                        if !entry.is_unused() {
                            println!("L2 Entry {}: {:?}", k, entry);

                            let phys=entry.frame().unwrap().start_address();
                            let virt=phys.as_u64()+boot_info.physical_memory_offset;
                            let l1_table: &PageTable=unsafe { &*(VirtAddr::new(virt).as_mut_ptr()) };

                            for (l, entry) in l1_table.iter().enumerate() {
                                if !entry.is_unused() {
                                    println!("L1 Entry {}: {:?}", l, entry);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    */

    //trigger exceptions for testing:
    /*
    // x86_64::instructions::interrupts::int3(); // Breakpoint
    // unsafe { *(0xdeadbeef as *mut u8)=42;}// Double fault
    // let ptr=0x2046b1 as *mut u8;
    // unsafe { let _=*ptr;}// Page fault read
    // unsafe { *ptr=42;}// Page fault write
    // stack_overflow();
    */

    hlt_loop();
}

/*
// Stack overflow test
fn stack_overflow() {
    stack_overflow(); 
}
*/

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}
