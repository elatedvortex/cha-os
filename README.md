# Cha-OS: A Minimal x86 Kernel

Welcome to **Cha-OS**, a minimalistic yet powerful x86 operating system built from the ground up in **Rust**. Unlike traditional OS development in C, Rust offers **memory safety guarantees**, eliminating entire classes of bugs such as buffer overflows and null pointer dereferences. With Rust's strong type system and ownership model, Cha-OS ensures a more secure and efficient kernel without sacrificing performance.

This project takes heavy inspiration from **Phil Opp's** fantastic series, [Writing an OS in Rust](https://os.phil-opp.com/), which provides a structured approach to kernel development in Rust.

## Features
- **Custom Interrupt Descriptor Table (IDT)**
- **Programmable Interrupt Controller (PIC) handling**
- **Basic exception handling (Breakpoint, Double Faults, Stack Overflow)**
- **Timer interrupt (PIT) for scheduling**
- **Keyboard input handling**
- **Custom VGA text-mode output**
- **Global Descriptor Table (GDT) with Task State Segment (TSS)**
- **Efficient CPU halting loop to reduce power consumption**
- **Dynamic Heap Allocation with a custom allocator** 🗂️

## Why Rust Over C?
- **Memory Safety** 🦀: Say goodbye to segmentation faults and buffer overflows!
- **Zero-Cost Abstractions** ⚡: Get high-level safety with low-level performance.
- **Concurrency Without Data Races** 🏎️: Rust’s ownership model prevents race conditions.
- **Fearless Refactoring** 🔧: Make changes with confidence, thanks to Rust’s strict compiler checks.

## Project Structure
```text
cha-os/
├── src/
│   ├── main.rs        # Kernel entry point and custom panic handlers
│   ├── memory.rs      # Kernel memory management & paging setup
│   ├── allocator.rs   # Dynamic heap allocation
│   ├── lib.rs         # Kernel initialization & utilities
│   ├── vga_buffer.rs  # VGA text mode driver
│   ├── interrupts.rs  # IDT & interrupt handlers
│   ├── gdt.rs         # Global Descriptor Table setup
├── Cargo.toml         # Rust project manifest
└── bootimage/         # Bootable kernel image
## Installation && Running
### Prerequisites
- **Rust nightly toolchain** with `rust-src` component
- `cargo-xbuild` for cross-compilation
- `qemu` for emulation

### Building the Kernel
```sh
cargo build --release
```

### Running with QEMU
```sh
cargo run
```

---

## Interrupts & Exception Handling
Cha-OS features a structured approach to handling **hardware and software interrupts**:

- **IDT Setup:** Defined using `x86_64::structures::idt`
- **Double Fault Handling:** Uses a separate stack via TSS
- **PIC Initialization:** Configured with `pic8259` crate
- **Keyboard Input:** Reads scan codes via I/O ports

---

## Example Keyboard Input
Pressing keys will display the corresponding character on-screen. The kernel processes **scan codes** using `pc-keyboard`:
```rust
if let Some(key) = keyboard.process_keyevent(key_event) {
    match key {
        DecodedKey::Unicode(character) => print!("{}", character),
        DecodedKey::RawKey(key) => print!("{:?}", key),
    }
}
```

---

## Example Heap Allocation
Cha-OS now supports **dynamic heap allocation** using Rust’s standard abstractions:
```rust
// Allocate on the heap
let heap_value = Box::new(42);
println!("Heap value: {}", heap_value);

// Growable vector
let mut vec = Vec::new();
for i in 0..5 {
    vec.push(i);
}
println!("Vector contents: {:?}", vec);

// Reference-counted smart pointer
use alloc::rc::Rc;
let rc_example = Rc::new("Cha-OS in Rust");
println!("Reference Count: {}", Rc::strong_count(&rc_example));
```

---

## Next Steps
- Implement **paging-based memory management**
- Add **system call support**
- Introduce **process scheduling** (beyond cooperative tasks)
- Develop a **basic userspace** with program loading
- Explore **filesystem support** (FAT32 or ext2)

---

## Contributing
Feel free to fork, experiment, and contribute! This project is for **learning** and **exploration**, so dive into the source code and start hacking.

---

🚀 **Cha-OS** — Chaos in control. Built for the fearless, powered by Rust. 🦀
