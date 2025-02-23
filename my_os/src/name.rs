/// Represents a short 8.3 filename.
#[derive(Debug, Clone, PartialEq)]
pub struct ShortFileName {
    name: [u8; 11], // 8 bytes for name + 3 for extension
}


