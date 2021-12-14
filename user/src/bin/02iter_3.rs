#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

const LEN: usize = 100;

#[no_mangle]
fn main() -> i32 {
    let iter: usize = 160000;
    for i in 1..=iter {
        if i % 10000 == 0 {
            println!("iter_3 : [{}/{}]", i, iter);
        }
    }
    println!("Test iter_3 OK!");
    0
}
