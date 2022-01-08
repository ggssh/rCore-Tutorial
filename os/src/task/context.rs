use crate::trap::trap_return;

#[derive(Debug, Clone, Copy)]
#[repr(C)] // 按照C内存布局
pub struct TaskContext {
    ra: usize, // 64bit = 8Bytes
    sp: usize,
    s: [usize; 12],
}

impl TaskContext {
    // 全部初始化为0
    pub fn zero_init() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }

    pub fn goto_restore(kstack_ptr: usize) -> Self {
        extern "C" {
            fn __restore();
        }
        Self {
            ra: __restore as usize,
            sp: kstack_ptr,
            s: [0; 12],
        }
    }

    pub fn goto_trap_return(kstack_ptr: usize) -> Self {
        Self {
            ra: trap_return as usize,
            sp: kstack_ptr,
            s: [0; 12],
        }
    }
}
