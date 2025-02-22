//! High-level directory entry representation for FAT32.

use crate::directory::cluster::Cluster;
use crate::filesystem::FatFileSystem;
use crate::directory::table::FatValue;

/// Represents metadata about a directory entry.
#[derive(Debug, Clone, Copy)]
pub struct DirectoryEntry {
    pub file_name: [u8; 11], // 8.3 format (8 chars name + 3 chars extension)
    pub attributes: u8,      // File attributes (read-only, hidden, etc.)
    pub start_cluster: Cluster,
    pub file_size: u32,
}

impl DirectoryEntry {
    /// Create a new directory entry.
    pub fn new(file_name: &str, start_cluster: Cluster, file_size: u32, attributes: u8) -> Self {
        let mut name_bytes = [0u8; 11];
        let name = file_name.as_bytes();

        // Copy filename into the 8.3 format
        let name_part = &name[0..name.len().min(8)];
        name_bytes[..name_part.len()].copy_from_slice(name_part);

        // Optional extension if present
        if let Some(dot_pos) = file_name.find('.') {
            let ext = &name[dot_pos + 1..];
            let ext_part = &ext[0..ext.len().min(3)];
            name_bytes[8..8 + ext_part.len()].copy_from_slice(ext_part);
        }

        Self {
            file_name: name_bytes,
            attributes,
            start_cluster,
            file_size,
        }
    }

    /// Check if the entry is a directory.
    pub fn is_directory(&self) -> bool {
        self.attributes & 0x10 != 0
    }

    /// Check if the entry is a file.
    pub fn is_file(&self) -> bool {
        self.attributes & 0x10 == 0
    }

    /// Convert the raw filename into a String.
    pub fn file_name(&self) -> String {
        let name = String::from_utf8_lossy(&self.file_name[..8])
            .trim_end_matches(' ')
            .to_string();
        let ext = String::from_utf8_lossy(&self.file_name[8..11])
            .trim_end_matches(' ')
            .to_string();

        if ext.is_empty() {
            name
        } else {
            format!("{}.{}", name, ext)
        }
    }
}

