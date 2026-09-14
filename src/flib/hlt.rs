use x86_64::instructions::hlt;

pub fn exec() -> ! {
    loop {
        hlt();
    }
}
