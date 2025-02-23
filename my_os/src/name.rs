/// Represents a short 8.3 filename.
#[derive(Debug, Clone, PartialEq)]
pub struct ShortFileName {
    name: [u8; 11], // 8 bytes for name + 3 for extension
}

impl ShortFileName {
    pub fn new(name: &str, ext: &str) -> Self {
        let mut short_name = [b' '; 11];
        let name_bytes = name.as_bytes();
        let ext_bytes = ext.as_bytes();

        // Copy name (up to 8 characters)
        for (i, &byte) in name_bytes.iter().take(8).enumerate() {
            short_name[i] = byte.to_ascii_uppercase();
        }

        // Copy extension (up to 3 characters)
        for (i, &byte) in ext_bytes.iter().take(3).enumerate() {
            short_name[8 + i] = byte.to_ascii_uppercase();
        }

        Self { name: short_name }
    }

    pub fn as_str(&self) -> String {
        format!(
            "{}.{}",
            String::from_utf8_lossy(&self.name[..8]).trim(),
            String::from_utf8_lossy(&self.name[8..]).trim()
        )
    }
}

