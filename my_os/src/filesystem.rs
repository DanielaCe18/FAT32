//! Simple FAT32 Filesystem Implementation

use crate::directory::cluster::Cluster;
use crate::directory::table::FatValue;
use spin::Mutex;

pub trait StorageDevice {
    fn read(&self, offset: u64, buffer: &mut [u8]) -> Result<(), ()>;
    fn write(&self, offset: u64, buffer: &[u8]) -> Result<(), ()>;
}

pub struct FatFileSystem<S: StorageDevice> {
    pub storage_device: Mutex<S>,
    pub partition_start: u64,
    pub cluster_size: u32,
}

impl<S: StorageDevice> FatFileSystem<S> {
    /// Create a new FAT32 filesystem instance.
    pub fn new(storage_device: S, partition_start: u64, cluster_size: u32) -> Self {
        Self {
            storage_device: Mutex::new(storage_device),
            partition_start,
            cluster_size,
        }
    }

    /// Read a cluster from the filesystem.
    pub fn read_cluster(&self, cluster: Cluster) -> Option<alloc::vec::Vec<u8>> {
        let mut buffer = alloc::vec![0; self.cluster_size as usize];
        let offset = self.partition_start + cluster.to_offset(self.cluster_size);

        if self.storage_device.lock().read(offset, &mut buffer).is_ok() {
            Some(buffer)
        } else {
            None
        }
    }

    /// Write data to a cluster.
    pub fn write_cluster(&self, cluster: Cluster, data: &[u8]) -> bool {
        let offset = self.partition_start + cluster.to_offset(self.cluster_size);
        self.storage_device.lock().write(offset, data).is_ok()
    }

    /// Allocate a new cluster.
    pub fn allocate_cluster(&self) -> Option<Cluster> {
        for cluster_id in 2..0xFFFF_FFF8 {
            if let FatValue::Free = FatValue::get(self, Cluster(cluster_id)) {
                FatValue::put(self, Cluster(cluster_id), FatValue::EndOfChain);
                return Some(Cluster(cluster_id));
            }
        }
        None
    }

    /// Free a cluster.
    pub fn free_cluster(&self, cluster: Cluster) {
        FatValue::put(self, cluster, FatValue::Free);
    }
}

