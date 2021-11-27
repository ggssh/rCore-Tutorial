use crate::sbi::console_putchar;
use core::fmt::{self, Write};

struct Stdout;

// impl Write for Stdout {
//     fn write_str(&mut self, s: &str) -> core::fmt::Result {
//         sys_write(1, s.as_bytes());
//         Ok(())
//     }
// }

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

// #[macro_export]
// macro_rules! error {
//     ($fmt: literal $(, $($arg: tt)+)?) => {
//         $crate::console::print(format_args!("\x1b[31m{}\x1b[0m",concat!($fmt, "\n") $(, $($arg)+)?));
//     };
// }
