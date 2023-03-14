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

#[test]
pub fn test() {
    let _b = Box::new_in(1, MyAlloc);
}

fn rdtsc() -> u64 {
    unsafe { std::arch::x86_64::_rdtsc() }
}

pub fn perf_test1() {
    let mut times = [0; 10];
    for i in 0..times.len() {
        let t0 = rdtsc();
        {
            let _b = std::hint::black_box(Box::new_in(1, MyAlloc));
        }
        times[i] = rdtsc().wrapping_sub(t0);
    }
    println!("new_in: {:?} cycles", times);
}

pub fn perf_test2() {
    let mut times = [0; 10];
    for i in 0..times.len() {
        let t0 = rdtsc();
        {
            let _b = std::hint::black_box(Box::new(1));
        }
        times[i] = rdtsc().wrapping_sub(t0);
    }
    println!("new: {:?} cycles", times);
}

fn main() {
    perf_test1();
    perf_test2();
}
