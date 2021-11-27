#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod lang_item;
mod sbi;

global_asm!(include_str!("entry.asm"));

/*清空.bss段*/
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

// #[no_mangle]
// extern "C" fn _start() {
//     println!("Hello rCore-Turtial !");
//     // sys_exit(9);
//     shutdown();
// }

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
    println!("Hello world");
    // error!("rust yes");
    // println!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    // println!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    // println!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    println!(
        "boot_stack [{:#x}, {:#x})",
        boot_stack as usize, boot_stack_top as usize
    );
    // println!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    // // sbi::shutdown();
    panic!("It should shutdown!");
}
