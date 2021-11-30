use riscv::register::sstatus::{self, Sstatus, SPP};

pub struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: Sstatus, // SPP 等字段给出 Trap 发生之前 CPU 处在哪个特权级（S/U）等信息
    pub sepc: usize,      // 记录trap发生前执行的最后一条指令的地址
}

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }
    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User); // 将sstatus寄存器的SPP字段设置为User
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry, // 将spec设置为应用程序入口点entry
        };
        cx.set_sp(sp);
        cx
    }
}
