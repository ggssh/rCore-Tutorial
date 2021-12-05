use crate::{
    task::{exit_current_and_run_next, suspend_current_and_run_next},
    timer::get_time_ms,
};

pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    // run_next_app()
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!")
}

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}
// pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
//     let us = get_time_us();
//     unsafe {
//         *ts = TimeVal {
//             sec: us / 1_000_000,
//             usec: us % 1_000_000,
//         };
//     }
//     // get_time_us() as isize
//     0
// }
pub fn sys_get_time() -> isize {
    get_time_ms() as isize
}
