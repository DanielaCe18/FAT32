//! Cluster handling for FAT32.

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Cluster(pub u32);

impl Cluster {
    /// Convert the cluster number to a byte offset.
    pub fn to_offset(&self, cluster_size: u32) -> u64 {
        (self.0 as u64 - 2) * cluster_size as u64
    }

    /// Check if the cluster number is valid.
    pub fn is_valid(&self) -> bool {
        self.0 >= 2 && self.0 < 0xFFFF_FFF8
    }
}

