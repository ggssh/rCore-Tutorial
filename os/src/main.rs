#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

use crate::{timer::set_next_trigger, trap::enable_timer_interrupt};
extern crate alloc;

#[macro_use]
mod console;
// mod batch;
mod config;
mod lang_item;
mod loader;
mod sbi;
mod sync;
mod syscall;
mod task;
mod timer;
mod trap;
mod mm;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

/*清空.bss段*/
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

// 避免编译器对它的名字进行混淆,导致entry.asm找不到外部符号rust_main从而链接失败
#[no_mangle]
pub fn rust_main() {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn boot_stack();
        fn boot_stack_top();
    }
    clear_bss();

    // mm::init();
    // mm::mm_test_heap();

    info!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    info!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    info!(
        "boot_stack [{:#x}, {:#x})",
        boot_stack as usize, boot_stack_top as usize
    );
    info!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    // // sbi::shutdown();
    // panic!("It should shutdown!");
    trap::init();

    loader::load_apps();
    enable_timer_interrupt();
    set_next_trigger();
    task::run_first_task();
    // batch::init();
    // batch::run_next_app();
    panic!("Unreachable in rust_main!");
}
