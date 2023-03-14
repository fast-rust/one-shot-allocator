#![feature(allocator_api)]
use std::{
    alloc::{AllocError, Allocator, Layout},
    ptr::NonNull,
    sync::atomic::{AtomicUsize, Ordering},
};

pub struct MyAlloc;

static mut MEMORY: [u8; 0x100000] = [0; 0x100000];
static PTR: AtomicUsize = AtomicUsize::new(0);
const MIN_ALIGNMENT: usize = 16;

unsafe impl Allocator for MyAlloc {
    #[inline]
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let align = layout.align();
        let size = layout.size();
        if align > MIN_ALIGNMENT {
            Err(AllocError)
        } else {
            unsafe {
                let aligned_size = (size + MIN_ALIGNMENT - 1) & !(MIN_ALIGNMENT - 1);
                let offset = PTR.fetch_add(aligned_size, Ordering::AcqRel);
                if offset + size <= MEMORY.len() {
                    let slice = std::slice::from_raw_parts(
                            MEMORY.as_ptr().offset(offset as isize),
                            size);
                    Ok(NonNull::from(slice))
                } else {
                    Err(AllocError)
                }
            }
        }
    }

    #[inline]
    unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
}

pub fn test() {
    let _b = Box::new_in(1, MyAlloc);
}

