use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::arch::x86_64::gdt;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(df_handler)
                .set_stack_index(gdt::DF_IST)
        };
        idt
    };
}

pub fn init() {
    IDT.load();
}

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    panic!("{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn df_handler(stack_frame: InterruptStackFrame, code: u64) -> ! {
    panic!(
        "EXCEPTION: DOUBLE FAULT\n{:#?}\nCode: {}",
        stack_frame, code
    );
}
