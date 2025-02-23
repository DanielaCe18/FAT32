#![cfg(test)]
extern crate std;

use crate::filesystem::{FatFileSystem, StorageDevice};
use crate::directory::cluster::Cluster;
use crate::directory::table::FatValue;
use crate::directory::dir_entry::{DirectoryEntry, DirectoryIterator};
use crate::process::Process;
use crate::scheduler::SCHEDULER;
use crate::syscall::*;
use spin::Mutex;
use std::vec::Vec;
use crate::directory::attribute::Attributes;
use crate::directory::name::ShortFileName;
use crate::directory::datetime::FatDateTime;
use crate::directory::offset_iter::ClusterOffsetIter;

// Mock storage device for testing purposes.
struct MockStorage {
    data: Mutex<Vec<u8>>, // Underlying storage represented as a vector of bytes.
}

impl MockStorage {
    /// Creates a new mock storage with the specified size.
    pub fn new(size: usize) -> Self {
        Self {
            data: Mutex::new(vec![0; size]),
        }
    }
}

// Implementing the StorageDevice trait for MockStorage.
impl StorageDevice for MockStorage {
    /// Reads data from the mock storage into the provided buffer.
    fn read(&self, offset: u64, buffer: &mut [u8]) -> Result<(), ()> {
        let data = self.data.lock();
        let offset = offset as usize;
        if offset + buffer.len() > data.len() {
            return Err(());
        }
        buffer.copy_from_slice(&data[offset..offset + buffer.len()]);
        Ok(())
    }

    /// Writes data from the buffer into the mock storage.
    fn write(&self, offset: u64, buffer: &[u8]) -> Result<(), ()> {
        let mut data = self.data.lock();
        let offset = offset as usize;
        if offset + buffer.len() > data.len() {
            return Err(());
        }
        data[offset..offset + buffer.len()].copy_from_slice(buffer);
        Ok(())
    }
}

/// Test: Verify that the filesystem initializes with the correct cluster size.
#[test]
fn test_filesystem_initialization() {
    let mock_storage = MockStorage::new(1024 * 1024);
    let fs = FatFileSystem::new(mock_storage, 0, 4096);

    assert_eq!(fs.cluster_size, 4096);
}

/// Test: Ensure that a cluster can be allocated in the filesystem.
#[test]
fn test_cluster_allocation() {
    let mock_storage = MockStorage::new(1024 * 1024);
    let fs = FatFileSystem::new(mock_storage, 0, 4096);

    let cluster = fs.allocate_cluster();
    assert!(cluster.is_some());
}

/// Test: Check conversion of FAT values to corresponding enum variants.
#[test]
fn test_fat_value_conversion() {
    let data_cluster = FatValue::Data(1234);
    assert_eq!(match data_cluster { FatValue::Data(val) => val, _ => 0 }, 1234);

    let end_of_chain = FatValue::EndOfChain;
    assert!(matches!(end_of_chain, FatValue::EndOfChain));
}

/// Test: Verify directory entry creation with expected attributes.
#[test]
fn test_directory_entry_creation() {
    let entry = DirectoryEntry::new("TESTFILE.TXT", Cluster(5), 1024, 0x20);

    assert_eq!(entry.file_name(), "TESTFILE.TXT");
    assert_eq!(entry.start_cluster.0, 5);
    assert_eq!(entry.file_size, 1024);
    assert!(entry.is_file());
    assert!(!entry.is_directory());
}

/// Test: Ensure DirectoryIterator correctly iterates through directory entries.
#[test]
fn test_directory_iterator() {
    let mock_storage = MockStorage::new(1024 * 1024);
    let fs = FatFileSystem::new(mock_storage, 0, 4096);

    let cluster = Cluster(2);
    let entry1 = DirectoryEntry::new("FILE1.TXT", cluster, 512, 0x20);
    let entry2 = DirectoryEntry::new("DIR1", cluster, 0, 0x10);

    // Write mock directory entries into storage.
    let entry1_bytes = unsafe { core::mem::transmute::<DirectoryEntry, [u8; 32]>(entry1) };
    let entry2_bytes = unsafe { core::mem::transmute::<DirectoryEntry, [u8; 32]>(entry2) };

    fs.storage_device.lock().write(0, &entry1_bytes).unwrap();
    fs.storage_device.lock().write(32, &entry2_bytes).unwrap();

    let mut dir_iter = DirectoryIterator::new(&fs, cluster);
    let first_entry = dir_iter.next().unwrap();
    let second_entry = dir_iter.next().unwrap();

    assert_eq!(first_entry.file_name(), "FILE1.TXT");
    assert_eq!(second_entry.file_name(), "DIR1");
}

