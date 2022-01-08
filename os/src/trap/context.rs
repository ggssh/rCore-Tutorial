use riscv::register::sstatus::{self, Sstatus, SPP};

// 将Trap上下文保存在应用地址空间的一个虚拟页面中，而不是切换到内核空间
#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32],     // 32个通用寄存器
    pub sstatus: Sstatus,   // SPP 等字段给出 Trap 发生之前 CPU 处在哪个特权级（S/U）等信息
    pub sepc: usize,        // 记录trap发生前执行的最后一条指令的地址
    pub kernel_satp: usize, // 内核地址空间的token，即内核页表的起始物理地址
    pub kernel_sp: usize,   // 当前应用在内核地址空间中的内核栈栈顶的虚拟地址
    pub trap_handler: usize, // 内核中trap handler入口点的虚拟地址
                            // 在应用初始化时由内核写入应用地址空间的TrapContext的相应位置，之后不再修改
}

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }
    pub fn app_init_context(
        entry: usize,
        sp: usize,
        kernel_satp: usize,
        kernel_sp: usize,
        trap_handler: usize,
    ) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User); // 将sstatus寄存器的SPP字段设置为User
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry, // 将spec设置为应用程序入口点entry
            kernel_satp,
            kernel_sp,
            trap_handler,
        };
        cx.set_sp(sp);
        cx
    }
}
