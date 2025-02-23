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

// Mock storage device for testing
struct MockStorage {
    data: Mutex<Vec<u8>>, // Mutex to protect access to the underlying data
}

impl MockStorage {
    // Create a new mock storage with the given size
    pub fn new(size: usize) -> Self {
        Self {
            data: Mutex::new(vec![0; size]),
        }
    }
}

impl StorageDevice for MockStorage {
    // Read data from the mock storage
    fn read(&self, offset: u64, buffer: &mut [u8]) -> Result<(), ()> {
        let data = self.data.lock();
        let offset = offset as usize;
        if offset + buffer.len() > data.len() {
            return Err(());
        }
        buffer.copy_from_slice(&data[offset..offset + buffer.len()]);
        Ok(())
    }

    // Write data to the mock storage
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

// Test filesystem initialization
#[test]
fn test_filesystem_initialization() {
    let mock_storage = MockStorage::new(1024 * 1024); // Create mock storage of 1MB
    let fs = FatFileSystem::new(mock_storage, 0, 4096); // Initialize filesystem with 4KB cluster size

    assert_eq!(fs.cluster_size, 4096); // Verify the cluster size
}

// Test cluster allocation
#[test]
fn test_cluster_allocation() {
    let mock_storage = MockStorage::new(1024 * 1024);
    let fs = FatFileSystem::new(mock_storage, 0, 4096);

    let cluster = fs.allocate_cluster(); // Allocate a cluster
    assert!(cluster.is_some()); // Ensure allocation was successful
}

// Test FAT value conversion
#[test]
fn test_fat_value_conversion() {
    let data_cluster = FatValue::Data(1234);
    assert_eq!(match data_cluster { FatValue::Data(val) => val, _ => 0 }, 1234); // Verify conversion from data cluster

    let end_of_chain = FatValue::EndOfChain;
    assert!(matches!(end_of_chain, FatValue::EndOfChain)); // Ensure end-of-chain value is correctly identified
}

// Test DirectoryEntry creation
#[test]
fn test_directory_entry_creation() {
    let entry = DirectoryEntry::new("TESTFILE.TXT", Cluster(5), 1024, 0x20); // Create a new directory entry

    assert_eq!(entry.file_name(), "TESTFILE.TXT"); // Check file name
    assert_eq!(entry.start_cluster.0, 5); // Verify start cluster
    assert_eq!(entry.file_size, 1024); // Verify file size
    assert!(entry.is_file()); // Ensure it is identified as a file
    assert!(!entry.is_directory()); // Ensure it is not identified as a directory
}

// Test DirectoryIterator functionality
#[test]
fn test_directory_iterator() {
    let mock_storage = MockStorage::new(1024 * 1024);
    let fs = FatFileSystem::new(mock_storage, 0, 4096);

    let cluster = Cluster(2);
    let entry1 = DirectoryEntry::new("FILE1.TXT", cluster, 512, 0x20);
    let entry2 = DirectoryEntry::new("DIR1", cluster, 0, 0x10);

    // Write mock entries into storage
    let entry1_bytes = unsafe { core::mem::transmute::<DirectoryEntry, [u8; 32]>(entry1) };
    let entry2_bytes = unsafe { core::mem::transmute::<DirectoryEntry, [u8; 32]>(entry2) };

    fs.storage_device.lock().write(0, &entry1_bytes).unwrap();
    fs.storage_device.lock().write(32, &entry2_bytes).unwrap();

    let mut dir_iter = DirectoryIterator::new(&fs, cluster); // Create directory iterator
    let first_entry = dir_iter.next().unwrap(); // Get first entry
    let second_entry = dir_iter.next().unwrap(); // Get second entry

    assert_eq!(first_entry.file_name(), "FILE1.TXT"); // Verify first entry
    assert_eq!(second_entry.file_name(), "DIR1"); // Verify second entry
}

// Test slab allocator
#[test]
fn test_slab_allocator() {
    use crate::slab::{Slab, StaticMemoryPool};
    use core::mem::MaybeUninit;

    const POOL_SIZE: usize = 1024;
    static MEMORY_POOL: StaticMemoryPool<POOL_SIZE> = StaticMemoryPool::new();

    unsafe {
        let slab = Slab::new(MEMORY_POOL.as_mut_ptr(), POOL_SIZE, 16); // Create a slab allocator

        let ptr1 = slab.alloc().expect("Allocation failed"); // Allocate first object
        let ptr2 = slab.alloc().expect("Allocation failed"); // Allocate second object

        assert!(!ptr1.is_null()); // Verify non-null pointer
        assert!(!ptr2.is_null());
        assert_ne!(ptr1, ptr2); // Ensure different allocations

        slab.free(ptr1); // Free first object
        let ptr3 = slab.alloc().expect("Reallocation failed"); // Reallocate

        assert_eq!(ptr1, ptr3); // Ensure memory was reused
    }
}

// Test virtual to physical address conversion
#[test]
fn test_virt_to_phys() {
    let virt_addr = x86_64::VirtAddr::new(0x1000);
    if let Some(phys_addr) = crate::memory::virt_to_phys(virt_addr) {
        println!("Physical address: {:?}", phys_addr); // Output physical address if conversion is successful
    } else {
        println!("Failed to convert virtual address to physical."); // Indicate failure
    }
}

// Test global allocator
#[test]
fn test_global_allocator() {
    use alloc::vec::Vec;

    let mut vec: Vec<u8> = Vec::new(); // Create vector
    for i in 0..100 {
        vec.push(i); // Fill vector with values
    }

    assert_eq!(vec.len(), 100); // Verify length
    assert_eq!(vec[0], 0); // Check first element
    assert_eq!(vec[99], 99); // Check last element
}

// Test failed allocation due to limited pool size
#[test]
fn test_failed_allocation() {
    use crate::slab::{Slab, StaticMemoryPool};
    use core::mem::MaybeUninit;

    const SMALL_POOL: usize = 128;
    static MEMORY_POOL: StaticMemoryPool<SMALL_POOL> = StaticMemoryPool::new();

    unsafe {
        let slab = Slab::new(MEMORY_POOL.as_mut_ptr(), SMALL_POOL, 64);

        let ptr1 = slab.alloc().expect("First allocation failed"); // First allocation should succeed
        let ptr2 = slab.alloc(); // Second allocation should fail

        assert!(ptr2.is_none()); // Ensure second allocation fails

        slab.free(ptr1); // Free first allocation
        let ptr3 = slab.alloc().expect("Reallocation failed"); // Reallocate

        assert_eq!(ptr1, ptr3); // Ensure reuse of freed memory
    }
}

// Test memory allocation and deallocation
#[test]
fn test_memory_allocation() {
    use crate::memory::{allocate, deallocate};

    let size = 256;
    let ptr = allocate(size); // Allocate 256 bytes
    assert!(!ptr.is_null(), "Allocation failed");

    unsafe {
        core::ptr::write_bytes(ptr, 0xAA, size); // Write pattern into memory
    }

    deallocate(ptr, size); // Deallocate memory
}

// Test round-robin scheduler
#[test]
fn test_scheduler_round_robin() {
    let mut scheduler = SCHEDULER.lock();

    let process1 = Process::new("Process 1"); // Create first process
    let process2 = Process::new("Process 2"); // Create second process

    scheduler.add_process(process1); // Add processes to scheduler
    scheduler.add_process(process2);

    if let Some(mut proc) = scheduler.next_process() {
        proc.process.run(); // Run first process
        scheduler.complete_process(proc.process);
    }

    if let Some(mut proc) = scheduler.next_process() {
        proc.process.run(); // Run second process
        scheduler.complete_process(proc.process);
    }
}

// Test memory allocation through syscall
#[test]
fn test_syscall_memory_allocation() {
    let size = 512;
    let ptr = syscall_alloc(size); // Allocate memory via syscall
    assert!(!ptr.is_null(), "Syscall allocation failed");

    unsafe { core::ptr::write_bytes(ptr, 0xBB, size) }; // Write pattern into allocated memory

    syscall_dealloc(ptr, size); // Deallocate memory
}

// Test process creation via syscall
#[test]
fn test_syscall_process_creation() {
    let process = syscall_create_process("TestSyscallProcess", 0x2000);
    assert_eq!(process.name, "TestSyscallProcess"); // Verify process name
    assert_eq!(process.state, crate::process::ProcessState::Ready); // Verify process state
}

// Test reading memory via syscall
#[test]
fn test_syscall_read_memory() {
    let size = 128;
    let ptr = syscall_alloc(size);
    unsafe { core::ptr::write_bytes(ptr, 0xCC, size) }; // Write pattern

    let read = syscall_read_mem(ptr, size); // Read memory
    assert!(read.is_some());
    assert_eq!(read.unwrap().len(), size); // Verify read size

    syscall_dealloc(ptr, size); // Deallocate memory
}

// Test process termination via syscall
#[test]
fn test_syscall_terminate_process() {
    let mut process = syscall_create_process("TermProcess", 0x3000);
    syscall_terminate_process(&mut process); // Terminate process
    assert_eq!(process.state, crate::process::ProcessState::Terminated); // Verify terminated state
}

// Test file attribute functionality
#[test]
fn test_attributes() {
    let attr = Attributes::new(Attributes::READ_ONLY | Attributes::HIDDEN);
    assert!(attr.is_read_only()); // Verify read-only attribute
    assert!(attr.is_hidden()); // Verify hidden attribute
    assert!(!attr.is_system()); // Ensure not marked as system
}

// Test short file name creation
#[test]
fn test_short_filename() {
    let filename = ShortFileName::new("example", "txt");
    assert_eq!(filename.as_str(), "EXAMPLE.TXT"); // Verify uppercase formatting
}

// Test FAT date-time conversion
#[test]
fn test_fat_datetime() {
    let datetime = FatDateTime::new(2024, 6, 1, 12, 0, 0);
    let timestamp = datetime.to_unix_timestamp(); // Convert to Unix timestamp
    assert!(timestamp > 1_700_000_000); // Ensure reasonable date range
}

// Test cluster offset iterator
#[test]
fn test_cluster_offset_iter() {
    let start_cluster = Cluster(5);
    let end_cluster = 7;
    let mut iter = ClusterOffsetIter::new(start_cluster, end_cluster);

    assert_eq!(iter.next::<()>(), Some(Cluster(5))); // Verify first cluster
    assert_eq!(iter.next::<()>(), Some(Cluster(6))); // Verify second cluster
    assert_eq!(iter.next::<()>(), Some(Cluster(7))); // Verify last cluster
    assert_eq!(iter.next::<()>(), None); // Ensure end of iteration
}
