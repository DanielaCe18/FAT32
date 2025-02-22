#![no_std]

use core::ptr::NonNull;
use x86_64::{PhysAddr, VirtAddr};

/// Converts a virtual address to a physical address.
pub fn virt_to_phys(virt_addr: VirtAddr) -> Option<PhysAddr> {
    let page_table = x86_64::registers::control::Cr3::read().0.start_address();

    let mut addr = virt_addr;
    for level in (1..=3).rev() {
        let index = addr.p4_index();

        // Calculate the table entry address.
        let entry_addr = page_table + (index as u64 * 8);
        let entry = unsafe { *(entry_addr.as_u64() as *const u64) };

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

