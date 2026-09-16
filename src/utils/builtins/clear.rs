use crate::drivers::vga::WRITER;

pub fn exec() {
    WRITER.lock().clear();
}
