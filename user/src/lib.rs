#![no_std]
// #![feature(asm)]
#![feature(llvm_asm)]
// 支持弱连接操作:如果在bin目录下找不到任何main,编译也能够通过并会在运行时报错
#![feature(linkage)]
#![feature(panic_info_message)]

#[macro_use]
pub mod console;
mod lang_items;
mod syscall;

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    clear_bss();
    // loop {}
    // unsafe {
    //     HEAP.lock()
    //         .init(HEAP_SPACE.as_ptr() as usize, USER_HEAP_SIZE);
    // }
    exit(main());
    panic!("unreachable after sys_exit!");
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}

// 清理bss段
fn clear_bss() {
    extern "C" {
        // 得到bss段起始地址
        fn start_bss();
        fn end_bss();
    }
    (start_bss as usize..end_bss as usize).for_each(|addr| unsafe {
        (addr as *mut u8).write_volatile(0);
    });
}

// 对系统调用再一次进行封装
use syscall::*;

pub fn write(fd: usize, buffer: &[u8]) -> isize {
    sys_write(fd, buffer)
}
pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}

pub fn yield_() -> isize {
    sys_yield()
}

pub fn get_time() -> isize {
    sys_get_time()
}
