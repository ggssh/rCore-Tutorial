pub const USER_STACK_SIZE: usize = 4096 * 2;
pub const KERNEL_STACK_SIZE: usize = 4096 * 2;
pub const MAX_APP_NUM: usize = 5;
pub const APP_BASE_ADDRESS: usize = 0x80400000;
pub const APP_SIZE_LIMIT: usize = 0x20000;

// 动态内存分配
pub const KERNEL_HEAP_SIZE: usize = 0x30_0000;
pub const PAGE_SIZE: usize = 4096; // 页面大小
pub const PAGE_SIZE_BITS: usize = 12; // 页内偏移位宽
                                      /*
                                         物理页号区间:
                                         left: ekernel的物理地址以向上取整的方式转化为物理页号
                                         right: MEMORY_END以向下取整方式转化为物理页号
                                      */
pub const MEMORY_END: usize = 0x80800000; // 将内存大小设置为8MB
pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;
pub const TRAP_CONTEXT: usize = TRAMPOLINE - PAGE_SIZE;

// 时间中断相关
pub const CLOCK_FREQ: usize = 12500000;
