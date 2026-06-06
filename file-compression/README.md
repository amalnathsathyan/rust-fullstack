# File Compression

A Rust utility for compressing files using gzip compression.

## Usage

```bash
cargo run <source> <target>
```

Where:
- `<source>` - Path to the file you want to compress
- `<target>` - Path where the compressed file will be saved

## Example

```bash
cargo run advt_157.pdf compressed_file.pdf
```

This compresses `advt_157.pdf` and saves the compressed output to `compressed_file.pdf`.

## Requirements

- Rust 1.x or later
- Dependencies: `flate2` for gzip compression

## Building

```bash
cargo build --release
```

## Notes

- The program uses gzip compression via the `flate2` crate
- Ensure both source and target paths are valid
- The target file will be created in the same directory where the command is run
