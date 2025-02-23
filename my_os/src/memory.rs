use x86_64::{PhysAddr, VirtAddr};
use x86_64::registers::control::Cr3;
use core::alloc::{GlobalAlloc, Layout};
use crate::ALLOCATOR;

/// Converts a virtual address to its corresponding physical address.
pub fn virt_to_phys(virt_addr: VirtAddr) -> Option<PhysAddr> {
    let (frame, _) = Cr3::read(); // Read the CR3 register to get the page table frame.
    let page_table = frame.start_address().as_u64();

    let mut addr = virt_addr;

    // Traverse page table levels (P4, P3, P2).
    for level in (1..=3).rev() {
        let index = addr.p4_index(); // Get the page table index for the current level.
        let entry_addr = page_table + (u64::from(index) * 8); // Calculate entry address.
        let entry = unsafe { *(entry_addr as *const u64) };   // Read the page table entry.

        if entry & 1 == 0 {
            return None; // Entry not present.
        }

        if level == 1 {
            let phys_base = entry & 0x000fffff_fffff000; // Extract physical address.
            let offset = virt_addr.as_u64() & 0xfff;     // Get offset within the page.
            return Some(PhysAddr::new(phys_base | offset));
        }

        // Move to the next level by setting addr to the next table's base address.
        addr = VirtAddr::new(entry & 0x000fffff_fffff000);
    }

    None // Return `None` if translation fails.
}

/// Allocates memory of the given size.
pub fn allocate(size: usize) -> *mut u8 {
    unsafe { ALLOCATOR.alloc(Layout::from_size_align(size, 8).unwrap()) }
}

/// Deallocates previously allocated memory.
pub fn deallocate(ptr: *mut u8, size: usize) {
    unsafe { ALLOCATOR.dealloc(ptr, Layout::from_size_align(size, 8).unwrap()) }
}
