#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]

#[macro_use]
mod sbi;
mod console;
mod lang_item;


/*清空.bss段*/
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

global_asm!(include_str!("entry.asm"));

// #[no_mangle]
// extern "C" fn _start() {
//     println!("Hello rCore-Turtial !");
//     // sys_exit(9);
//     shutdown();
// }

#[no_mangle]
pub fn rust_main()  {
    clear_bss();
    println!("Hello world");
    sbi::shutdown();
    // panic!("It should shutdown!");
}
