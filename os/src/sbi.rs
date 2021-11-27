/*关机指令相关参数 */
const SBI_SHUTDOWN: usize = 8;
/*添加裸机打印相关函数*/
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_WRITE: usize = 64;

#[inline(always)]
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        llvm_asm!("ecall"
            : "={x10}" (ret)
            : "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x17}" (which)
            : "memory"
            : "volatile");
    }
    ret
}

// fn syscall(id: usize, args: [usize; 3]) -> isize {
//     let mut ret: isize;
//     unsafe {
//         llvm_asm!("ecall"
//             : "={x10}" (ret)
//             : "{x10}" (args[0]), "{x11}" (args[1]), "{x12}" (args[2]), "{x17}" (id)
//             : "memory"
//             : "volatile"
//         );
//     }
//     ret
// }

pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}

pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    panic!("It should shutdown");
}

// pub fn sys_exit(xstate: i32) -> usize {
//     sbi_call(SYSCALL_EXIT, xstate as usize, 0, 0)
// }

// /*封装SYSCALL_WRITE系统调用*/
// pub fn sys_write(fd: usize, buffer: &[u8]) -> usize {
//     sbi_call(SYSCALL_WRITE, fd, buffer.as_ptr() as usize, buffer.len())
// }
