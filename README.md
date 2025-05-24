# CRC Modbus RTU Calculator

A Rust project for fast Modbus RTU CRC calculation with performance comparison capabilities of different implementations.

## Features

- ✅ **Console version** - interactive mode with performance comparison
- ✅ **GUI application** - simple graphical user interface
- ✅ **Performance optimizations**:
  - Original algorithm (8-byte unrolling)
  - Optimized algorithm (16-byte unrolling + unsafe pointers)
  - Parallel processing with Rayon for large iterations
- ✅ **Performance measurement** - timing and CRC/s metrics

## Requirements

- Rust (latest stable version)
- On macOS: Xcode Command Line Tools may be required for some dependencies

```bash
# Install Xcode Command Line Tools (macOS only)
xcode-select --install
```

## Installation and Usage

### Console Application
```bash
# Run the console application
cargo run --release

# Or build and run separately
cargo build --release
./target/release/rust_crc_project
```

Usage example:
```
Enter byte sequence (HEX, space separated):
01 04 00 00 00 0A

Enter number of repetitions (1 to 1000000000):
1000000

Choose mode:
1) Original algorithm (8-byte unrolling)
2) Optimized algorithm (16-byte unrolling)
3) Compare both
```

### GUI Application

```bash
# Run the GUI application
cargo run --bin gui --release

# Or build and run separately
cargo build --release
./target/release/gui
```

The GUI offers:
- 📝 HEX data input field
- 🔢 Iteration count field
- ⚙️ Checkbox to select optimized version
- 📊 Results display (CRC in various formats + performance)
- 🎯 Buttons with sample data

## Optimizations

### 1. **16-byte unrolling** (vs original 8-byte)
- Larger data blocks processed at once
- Better CPU cache utilization
- Using `unsafe` for faster memory access

### 2. **Avoiding data cloning**
- Sharing references instead of `Arc::clone`
- Lower memory usage

### 3. **Parallel processing**
- Automatic for > 100,000 iterations
- Batch splitting for better load balancing

## Performance

On modern CPUs you can expect:
- **~1-5 million CRC/s** for the original version
- **~10-20% boost** for the optimized version
- **Linear scaling** with number of cores for large iterations

## Additional Commands

```bash
# Run tests
cargo test

# Check code (linting)
cargo clippy

# Format code
cargo fmt

# Run in development mode (faster compilation)
cargo run
cargo run --bin gui

# Check dependencies
cargo tree
```

## Dependencies

- `rayon` - Parallel processing
- `egui` + `eframe` - GUI framework
