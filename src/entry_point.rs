extern crate linked_list_allocator;

use self::linked_list_allocator::Heap;
use alloc::allocator::Alloc;
use alloc::allocator::AllocErr;
use alloc::allocator::Layout;
use core::mem;
use core::ptr;
use syscalls;

const HEAP_SIZE: usize = 0x400;

// None-threaded heap wrapper based on `r9` register instead of global variable
pub(crate) struct TockAllocator;

impl TockAllocator {
    unsafe fn heap(&self) -> &mut Heap {
        let heap: *mut Heap;
        asm!("mov $0, r9" : "=r"(heap) : : : "volatile");
        &mut *heap
    }

    /// Initializes an empty heap
    ///
    /// # Unsafety
    ///
    /// This function must be called at most once. The memory between [`heap_location`] and [`heap_top`] must not overlap with any other memory section.
    #[inline(never)]
    unsafe fn init(&mut self, heap_bottom: usize, heap_top: usize) {
        asm!("mov r9, $0" : : "r"(heap_bottom) : : "volatile");

        let effective_heap_bottom = heap_bottom + mem::size_of::<Heap>();

        let heap = heap_bottom as *mut Heap;
        *heap = Heap::new(effective_heap_bottom, heap_top - effective_heap_bottom);
    }
}

unsafe impl<'a> Alloc for &'a TockAllocator {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        self.heap().alloc(layout)
    }

    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        self.heap().dealloc(ptr, layout)
    }
}

/// Tock programs' entry point
#[doc(hidden)]
#[no_mangle]
#[naked]
#[link_section = ".start"]
pub unsafe extern "C" fn _start(
    text_start: usize,
    mem_start: usize,
    _memory_len: usize,
    _app_heap_break: usize,
) -> ! {
    extern "C" {
        // This function is created internally by`rustc`. See `src/lang_items.rs` for more details.
        fn main(argc: isize, argv: *const *const u8) -> isize;
    }

    let stack_size = *(text_start as *const usize).offset(9);
    let real_stack_top = mem_start + stack_size;
    // Set the effective stack top below the real stack stop and use the space in between for the heap
    let effective_stack_top = real_stack_top - HEAP_SIZE;
    TockAllocator.init(effective_stack_top, real_stack_top);

    asm!("mov sp, $0" : : "r"(effective_stack_top) : "memory" :  "volatile" );

    main(0, ptr::null());

    loop {
        syscalls::yieldk();
    }
}
