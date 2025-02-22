//! FAT Table Management

use crate::directory::cluster::Cluster;
use crate::filesystem::FatFileSystem;
use spin::Mutex;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FatValue {
    Free,
    Data(u32),
    EndOfChain,
    Bad,
}

impl FatValue {
    /// Get the value of a cluster from the FAT table.
    pub fn get<S: StorageDevice>(fs: &FatFileSystem<S>, cluster: Cluster) -> Self {
        let offset = cluster.to_offset(4); // FAT32 uses 4 bytes per entry
        let mut buffer = [0u8; 4];

        if fs.storage_device.lock().read(fs.partition_start + offset, &mut buffer).is_ok() {
            match u32::from_le_bytes(buffer) {
                0x0000_0000 => FatValue::Free,
                0x0FFF_FFF8..=0x0FFF_FFFF => FatValue::EndOfChain,
                0x0FFF_FFF7 => FatValue::Bad,
                val => FatValue::Data(val),
            }
        } else {
            FatValue::Bad
        }
    }

    /// Set the value of a cluster in the FAT table.
    pub fn put<S: StorageDevice>(fs: &FatFileSystem<S>, cluster: Cluster, value: Self) {
        let offset = cluster.to_offset(4);
        let raw_value = match value {
            FatValue::Free => 0x0000_0000,
            FatValue::EndOfChain => 0x0FFF_FFFF,
            FatValue::Bad => 0x0FFF_FFF7,
            FatValue::Data(val) => val,
        };

        let buffer = raw_value.to_le_bytes();
        let _ = fs.storage_device.lock().write(fs.partition_start + offset, &buffer);
    }
}

