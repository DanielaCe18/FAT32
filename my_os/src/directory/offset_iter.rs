use crate::directory::cluster::Cluster;
use crate::filesystem::FatFileSystem;
use crate::filesystem::StorageDevice;

/// Iterator to navigate through clusters.
pub struct ClusterOffsetIter {
    current: Cluster,
    end: u32,
}

impl ClusterOffsetIter {
    pub fn new(start: Cluster, end: u32) -> Self {
        Self { current: start, end }
    }

    pub fn next<S: StorageDevice>(&mut self, _fs: &FatFileSystem<S>) -> Option<Cluster> {
        if self.current.0 <= self.end {
            let cluster = self.current;
            self.current.0 += 1;
            Some(cluster)
        } else {
            None
        }
    }
}

