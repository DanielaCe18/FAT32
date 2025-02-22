/// Mock implementation of a storage device for testing.
pub trait StorageDevice {
    fn read(&self, offset: u64, buffer: &mut [u8]) -> Result<(), ()>;
    fn write(&self, offset: u64, buffer: &[u8]) -> Result<(), ()>;
}

pub struct MockStorage {
    data: Vec<u8>,
}

impl MockStorage {
    pub fn new(size: usize) -> Self {
        MockStorage {
            data: vec![0; size],
        }
    }
}

impl StorageDevice for MockStorage {
    fn read(&self, offset: u64, buffer: &mut [u8]) -> Result<(), ()> {
        if offset as usize + buffer.len() > self.data.len() {
            return Err(());
        }
        buffer.copy_from_slice(&self.data[offset as usize..offset as usize + buffer.len()]);
        Ok(())
    }

    fn write(&self, offset: u64, buffer: &[u8]) -> Result<(), ()> {
        if offset as usize + buffer.len() > self.data.len() {
            return Err(());
        }
        self.data[offset as usize..offset as usize + buffer.len()].copy_from_slice(buffer);
        Ok(())
    }
}

