mod context;

pub use context::TrapContext;

// use crate::batch::run_next_app;
use crate::{syscall::syscall, task::suspend_current_and_run_next, timer::set_next_trigger};
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Trap},
    stval, stvec,
};

global_asm!(include_str!("trap.S"));

pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        // 此时进入S模式的Trap无论原因如何,处理Trap的入口地址都是BASE<<2
        stvec::write(__alltraps as usize, TrapMode::Direct)
    }
}

use riscv::register::sie;

// 防止时钟中断被屏蔽
pub fn enable_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}

// 在trap.asm中调用
#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    // info!("call trap_handler");
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            error!("[kernel] PageFault in application, core dumped.");
            // run_next_app();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            error!("[kernel] IllegalInstruction in application, core dumped.");
            // run_next_app();
        }
        Trap::Interrupt(scause::Interrupt::SupervisorTimer) => {
            // 触发S mode时钟中断时,首先重新设置一个10ms计时器,暂停当前应用并切换到下一个
            set_next_trigger();
            suspend_current_and_run_next();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    cx
}
