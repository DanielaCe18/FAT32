#![cfg(test)]

use crate::filesystem::FatFileSystem;
use crate::directory::cluster::Cluster;
use crate::directory::table::FatValue;
use crate::filesystem::{FatVolumeBootRecord, FatFsType};
use spin::Mutex;
use std::sync::Arc;

// Mock storage device
struct MockStorage {
    data: Mutex<Vec<u8>>,
}

impl MockStorage {
    fn new(size: usize) -> Self {
        MockStorage {
            data: Mutex::new(vec![0; size]),
        }
    }
}

impl storage_device::StorageDevice for MockStorage {
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

#[test]
fn test_filesystem_initialization() {
    let mock_storage = MockStorage::new(1024 * 1024); // 1 MB mock storage
    let boot_record = FatVolumeBootRecord {
        bytes_per_block: 512,
        blocks_per_cluster: 1,
        reserved_block_count: 32,
        fats_count: 2,
        fat_size: 256,
        root_dir_childs_cluster: 2,
        fat_type: FatFsType::Fat32,
        cluster_count: 4096,
        media_type: 0xF8,
    };

    let fs = FatFileSystem::new(
        mock_storage,
        0,
        32 * 512,
        1024 * 1024,
        boot_record,
    )
    .expect("Failed to create FAT filesystem");

    assert_eq!(fs.boot_record.fat_type, FatFsType::Fat32);
}

#[test]
fn test_cluster_allocation() {
    let mock_storage = MockStorage::new(1024 * 1024);
    let boot_record = FatVolumeBootRecord {
        bytes_per_block: 512,
        blocks_per_cluster: 1,
        reserved_block_count: 32,
        fats_count: 2,
        fat_size: 256,
        root_dir_childs_cluster: 2,
        fat_type: FatFsType::Fat32,
        cluster_count: 4096,
        media_type: 0xF8,
    };

    let fs = FatFileSystem::new(
        mock_storage,
        0,
        32 * 512,
        1024 * 1024,
        boot_record,
    )
    .expect("Failed to create FAT filesystem");

    let cluster = fs.alloc_cluster(None).expect("Failed to allocate cluster");
    assert!(cluster.0 >= 2);
}
