#![cfg(test)]
extern crate std;

use crate::filesystem::{FatFileSystem, StorageDevice};
use crate::directory::cluster::Cluster;
use crate::directory::table::FatValue;
use crate::directory::dir_entry::{DirectoryEntry, DirectoryIterator};
use spin::Mutex;
use std::vec::Vec;

// Mock storage device for testing
struct MockStorage {
    data: Mutex<Vec<u8>>,
}

impl MockStorage {
    pub fn new(size: usize) -> Self {
        Self {
            data: Mutex::new(vec![0; size]),
        }
    }
}

impl StorageDevice for MockStorage {
    fn read(&self, offset: u64, buffer: &mut [u8]) -> Result<(), ()> {
        let data = self.data.lock();
        let offset = offset as usize;
        if offset + buffer.len() > data.len() {
            return Err(());
        }
        buffer.copy_from_slice(&data[offset..offset + buffer.len()]);
        Ok(())
    }

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
    let mock_storage = MockStorage::new(1024 * 1024);
    let fs = FatFileSystem::new(mock_storage, 0, 4096);

    assert_eq!(fs.cluster_size, 4096);
}

// Test cluster allocation
#[test]
fn test_cluster_allocation() {
    let mock_storage = MockStorage::new(1024 * 1024);
    let fs = FatFileSystem::new(mock_storage, 0, 4096);

    let cluster = fs.allocate_cluster();
    assert!(cluster.is_some());
}

// Test FAT value conversion
#[test]
fn test_fat_value_conversion() {
    let data_cluster = FatValue::Data(1234);
    assert_eq!(match data_cluster { FatValue::Data(val) => val, _ => 0 }, 1234);

    let end_of_chain = FatValue::EndOfChain;
    assert!(matches!(end_of_chain, FatValue::EndOfChain));
}

// Test DirectoryEntry creation
#[test]
fn test_directory_entry_creation() {
    let entry = DirectoryEntry::new("TESTFILE.TXT", Cluster(5), 1024, 0x20);

    assert_eq!(entry.file_name(), "TESTFILE.TXT");
    assert_eq!(entry.start_cluster.0, 5);
    assert_eq!(entry.file_size, 1024);
    assert!(entry.is_file());
    assert!(!entry.is_directory());
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

    let mut dir_iter = DirectoryIterator::new(&fs, cluster);
    let first_entry = dir_iter.next().unwrap();
    let second_entry = dir_iter.next().unwrap();

    assert_eq!(first_entry.file_name(), "FILE1.TXT");
    assert_eq!(second_entry.file_name(), "DIR1");
}

