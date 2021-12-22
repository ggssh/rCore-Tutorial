mod heap_allocator;

pub fn init(){
    heap_allocator::init_heap();
}

pub fn mm_test_heap(){
    heap_allocator::heap_test();
}