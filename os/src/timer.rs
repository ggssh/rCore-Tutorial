use riscv::register::time;

// 相当于kernel内置的时钟
pub fn get_time() -> usize {
    time::read()
}

use crate::{config::CLOCK_FREQ, sbi::set_timer};
const TICKS_PER_SEC: usize = 100;

pub fn set_next_trigger() {
    // CLOCK_FREQ / TICKS_PER_SEC 10ms mtime的增量,即下次时钟中断的计数器增量值
    set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
}

const MICRO_PER_SEC: usize = 1_000_000;
pub fn get_time_ms() -> usize {
    time::read() / (CLOCK_FREQ / MICRO_PER_SEC)
}
