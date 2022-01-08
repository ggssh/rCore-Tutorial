pub mod address;
pub mod frame_allocator;
pub mod heap_allocator;
pub mod memory_set;
pub mod page_table;

pub use memory_set::KERNEL_SPACE;
pub use memory_set::remap_test;

pub fn init() {
    // 初始化全局动态内存分配器
    heap_allocator::init_heap();
    // 初始化物理页帧管理器
    frame_allocator::init_frame_allocator();
    info!("into activating kernel_sapce");
    // 创建内核地址空间并让CPU开启分页模式
    KERNEL_SPACE.exclusive_access().activate();
}

pub fn mm_test_heap() {
    heap_allocator::heap_test();
}
