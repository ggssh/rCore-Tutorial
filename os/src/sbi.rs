// 关机指令相关参数
const SBI_SHUTDOWN: usize = 8;
// 添加裸机打印相关函数
const SBI_CONSOLE_PUTCHAR: usize = 1;
// 设置mtimecmp的值(感谢rust_sbi,太令人泪目了)
const SBI_SET_TIMER: usize = 0;

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

pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}

// 由 SEE 提供的标准 SBI 接口函数
pub fn set_timer(timer: usize) {
    sbi_call(SBI_SET_TIMER, timer, 0, 0);
}

pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    panic!("It should shutdown");
}