/// Test: Ensure the slab allocator correctly allocates and reuses memory.
#[test]
fn test_slab_allocator() {
    use crate::slab::{Slab, StaticMemoryPool};
    use core::mem::MaybeUninit;

    const POOL_SIZE: usize = 1024;
    static MEMORY_POOL: StaticMemoryPool<POOL_SIZE> = StaticMemoryPool::new();

    unsafe {
        let slab = Slab::new(MEMORY_POOL.as_mut_ptr(), POOL_SIZE, 16);

        // Allocate two objects.
        let ptr1 = slab.alloc().expect("Allocation failed");
        let ptr2 = slab.alloc().expect("Allocation failed");

        assert!(!ptr1.is_null());
        assert!(!ptr2.is_null());
        assert_ne!(ptr1, ptr2);

        // Free the first object and reallocate.
        slab.free(ptr1);
        let ptr3 = slab.alloc().expect("Reallocation failed");

        // Ensure the memory was reused.
        assert_eq!(ptr1, ptr3);
    }
}

/// Test: Verify virtual to physical address conversion.
#[test]
fn test_virt_to_phys() {
    let virt_addr = x86_64::VirtAddr::new(0x1000);
    if let Some(phys_addr) = crate::memory::virt_to_phys(virt_addr) {
        println!("Physical address: {:?}", phys_addr);
    } else {
        println!("Failed to convert virtual address to physical.");
    }
}

/// Test: Check global allocator's ability to allocate memory for a vector.
#[test]
fn test_global_allocator() {
    use alloc::vec::Vec;

    let mut vec: Vec<u8> = Vec::new();
    for i in 0..100 {
        vec.push(i);
    }

    assert_eq!(vec.len(), 100);
    assert_eq!(vec[0], 0);
    assert_eq!(vec[99], 99);
}

/// Test: Ensure failed allocation when the pool size is insufficient.
#[test]
fn test_failed_allocation() {
    use crate::slab::{Slab, StaticMemoryPool};
    use core::mem::MaybeUninit;

    const SMALL_POOL: usize = 128;
    static MEMORY_POOL: StaticMemoryPool<SMALL_POOL> = StaticMemoryPool::new();

    unsafe {
        let slab = Slab::new(MEMORY_POOL.as_mut_ptr(), SMALL_POOL, 64);

        let ptr1 = slab.alloc().expect("First allocation failed");
        let ptr2 = slab.alloc();

        // Second allocation should fail due to limited pool size.
        assert!(ptr2.is_none());

        // Free and reallocate.
        slab.free(ptr1);
        let ptr3 = slab.alloc().expect("Reallocation failed");

        assert_eq!(ptr1, ptr3);
    }
}

/// Test: Verify successful memory allocation and deallocation.
#[test]
fn test_memory_allocation() {
    use crate::memory::{allocate, deallocate};

    let size = 256;
    let ptr = allocate(size);
    assert!(!ptr.is_null(), "Allocation failed");

    unsafe {
        // Write known value into the allocated memory.
        core::ptr::write_bytes(ptr, 0xAA, size);
    }

    // Deallocate the memory.
    deallocate(ptr, size);
}

/// Test: Ensure round-robin scheduling of processes.
#[test]
fn test_scheduler_round_robin() {
    let mut scheduler = SCHEDULER.lock();

    // Create test processes.
    let process1 = Process::new("Process 1");
    let process2 = Process::new("Process 2");

    // Add processes to the scheduler.
    scheduler.add_process(process1);
    scheduler.add_process(process2);

    // Run the scheduler and complete each process.
    if let Some(mut proc) = scheduler.next_process() {
        proc.process.run();
        scheduler.complete_process(proc.process);
    }

    if let Some(mut proc) = scheduler.next_process() {
        proc.process.run();
        scheduler.complete_process(proc.process);
    }
}
