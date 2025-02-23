//! FAT Table Management

use crate::directory::cluster::Cluster;
use crate::filesystem::{FatFileSystem, StorageDevice};

/// Represents possible values of a FAT entry.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FatValue {
    Free,           // Cluster is free.
    Data(u32),      // Points to the next cluster in the chain.
    EndOfChain,     // Marks the end of a cluster chain.
    Bad,            // Cluster is marked as bad.
}

impl FatValue {
    /// Retrieves the FAT entry for a given cluster.
    pub fn get<S: StorageDevice>(fs: &FatFileSystem<S>, cluster: Cluster) -> Self {
        let offset = cluster.to_offset(4); // FAT32 uses 4 bytes per entry.
        let mut buffer = [0u8; 4];         // Buffer to store the read value.

        // Read the FAT entry from storage.
        if fs.storage_device.lock().read(fs.partition_start + offset, &mut buffer).is_ok() {
            match u32::from_le_bytes(buffer) {
                0x0000_0000 => FatValue::Free,           // Unused cluster.
                0x0FFF_FFF8..=0x0FFF_FFFF => FatValue::EndOfChain, // End of chain marker.
                0x0FFF_FFF7 => FatValue::Bad,            // Bad cluster marker.
                val => FatValue::Data(val),              // Next cluster in chain.
            }
        } else {
            FatValue::Bad // Return `Bad` if read fails.
        }
    }

    /// Sets the FAT entry for a given cluster.
    pub fn put<S: StorageDevice>(fs: &FatFileSystem<S>, cluster: Cluster, value: Self) {
        let offset = cluster.to_offset(4); // FAT32 uses 4 bytes per entry.

        // Convert `FatValue` to its corresponding raw value.
        let raw_value = match value {
            FatValue::Free => 0x0000_0000,
            FatValue::EndOfChain => 0x0FFF_FFFF,
            FatValue::Bad => 0x0FFF_FFF7,
            FatValue::Data(val) => val,
        };

        // Write the value into the FAT table.
        let buffer = raw_value.to_le_bytes();
        let _ = fs.storage_device.lock().write(fs.partition_start + offset, &buffer);
    }
}
