#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{get_time,sleep};

#[no_mangle]
fn main() -> i32 {
    println!("Enter sleep app");
    // let current_timer = get_time();
    // let wait_for = current_timer + 3000;
    // while get_time() < wait_for {
    //     yield_();
    // }
    let period_time:usize = 10000;
    sleep(period_time);
    println!("Test sleep OK!");
    0
}