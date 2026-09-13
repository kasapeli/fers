#[macro_export]
macro_rules! print {
        ($($arg:tt)*) => ($crate::drivers::vga::print(format_args!($($arg)*)));
}
