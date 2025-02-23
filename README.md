# My_OS FAT32 Filesystem

## 📖 Project Overview
My_OS is a custom operating system featuring a lightweight and efficient FAT32 filesystem implemented in Rust. This project demonstrates a modular approach to filesystem design, ensuring compatibility with embedded systems through the `no_std` environment. The FAT32 implementation supports basic file operations, memory allocation, and system calls.

## 📂 Project Structure
The project follows a modular design for clarity and maintainability:

```
my_os/
├─ src/
│  ├─ directory/          # FAT32 filesystem modules
│  │  ├─ attribute.rs     # File attributes (read-only, hidden, system)
│  │  ├─ cluster.rs       # Cluster management
│  │  ├─ datetime.rs      # Date and time handling
│  │  ├─ dir_entry.rs     # Directory entries
│  │  ├─ name.rs          # File name support (short and long)
│  │  ├─ offset_iter.rs   # Cluster iteration
│  │  └─ table.rs         # FAT table management
│  ├─ process/            # Process management
│  │  ├─ context.rs       # Process context switching
│  │  ├─ mod.rs           # Module declarations
│  │  └─ process.rs       # Process creation and management
│  ├─ tests/              # Unit and integration tests
│  │  ├─ tests.rs         # Core functionality tests
│  │  └─ filesystem.rs    # Filesystem-specific tests
│  ├─ lib.rs              # Kernel library entry point
│  ├─ main.rs             # OS entry point (_start)
│  ├─ memory.rs           # Memory management
│  ├─ scheduler.rs        # Process scheduling
│  ├─ slab.rs             # Slab allocator for efficient memory use
│  └─ syscall.rs          # System call interface
├─ Cargo.toml              # Project dependencies and build configuration
└─ Cargo.lock              # Dependency lock file
```

## ⚙️ Technical Choices
- **No-Std Environment:** Utilizes `#![no_std]` for lightweight kernel development, ideal for embedded systems.
- **FAT32 Filesystem:** Supports cluster-based storage, short (8.3) and long file names (LFN), and file attributes.
- **Slab Allocator:** Efficient memory allocation with reduced fragmentation.
- **Spinlocks & Mutex:** Ensures safe concurrent access without OS-level threads.

## 📜 Requirements
Ensure you have the following tools and dependencies installed:

- **Rust:** Latest stable version. Install via [rustup](https://rustup.rs/).
- **Cargo:** Rust's package manager.
- **QEMU:** For virtualized OS testing. Install via your package manager:
  ```bash
  # Ubuntu/Debian
  sudo apt install qemu-system-x86

  # macOS (using Homebrew)
  brew install qemu
  ```
- **bootimage:** For creating bootable images:
  ```bash
  cargo install bootimage
  ```

Ensure the `rust-src` component is installed:
```bash
rustup component add rust-src
```

## 📝 Code Highlights
### Entry Point (`main.rs`)
```rust
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to My OS!");
    let mut process = Process::new("Init");
    process.run();
    process.terminate();
    loop {}
}
```
- `_start` is the OS entry point.
- Creates and runs an initial process.
- Infinite loop maintains kernel execution.

## 🚀 Build and Run Instructions

### Build in No-Std Mode
```bash
cargo build --no-default-features --features "no_std"
```

### Run Tests in Std Mode
```bash
cargo test --features std
```

### Enable Debugging
```bash
cargo build --features "debug global_alloc"
```

### Create Bootable Image
If using `bootimage`:
```bash
cargo bootimage
```

### Run with QEMU
```bash
qemu-system-x86_64 -drive format=raw,file=target/x86_64-my_os/debug/bootimage-my_os.bin
```

## 🧪 Example Test
Located in `src/tests/tests.rs`:
```rust
#[test]
fn test_process_creation() {
    let process = Process::new("TestProcess");
    assert_eq!(process.name, "TestProcess");
    assert_eq!(process.state, ProcessState::Ready);
}
```

## 📜 Future Improvements
- Write-back caching for faster operations.
- Journaling for crash resilience.
- Advanced process scheduling.

## 📄 License
This project is licensed under the MIT License. See the `LICENSE` file for details.

---

🎯 *This project is a step towards building a complete OS with robust filesystem support.*

