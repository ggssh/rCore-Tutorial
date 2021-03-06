fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        llvm_asm!(
            "ecall"
            : "={x10}" (ret)
            : "{x10}" (args[0]),"{x11}" (args[1]),"{x12}" (args[2]), "{x17}" (id)
            : "memory"
            : "volatile"
        );
    }
    ret
}

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_READ: usize = 63;

// 功能：退出应用程序并将返回值告知批处理系统。
// 参数：`xstate` 表示应用程序的返回值。
// 返回值：该系统调用不应该返回。
// syscall ID：93
pub fn sys_exit(xstate: i32) -> isize {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}

/*封装SYSCALL_WRITE系统调用*/
// 功能：将内存中缓冲区中的数据写入文件。
// 参数：`fd` 表示待写入文件的文件描述符；
//      `buf` 表示内存中缓冲区的起始地址；
//      `len` 表示内存中缓冲区的长度。
// 返回值：返回成功写入的长度。
// syscall ID：64
pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn sys_read(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_READ, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn sys_yield() -> isize {
    syscall(SYSCALL_YIELD, [0, 0, 0])
}

pub fn sys_get_time() -> isize {
    syscall(SYSCALL_GET_TIME, [0, 0, 0])
}
