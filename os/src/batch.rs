use crate::sync::UPSafeCell;
use crate::trap::TrapContext;
use core::ops::Add;
use lazy_static::*;

const MAX_APP_NUM: usize = 16;
const APP_BASE_ADDRESS: usize = 0x80400000;
const APP_SIZE_LIMIT: usize = 0x20000;
// 内核栈和用户栈的大小分别为8kib,两者是以全局变量的形式实例化在批处理操作系统的.bss段中
const USER_STACK_SIZE: usize = 4096 * 2;
const KERNEL_STACK_SIZE: usize = 4096 * 2;

#[repr(align(4096))]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

impl KernelStack {
    // 获取栈顶地址
    fn get_sp(&self) -> usize {
        // 数组的结尾地址
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }

    /// 返回值是内核栈压入Trap上下文之后的栈顶
    pub fn push_context(&self, cx: TrapContext) -> &'static mut TrapContext {
        let cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *cx_ptr = cx;
        }
        unsafe { cx_ptr.as_mut().unwrap() }
    }
}

#[repr(align(4096))]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

impl UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}

static KERNEL_STACK: KernelStack = KernelStack {
    data: [0; KERNEL_STACK_SIZE],
};
static USER_STACK: UserStack = UserStack {
    data: [0; USER_STACK_SIZE],
};

// 应用管理器AppManager
// 能够找到并加载应用程序二进制码
// 主要功能:
// 保存应用数量和各自的位置信息,以及当前执行到第几个应用了
// 根据应用程序位置信息,初始化好应用所需内存空间,并加载应用执行
struct AppManager {
    num_app: usize,
    current_app: usize,
    app_start: [usize; MAX_APP_NUM + 1],
}

lazy_static! {
    // 只有APP_MANAGER第一次被用到的时候才会进行实际的初始化工作
    static ref APP_MANAGER: UPSafeCell<AppManager> = unsafe {
        UPSafeCell::new({
            extern "C" {
                // 找到link_app.asm中的符号_num_app
                fn _num_app();
            }
            let num_app_ptr = _num_app as usize as *const usize;
            let num_app = num_app_ptr.read_volatile();
            let mut app_start: [usize; MAX_APP_NUM + 1] = [0; MAX_APP_NUM + 1];
            let app_start_raw: &[usize] =
                core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1);
            app_start[..=num_app].copy_from_slice(app_start_raw);
            AppManager {
                num_app,
                current_app: 0,
                app_start,
            }
        })
    };
}

impl AppManager {
    pub fn print_app_info(&self) {
        info!("[rCore kernel] num_app = {}", self.num_app);
        for i in 0..self.num_app {
            info!(
                "[rCore kernel] app_{} [{:#x}, {:#x})",
                &i,
                self.app_start[i],
                self.app_start[i + 1]
            );
        }
    }

    // 将app_id对应的程序二进制镜像加载到内存以base_address开始的位置
    unsafe fn load_app(&self, app_id: usize) {
        if app_id >= self.num_app {
            panic!("All applications completed!\n There is no application need to run.");
        }
        info!("[rCore kernel] Loading app_{}", app_id);
        // clear icache
        asm!("fence.i");
        // 清空内存(大小为APP_SIZE_LIMIT)
        core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, APP_SIZE_LIMIT).fill(0);
        let app_src = core::slice::from_raw_parts(
            self.app_start[app_id] as *const u8,
            self.app_start[app_id + 1] - self.app_start[app_id],
        );
        // 本质上是将操作系统数据段的一部分数据(实际上是应用程序)复制到了一个可以执行代码的内存区域
        let app_dst = core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, app_src.len());
        app_dst.copy_from_slice(app_src);
    }

    pub fn get_current_app(&self) -> usize {
        self.current_app
    }

    pub fn move_to_next_app(&mut self) {
        self.current_app += 1;
    }
}

// 向外暴露的接口
pub fn init() {
    print_app_info();
}

pub fn print_app_info() {
    APP_MANAGER.exclusive_access().print_app_info();
}

pub fn run_next_app() -> ! {
    let mut app_manager = APP_MANAGER.exclusive_access();
    let current_app = app_manager.get_current_app();
    unsafe {
        app_manager.load_app(current_app);
    }
    app_manager.move_to_next_app();
    drop(app_manager);
    // before this we have to drop local variables related to resources manually and release the resources
    extern "C" {
        fn __restore(cx_addr: usize);
    }
    unsafe {
        // 在内核栈压入一个Trap上下文
        // push_context的返回值为内核栈栈顶,作为__restore的参数,在__restore中完成 sp<-a0
        // 使sp仍然指向内核栈的栈顶
        __restore(KERNEL_STACK.push_context(TrapContext::app_init_context(
            APP_BASE_ADDRESS,
            USER_STACK.get_sp(),
        )) as *const _ as usize);
    }
    panic!("Unreachable in batch::run_current_app!");
}
