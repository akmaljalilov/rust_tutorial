#[test]
fn test1() {
    use std::alloc::{GlobalAlloc, System, Layout};

    struct MyAllocator;

    unsafe impl GlobalAlloc for MyAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            System.alloc(layout)
        }


        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            System.dealloc(ptr, layout);
        }
    }

    #[global_allocator]
    static GLOBAL: MyAllocator = MyAllocator;

    fn test() {
        let mut v = Vec::new();
        v.push(1);
        v.push(1);
    }
    test()
}