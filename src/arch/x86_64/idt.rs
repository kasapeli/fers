extern crate alloc;

use crate::{flib::hlt, println};
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

use crate::arch::x86_64::gdt;
use crate::drivers::keyboard;

// pic
pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 1;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(df_handler)
                .set_stack_index(gdt::DF_IST)
        };
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt[InterruptIndex::Timer.as_u8()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_u8()].set_handler_fn(keyboard::keyboard_handler);
        idt
    };
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    pub fn as_u8(self) -> u8 {
        self as u8
    }
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

pub extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;
    println!(
        "EXCEPTION: PAGE FAULT\nAccessed Address: {:?}\nError Code: {:#?}\nStack Frame: {:#?}",
        Cr2::read(),
        code,
        stack_frame
    );
    hlt::exec();
}

pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}
