global_asm!(include_str!("switch.S"));

use super::TaskContext;

extern "C" {
    // 在调用前后Rust编译器会自动插入保存/恢复调用者保存寄存器的汇编代码
    // 将switch.S 中的__switch封装为一个函数
    pub fn __switch(current_task_cx_ptr: *mut TaskContext, next_task_cx_ptr: *const TaskContext);
    // 使用*mut 是因为它的sp和ra以及12个寄存器会变
}
