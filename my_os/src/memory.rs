use x86_64::{PhysAddr, VirtAddr};
use x86_64::registers::control::Cr3;
use core::alloc::{GlobalAlloc, Layout};
use crate::ALLOCATOR;

// Convert virtual address to physical address
pub fn virt_to_phys(virt_addr: VirtAddr) -> Option<PhysAddr> {
    let (frame, _) = Cr3::read();
    let page_table = frame.start_address().as_u64();

    let mut addr = virt_addr;

    for level in (1..=3).rev() {
        let index = addr.p4_index();
        let entry_addr = page_table + (u64::from(index) * 8);
        let entry = unsafe { *(entry_addr as *const u64) };

        if entry & 1 == 0 {
            return None;
        }

        if level == 1 {
            let phys_base = entry & 0x000fffff_fffff000;
            let offset = virt_addr.as_u64() & 0xfff;
            return Some(PhysAddr::new(phys_base | offset));
        }

        addr = VirtAddr::new(entry & 0x000fffff_fffff000);
    }

    None
}

// Allocate memory
pub fn allocate(size: usize) -> *mut u8 {
    unsafe { ALLOCATOR.alloc(Layout::from_size_align(size, 8).unwrap()) }
}

// Deallocate memory
pub fn deallocate(ptr: *mut u8, size: usize) {
    unsafe { ALLOCATOR.dealloc(ptr, Layout::from_size_align(size, 8).unwrap()) }
}

