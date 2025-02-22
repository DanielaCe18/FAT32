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
