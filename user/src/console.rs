use crate::write;
use crate::read;
use core::fmt::{self, Write};

const STDOUT: usize = 1;
const STDIN: usize = 0;
struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write(STDOUT, s.as_bytes());
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(,$($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    };
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(,$($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt,"\n") $(, $($arg)+)?));
    };
}

// 输出比较重要的信息
#[macro_export]
macro_rules! info {
    ($fmt: literal $(,$($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!("\x1b[34m","[INFO]",$fmt,"\x1b[0m","\n") $(, $($arg)+)?));
    };
}

// 表示发生不常见错误,但是并不一定导致系统错误
#[macro_export]
macro_rules! warn {
    ($fmt: literal $(,$($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!("\x1b[93m","[WARNING]",$fmt,"\x1b[0m","\n") $(, $($arg)+)?));
    };
}

// 表示发生严重错误,很可能已经导致程序崩溃
#[macro_export]
macro_rules! error {
    ($fmt: literal $(,$($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!("\x1b[31m","[ERROR]",$fmt,"\x1b[0m","\n") $(, $($arg)+)?));
    };
}

pub fn getchar() -> u8 {
    let mut c = [0u8;1];
    read(STDIN,&mut c);
    c[0]
}