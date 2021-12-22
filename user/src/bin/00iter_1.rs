#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

const LEN: usize = 100;

#[no_mangle]
fn main() -> i32 {
    let iter: usize = 200000;
    for i in 1..=iter {
        if i % 10000 == 0 {
            println!("iter_1 : [{}/{}]", i, iter);
        }
    }
    println!("Test iter_1 OK!");
    0
}
