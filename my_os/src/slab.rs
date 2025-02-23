#![cfg_attr(not(feature = "std"), no_std)]

use core::alloc::{GlobalAlloc, Layout};
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicU8, Ordering};
use core::ptr::null_mut;
use spin::Mutex;
use core::marker::PhantomData;

// Atomically sets a bit in the bitmap and returns its previous state.
fn atomic_bts(bitmap: &[AtomicU8], index: usize) -> Option<bool> {
    let byte_index = index / 8;
    let bit_index = (index % 8) as u8;
    bitmap.get(byte_index).map(|byte| {
        let prev = byte.fetch_or(1 << bit_index, Ordering::Relaxed);
        (prev & (1 << bit_index)) != 0
    })
}

// Atomically clears a bit in the bitmap and returns its previous state.
fn atomic_btc(bitmap: &[AtomicU8], index: usize) -> Option<bool> {
    let byte_index = index / 8;
    let bit_index = (index % 8) as u8;
    bitmap.get(byte_index).map(|byte| {
        let prev = byte.fetch_and(!(1 << bit_index), Ordering::Relaxed);
        (prev & (1 << bit_index)) != 0
    })
}

// Calculates the ceiling of integer division.
fn div_ceil(num: usize, den: usize) -> usize {
    (num + den - 1) / den
}

// Slab allocator for managing fixed-size object allocations.
pub struct Slab {
    bitmap: &'static mut [AtomicU8], // Bitmap tracking allocated objects.
    data: *mut u8,                   // Memory region for object storage.
    object_size: usize,              // Size of each allocated object.
    num_objects: usize,              // Total number of objects in the slab.
    _marker: PhantomData<*mut u8>,
}

unsafe impl Send for Slab {}
unsafe impl Sync for Slab {}

impl Slab {
    // Creates a new slab with a bitmap and object storage.
    pub unsafe fn new(mem: *mut MaybeUninit<u8>, size: usize, object_size: usize) -> Self {
        let num_objects = size / object_size;
        let bitmap_size = div_ceil(num_objects, 8);

        // Ensure memory is sufficient for bitmap and objects.
        assert!(size >= bitmap_size + num_objects * object_size);

        // Initialize bitmap to mark all objects as free.
        let bitmap = {
            let raw_bitmap = mem.add(size - bitmap_size) as *mut AtomicU8;
            for i in 0..bitmap_size {
                (*raw_bitmap.add(i)).store(0, Ordering::Relaxed);
            }
            core::slice::from_raw_parts_mut(raw_bitmap, bitmap_size)
        };

        Slab {
            bitmap,
            data: mem as *mut u8,
            object_size,
            num_objects,
            _marker: PhantomData,
        }
    }

    // Allocates an object, returning a pointer if available.
    pub fn alloc(&self) -> Option<*mut u8> {
        for i in 0..self.num_objects {
            if let Some(false) = atomic_bts(self.bitmap, i) {
                let ptr = unsafe { self.data.add(i * self.object_size) };
                return Some(ptr);
            }
        }
        None
    }

    // Frees an allocated object by clearing its bitmap bit.
    pub unsafe fn free(&self, ptr: *mut u8) {
        let offset = ptr.offset_from(self.data) as usize;
        assert!(offset % self.object_size == 0);
        let index = offset / self.object_size;
        assert!(index < self.num_objects);
        atomic_btc(self.bitmap, index);
    }
}

// Static memory pool for preallocated storage.
pub struct StaticMemoryPool<const SIZE: usize> {
    pool: MaybeUninit<[MaybeUninit<u8>; SIZE]>,
}

impl<const SIZE: usize> StaticMemoryPool<SIZE> {
    pub const fn new() -> Self {
        Self {
            pool: MaybeUninit::uninit(),
        }
    }

    // Returns a mutable pointer to the memory pool.
    pub fn as_mut_ptr(&self) -> *mut MaybeUninit<u8> {
        self.pool.as_ptr() as *mut MaybeUninit<u8>
    }

    pub fn len(&self) -> usize {
        SIZE
    }
}

// Global allocator implementing the GlobalAlloc trait.
pub struct GlobalAllocator;

// Global pool mutex to manage multiple slabs.
static GLOBAL_POOLS: Mutex<Option<[Option<Slab>; 2]>> = Mutex::new(None);

// Two static memory pools for different object sizes.
static POOL_1: StaticMemoryPool<1024> = StaticMemoryPool::new();
static POOL_2: StaticMemoryPool<2048> = StaticMemoryPool::new();

unsafe impl GlobalAlloc for GlobalAllocator {
    // Allocates memory by selecting an appropriate slab.
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let mut pools = GLOBAL_POOLS.lock();

        // Initialize slabs if not already done.
        if pools.is_none() {
            *pools = Some([
                Some(Slab::new(
                    POOL_1.as_mut_ptr(),
                    POOL_1.len(),
                    8, // Object size for pool 1.
                )),
                Some(Slab::new(
                    POOL_2.as_mut_ptr(),
                    POOL_2.len(),
                    16, // Object size for pool 2.
                )),
            ]);
        }

        // Find a suitable slab for allocation.
        if let Some(pools) = pools.as_mut() {
            for slab in pools.iter_mut().flatten() {
                if slab.object_size >= size {
                    if let Some(ptr) = slab.alloc() {
                        return ptr;
                    }
                }
            }
        }
        null_mut()
    }

    // Deallocates memory by finding the corresponding slab.
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let size = layout.size();
        let pools = GLOBAL_POOLS.lock();

        if let Some(pools) = pools.as_ref() {
            for slab in pools.iter().flatten() {
                if slab.object_size >= size {
                    slab.free(ptr);
                    return;
                }
            }
        }
    }
}
