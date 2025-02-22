pub mod attribute;
pub mod datetime;
pub mod name;
pub mod offset_iter;
pub mod directory;
pub mod filesystem;
pub mod memory;
pub mod process;
pub mod scheduler;
pub mod syscall;


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
