# Cha-OS: A Minimal x86 Kernel

Welcome to **Cha-OS**, a minimalistic yet powerful x86 operating system built from the ground up. This project serves as an exploration into low-level system programming, demonstrating how to build an OS kernel with **interrupt handling**, **memory management**, and **hardware communication**.

## Features
- **Custom Interrupt Descriptor Table (IDT)**
- **Programmable Interrupt Controller (PIC) handling**
- **Basic exception handling (Breakpoint, Double Faults, Stack Overflow)**
- **Timer interrupt (PIT) for scheduling**
- **Keyboard input handling**
- **Custom VGA text-mode output**
- **Global Descriptor Table (GDT) with Task State Segment (TSS)**
- **Efficient CPU halting loop to reduce power consumption**

## Project Structure
```md
cha-os/
├── src/
│   ├── main.rs        # Kernel entry point with panic handlers
│   ├── lib.rs         # Kernel initialization & libraries or utilities
│   ├── vga_buffer.rs  # VGA text mode driver
│   ├── interrupts.rs  # IDT & interrupt handlers
│   ├── gdt.rs         # Global Descriptor Table setup
├── Cargo.toml         # Rust project manifest
└── bootimage/         # Bootable kernel image
```

## Installation & Running
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

## Interrupts & Exception Handling
Cha-OS features a structured approach to handling **hardware and software interrupts**:

- **IDT Setup:** Defined using `x86_64::structures::idt`
- **Double Fault Handling:** Uses a separate stack via TSS
- **PIC Initialization:** Configured with `pic8259` crate
- **Keyboard Input:** Reads scan codes via I/O ports

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

## Next Steps
- Implement memory management (paging)
- Add syscall support
- Develop a basic userspace
- Introduce process scheduling

## Contributing
Feel free to fork, experiment, and contribute! This project is for **learning** and **exploration**, so dive into the source code and start hacking.

---
🚀 **Cha-OS** — Chaos in control. Built for the fearless.


