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

// 输出比较重要的信息
#[macro_export]
macro_rules! info {
    ($fmt: literal $(,$($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!("\x1b[34m",$fmt,"\x1b[0m","\n") $(, $($arg)+)?));
    };
}

// 表示发生不常见错误,但是并不一定导致系统错误
#[macro_export]
macro_rules! warn {
    ($fmt: literal $(,$($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!("\x1b[93m",$fmt,"\x1b[0m","\n") $(, $($arg)+)?));
    };
}

// 表示发生严重错误,很可能已经导致程序崩溃
#[macro_export]
macro_rules! error {
    ($fmt: literal $(,$($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!("\x1b[31m",$fmt,"\x1b[0m","\n") $(, $($arg)+)?));
    };
}
