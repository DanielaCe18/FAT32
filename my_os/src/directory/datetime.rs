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

impl FatDateTime {
    pub fn new(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        }
    }

    /// Converts the FAT date-time to a UNIX timestamp (simple approximation).
    pub fn to_unix_timestamp(&self) -> u64 {
        let days = (self.year as u64 - 1970) * 365 + (self.month as u64 * 30) + self.day as u64;
        let seconds = days * 24 * 60 * 60;
        seconds + (self.hour as u64 * 3600) + (self.minute as u64 * 60) + self.second as u64
    }
}

