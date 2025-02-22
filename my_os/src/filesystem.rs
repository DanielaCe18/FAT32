use crate::directory::cluster::Cluster;
use crate::directory::table::FatValue;
use spin::Mutex;

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
