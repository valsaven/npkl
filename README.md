# npkl: Node Package Killer

A handy tool to identify and remove node_modules directories. Written in Rust for blazing fast performance.

This utility recursively searches for node_modules directories from the point it's launched, presents a menu to review, select, and delete the ones you don't need to free up space.

## Features

- Recursively search for node_modules directories.
- Review the path and size of each found directory.
- Selectively delete unwanted node_modules directories with a simple interface.

## Building

To build it from source you need Rust 1.73 or later, preferably via rustup.

### Global Installation

```bash
rustup update
cargo install npkl
```

### Local Building

Clone this repo and then:

```bash
cargo build --release
# Optionally, you can compress the executable file (Windows example, reduces the file size from ~338KB to ~123KB):
upx --best --lzma target/release/npkl
```

## License

This project is licensed under the MIT License - see the [LICENSE file](./LICENSE) for details.
