#![cfg_attr(not(feature = "std"), no_std)]

use core::alloc::{GlobalAlloc, Layout};
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicU8, Ordering};
use core::ptr::null_mut;
use spin::Mutex;
use core::marker::PhantomData;

#[cfg(feature = "debug")]
use core::fmt::{self, Write};

#[cfg(feature = "debug")]
use spin::Mutex as SpinMutex;

fn atomic_bts(bitmap: &[AtomicU8], index: usize) -> Option<bool> {
    let byte_index = index / 8;
    let bit_index = (index % 8) as u8;
    bitmap.get(byte_index).map(|byte| {
        let prev = byte.fetch_or(1 << bit_index, Ordering::Relaxed);
        (prev & (1 << bit_index)) != 0
    })
}

fn atomic_btc(bitmap: &[AtomicU8], index: usize) -> Option<bool> {
    let byte_index = index / 8;
    let bit_index = (index % 8) as u8;
    bitmap.get(byte_index).map(|byte| {
        let prev = byte.fetch_and(!(1 << bit_index), Ordering::Relaxed);
        (prev & (1 << bit_index)) != 0
    })
}

fn div_ceil(num: usize, den: usize) -> usize {
    (num + den - 1) / den
}

pub struct Slab {
    bitmap: &'static mut [AtomicU8],
    data: *mut u8,
    object_size: usize,
    num_objects: usize,
    _marker: PhantomData<*mut u8>,
}

unsafe impl Send for Slab {}
unsafe impl Sync for Slab {}

impl Slab {
    pub unsafe fn new(mem: *mut MaybeUninit<u8>, size: usize, object_size: usize) -> Self {
        let num_objects = size / object_size;
        let bitmap_size = div_ceil(num_objects, 8);

        assert!(size >= bitmap_size + num_objects * object_size);

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

    pub fn alloc(&self) -> Option<*mut u8> {
        for i in 0..self.num_objects {
            if let Some(false) = atomic_bts(self.bitmap, i) {
                let ptr = unsafe { self.data.add(i * self.object_size) };
                return Some(ptr);
            }
        }
        None
    }

    pub unsafe fn free(&self, ptr: *mut u8) {
        let offset = ptr.offset_from(self.data) as usize;
        assert!(offset % self.object_size == 0);
        let index = offset / self.object_size;
        assert!(index < self.num_objects);
        atomic_btc(self.bitmap, index);
    }
}

pub struct StaticMemoryPool<const SIZE: usize> {
    pool: MaybeUninit<[MaybeUninit<u8>; SIZE]>,
}

impl<const SIZE: usize> StaticMemoryPool<SIZE> {
    pub const fn new() -> Self {
        Self {
            pool: MaybeUninit::uninit(),
        }
    }

    pub fn as_mut_ptr(&self) -> *mut MaybeUninit<u8> {
        self.pool.as_ptr() as *mut MaybeUninit<u8>
    }

    pub fn len(&self) -> usize {
        SIZE
    }
}

pub struct GlobalAllocator;

static GLOBAL_POOLS: Mutex<Option<[Option<Slab>; 2]>> = Mutex::new(None);

static POOL_1: StaticMemoryPool<1024> = StaticMemoryPool::new();
static POOL_2: StaticMemoryPool<2048> = StaticMemoryPool::new();

#[cfg(feature = "debug")]
static DEBUG_LOG: SpinMutex<Option<DebugLogger>> = SpinMutex::new(None);

#[cfg(feature = "debug")]
struct DebugLogger;

#[cfg(feature = "debug")]
impl DebugLogger {
    fn log(&self, message: &str) {
        for byte in message.bytes() {
            unsafe {
                self.write_byte(byte);
            }
        }
    }

    fn get_logger(&self) -> impl Write {
        struct DebugWriter;

        impl Write for DebugWriter {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                let logger = DebugLogger;
                logger.log(s);
                Ok(())
            }
        }

        DebugWriter
    }

    unsafe fn write_byte(&self, byte: u8) {
        const DEBUG_OUTPUT: *mut u8 = 0x1000 as *mut u8;
        core::ptr::write_volatile(DEBUG_OUTPUT, byte);
    }
}

unsafe impl GlobalAlloc for GlobalAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        #[cfg(feature = "debug")]
        if let Some(logger) = DEBUG_LOG.lock().as_ref() {
            logger.log(&format!("Allocating {} bytes\n", layout.size()));
        }

        let size = layout.size();
        let mut pools = GLOBAL_POOLS.lock();

        if pools.is_none() {
            *pools = Some([
                Some(Slab::new(
                    POOL_1.as_mut_ptr(),
                    POOL_1.len(),
                    8,
                )),
                Some(Slab::new(
                    POOL_2.as_mut_ptr(),
                    POOL_2.len(),
                    16,
                )),
            ]);
        }

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

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        #[cfg(feature = "debug")]
        if let Some(logger) = DEBUG_LOG.lock().as_ref() {
            logger.log(&format!("Deallocating {} bytes\n", layout.size()));
        }

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

#[global_allocator]
static ALLOCATOR: GlobalAllocator = GlobalAllocator;

