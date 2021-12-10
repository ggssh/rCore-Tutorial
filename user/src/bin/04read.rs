#![no_std]
#![no_main]

use user_lib::console::getchar;

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    // read syscall 通过在将输入的字符打印出来来说明系统调用成功
    getchar();
    0
}
