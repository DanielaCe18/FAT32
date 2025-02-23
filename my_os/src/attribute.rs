/// File attribute flags for FAT32.
#[derive(Debug, Clone, Copy)]
pub struct Attributes(u8);

impl Attributes {
    pub const READ_ONLY: u8 = 0x01;
    pub const HIDDEN: u8 = 0x02;
    pub const SYSTEM: u8 = 0x04;
    pub const DIRECTORY: u8 = 0x10;
    pub const ARCHIVE: u8 = 0x20;

    pub fn new(value: u8) -> Self {
        Self(value)
    }

    pub fn is_read_only(self) -> bool {
        self.0 & Self::READ_ONLY != 0
    }

    pub fn is_hidden(self) -> bool {
        self.0 & Self::HIDDEN != 0
    }

