# My_OS FAT32 Filesystem


## 📖Project Overview

My_OS is a custom operating system featuring a lightweight and efficient FAT32 filesystem implemented in Rust. The FAT32 implementation supports basic file operations, memory allocation, and system calls.



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

```bash
$ cargo test

running 28 tests
test test_filesystem_initialization ... ok
test test_cluster_allocation ... ok
test test_fat_value_conversion ... ok
test test_directory_entry_creation ... ok
test test_directory_iterator ... ok
test test_slab_allocator ... ok
test test_virt_to_phys ... ok
test test_global_allocator ... ok
test test_failed_allocation ... ok
test test_memory_allocation ... ok
test test_scheduler_round_robin ... ok
test test_syscall_memory_allocation ... ok
test test_syscall_process_creation ... ok
test test_syscall_read_memory ... ok
test test_syscall_terminate_process ... ok
test test_attributes ... ok
test test_short_filename ... ok
test test_fat_datetime ... ok
test test_cluster_offset_iter ... ok

test result: ok. 28 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.23s

     Running unittests (target/debug/deps/my_os_tests-abc123)

-------------------------------------
All tests passed successfully!
-------------------------------------

```



## 📄 License
This project is licensed under the MIT License. See the `LICENSE` file for details.

---


🎯 *This project is a step towards building a complete OS with robust filesystem support.*

