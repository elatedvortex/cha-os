#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
use cha_os::interrupts;
use core::panic::PanicInfo;
use cha_os::vga_buffer;
use cha_os::println;
use cha_os::init;
use bootloader::{BootInfo, entry_point};
entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo)->!{
   crate::interrupts::init_idt();
    use  cha_os::memory::active_level_4_table;
    use cha_os::memory::translate_addr;
    use x86_64::VirtAddr;
    println!("Hello test{}","!");
    init();

    let phys_mem_offset=VirtAddr::new(boot_info.physical_memory_offset);
    let addresses=[
        0xb8000,0x20109,0x010202_001_1a,boot_info.physical_memory_offset,
    ];
    for &address in &addresses{
        let virt = VirtAddr::new(address);
        let phys=unsafe{translate_addr(virt,phys_mem_offset)};
        println!("{:?}->{:?}",virt,phys);
    }

    

    /*
    let l4_table=unsafe {
        active_level_4_table(
            phys_mem_offset)
    };
    for (i,entry) in l4_table.iter().enumerate(){
        if !entry.is_unused(){
            println!("L4 Entry {}: {:?}",i,entry);
        }

        use x86_64::structures::paging::PageTable;
        let phys=entry.frame().unwrap().start_address();
        let virt=phys.as_u64() + boot_info.physical_memory_offset;
        let ptr=VirtAddr::new(virt).as_mut_ptr();
        let l3_table: &PageTable =unsafe {&*ptr};
        

        for (i,entry) in l3_table.iter().enumerate(){
            if !entry.is_unused(){
                println!("L3 Entry {}:{:?}",i,entry);
            }
            use x86_64::structures::paging::PageTable;
            let phys=entry.frame().unwrap().start_address();
            let virt=phys.as_u64() + boot_info.physical_memory_offset;
            let ptr=VirtAddr::new(virt).as_mut_ptr();
            let l2_table: &PageTable =unsafe {&*ptr};
            
            for (i,entry) in l2_table.iter().enumerate(){
                if !entry.is_unused(){
                    println!("L2 Entry {}:{:?}",i,entry);

                     use x86_64::structures::paging::PageTable;
                     let phys=entry.frame().unwrap().start_address();
                     let virt=phys.as_u64() + boot_info.physical_memory_offset;
                     let ptr=VirtAddr::new(virt).as_mut_ptr();
                     let l1_table: &PageTable =unsafe {&*ptr};
                     for (i,entry) in l1_table.iter().enumerate(){
                         if !entry.is_unused(){
                             println!("L1 Entry {}:{:?}",i,entry);
                         }
                     }
                }
            }
        }
    }
    */

    
    //test for double fault handler
    //unsafe{
      //   *(0xdeadbeef as *mut u8)=42;
    //};
    //stack_overflow();
    //x86_64::instructions::interrupts::int3();
    //page fault test
    // let ptr=0x2046b1 as *mut u8;
    // unsafe { let x=*ptr;}
    // println!("read worked");
    // unsafe{*ptr=42;}
    // println!("Write worked");
    // 
    
    fn stack_overflow(){
        stack_overflow();   
    }
    println!("No crash!");
    cha_os::hlt_loop();
    //loop {
    //use cha_os::print;
    //print!("-");
    //}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    cha_os::hlt_loop();
}
