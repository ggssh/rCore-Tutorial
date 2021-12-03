#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TaskStatus {
    UnInit,  // 未初始化
    Ready,   // 准备就绪
    Running, // 正在运行
    Exited,  // 已退出
}


use super::TaskContext;
// 任务控制块
#[derive(Debug, Clone, Copy)]
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext, // 任务上下文
}

