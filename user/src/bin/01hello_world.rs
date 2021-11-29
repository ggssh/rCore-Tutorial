#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("Hello world(It is glad to see these words!)");
    println!("102");
    unsafe {
        asm!("sret");
    }
    0
}
