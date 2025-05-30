use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};
use x86_64::instructions::tables::load_tss;
use lazy_static::lazy_static;
pub const DOUBLE_FAULT_IST_INDEX: u16=0;
//tss
lazy_static! {
    static ref TSS: TaskStateSegment={
        let mut tss=TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize]={
            const STACK_SIZE: usize=4096 * 5;
            static mut STACK: [u8; STACK_SIZE]=[0; STACK_SIZE];

            unsafe {
                let stack_start=VirtAddr::from_ptr(STACK.as_ptr());
                let stack_end=stack_start + STACK_SIZE as u64;
                stack_end
            }
        };
        tss
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}
//gdt things
lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors)={
        let mut gdt=GlobalDescriptorTable::new();
        let code_selector=gdt.append(Descriptor::kernel_code_segment());
        let tss_selector=gdt.append(Descriptor::tss_segment(&TSS)); 
        (gdt, Selectors {code_selector,tss_selector})
    };
}

pub fn init() {
    GDT.0.load();
    unsafe { load_tss(GDT.1.tss_selector); }
}
