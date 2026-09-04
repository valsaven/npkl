# npkl: Node Package Killer

A handy, lightweight CLI tool to identify and remove `node_modules` directories. Written in Rust for blazing-fast performance.

This utility recursively searches for `node_modules` directories starting from your current working directory (or a specified path), calculates their size and presents an interactive menu to select and delete what you no longer need.

## Features

- 🚀 **Blazing Fast**: Written in Rust, optimizing recursion and size calculations.
- ⚡ **Lightweight & Efficient**: Minimal memory and CPU footprint — ideal for virtual machines, CI/CD environments, and resource-constrained systems.
- 📊 **Smart Sorting**: Sort discovered directories by size (largest first) or by file path (A-Z).
- 🎯 **Interactive Control**: Simple, intuitive TUI interface to review, select and batch-delete items.
- 📦 **Zero Runtime Dependencies**: Single self-contained binary, no Node.js runtime required to clean up your projects.

## Installation

### Pre-built Binaries

Download the latest compiled binary for your platform directly from the [GitHub Releases](https://github.com/valsaven/npkl/releases) page - no Rust toolchain required.

### From Crates.io (Recommended)

Requires Rust 1.95 or later:

```bash
cargo install npkl
```

### From Source

```bash
git clone https://github.com/valsaven/npkl.git
cd npkl
cargo build --release
```

*Note: The `release` profile already enables `strip`, `lto`, and `codegen-units = 1` for a minimal binary footprint (~396 KB).*

## Usage

```bash
npkl [path] [--sort size|path]
```

### Options

- `[path]` - Optional target directory to scan. Defaults to the current directory (`.`).
- `--sort <MODE>` - Controls entry sorting: `size` (largest first, default) or `path` (A-Z).

### Controls

- **Up/Down Arrows**: Move selection
- **Space**: Toggle entry selection
- **Enter**: Confirm and delete selected directories
- **Esc**: Quit without making changes

## Development

### Running Tests

```bash
cargo test
```

## License

This project is licensed under the MIT License - see the [LICENSE file](./LICENSE) for details.
