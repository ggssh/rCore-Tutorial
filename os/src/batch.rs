use core::ops::Add;

use crate::sync::UPSafeCell;
use lazy_static::*;

const MAX_APP_NUM: usize = 16;
const APP_BASE_ADDRESS: usize = 0x80400000;
const APP_SIZE_LIMIT: usize = 0x20000;

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
        println!("[rCore kernel] num_app = {}", self.num_app);
        for i in 0..self.num_app {
            println!(
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
        println!("[rCore kernel] Loading app_{}", app_id);
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
    unsafe{
        app_manager.load_app(current_app);
    }
    app_manager.move_to_next_app();
    drop(app_manager);
    // before this we have to drop local variables related to resources manually and release the resources
    extern "C" {
        fn __restore(cx_addr:usize);
    }
    // unsafe{
    //     __restore()
    // }
    panic!("Unreachable in batch::run_current_app!");
}
