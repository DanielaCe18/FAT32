/// Represents a FAT-compatible date and time.
#[derive(Debug, Clone, Copy)]
pub struct FatDateTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}


