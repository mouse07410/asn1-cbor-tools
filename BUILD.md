# Building the Project

This Cargo project builds two separate executable binaries from two source files in the same directory.

## Project Structure

```
asn1-cbor-tools/
├── Cargo.toml          # Project manifest with two binary targets
├── README.md           # Project documentation
├── .gitignore          # Git ignore rules
├── src/
│   ├── dumpasn1.rs     # ASN.1 DER dumper source
│   └── dumpcbor.rs     # CBOR dumper source
└── docs/
    ├── CLI_REFERENCE.md
    ├── QUICK_REFERENCE.md
    └── EXAMPLES.md
```

## Building Both Executables

### Build All Binaries
```bash
cargo build --release
```

This will create:
- `target/release/dumpasn1`
- `target/release/dumpcbor`

### Build Specific Binary
```bash
# Build only dumpasn1
cargo build --release --bin dumpasn1

# Build only dumpcbor
cargo build --release --bin dumpcbor
```

### Development Build (faster, no optimization)
```bash
cargo build
```

This creates debug builds in `target/debug/`

## Installing

### Install to Cargo's bin directory
```bash
# Install both tools
cargo install --path .

# Install specific tool
cargo install --path . --bin dumpasn1
cargo install --path . --bin dumpcbor
```

After installation, the tools are available in `~/.cargo/bin/` (make sure this is in your PATH).

### Manual Installation
```bash
# Build release versions
cargo build --release

# Copy to system bin directory (may need sudo)
sudo cp target/release/dumpasn1 /usr/local/bin/
sudo cp target/release/dumpcbor /usr/local/bin/

# Or copy to user bin directory
mkdir -p ~/bin
cp target/release/dumpasn1 ~/bin/
cp target/release/dumpcbor ~/bin/
# Make sure ~/bin is in your PATH
```

## Running

### Run Without Installing
```bash
# Run directly with cargo
cargo run --bin dumpasn1 -- certificate.der
cargo run --bin dumpcbor -- data.cbor

# Or run the compiled binary
./target/release/dumpasn1 certificate.der
./target/release/dumpcbor data.cbor
```

### Run After Installing
```bash
dumpasn1 certificate.der
dumpcbor data.cbor
```

## Testing

### Quick Syntax Check
```bash
cargo check
```

### Run with Verbose Output
```bash
cargo run --bin dumpasn1 -- --help
cargo run --bin dumpcbor -- --help
```

### Create Test Files
```bash
# ASN.1 test file (requires OpenSSL)
echo "02 01 2A" | xxd -r -p > test_integer.der
cargo run --bin dumpasn1 -- test_integer.der

# CBOR test file (requires Python with cbor2)
python3 -c "import cbor2, sys; cbor2.dump({'test': 42}, sys.stdout.buffer)" > test.cbor
cargo run --bin dumpcbor -- test.cbor
```

## Cargo.toml Configuration Explained

The key to building multiple binaries is the `[[bin]]` sections:

```toml
[[bin]]
name = "dumpasn1"      # Output binary name
path = "src/dumpasn1.rs"  # Source file path

[[bin]]
name = "dumpcbor"      # Output binary name
path = "src/dumpcbor.rs"  # Source file path
```

Each `[[bin]]` section defines a separate binary target. Cargo will:
1. Compile each source file independently
2. Create separate executables with the specified names
3. Allow building/running them individually or together

## Build Profiles

### Release Profile (Optimized)
```bash
cargo build --release
```
- Maximum optimization (`opt-level = 3`)
- Link-time optimization enabled (`lto = true`)
- Single codegen unit for best optimization (`codegen-units = 1`)
- Debug symbols stripped (`strip = true`)
- Larger binary size but maximum performance

### Debug Profile (Fast Compile)
```bash
cargo build
```
- No optimization (`opt-level = 0`)
- Faster compilation
- Includes debug symbols
- Larger binary, slower execution

## Cargo Commands Reference

### Building
```bash
cargo build                    # Debug build (both binaries)
cargo build --release          # Release build (both binaries)
cargo build --bin dumpasn1     # Build specific binary
cargo build --release --bin dumpcbor  # Release build of specific binary
```

### Running
```bash
cargo run --bin dumpasn1 -- [ARGS]    # Run dumpasn1
cargo run --bin dumpcbor -- [ARGS]    # Run dumpcbor
cargo run --release --bin dumpasn1 -- [ARGS]  # Run optimized version
```

### Checking
```bash
cargo check                    # Quick syntax check (no code generation)
cargo clippy                   # Lint warnings (if clippy installed)
cargo fmt                      # Format code (if rustfmt installed)
```

### Cleaning
```bash
cargo clean                    # Remove all build artifacts
```

### Documentation
```bash
cargo doc --open              # Build and open documentation
```

## Cross-Compilation

### For Different Targets
```bash
# List available targets
rustup target list

# Add a target
rustup target add x86_64-pc-windows-gnu

# Build for specific target
cargo build --release --target x86_64-pc-windows-gnu
```

### Common Targets
- `x86_64-unknown-linux-gnu` - Linux (64-bit)
- `x86_64-apple-darwin` - macOS (64-bit)
- `x86_64-pc-windows-msvc` - Windows (64-bit, MSVC)
- `x86_64-pc-windows-gnu` - Windows (64-bit, MinGW)
- `aarch64-unknown-linux-gnu` - ARM64 Linux
- `aarch64-apple-darwin` - Apple Silicon macOS

## Optimization Tips

### Smaller Binary Size
```bash
# Use system allocator (edit Cargo.toml)
# Add to the top of main files:
# use std::alloc::System;
# #[global_allocator]
# static GLOBAL: System = System;

# Or use UPX compression (external tool)
upx --best --lzma target/release/dumpasn1
upx --best --lzma target/release/dumpcbor
```

### Faster Compilation
```bash
# Use mold linker (Linux)
# Add to ~/.cargo/config.toml:
# [target.x86_64-unknown-linux-gnu]
# linker = "clang"
# rustflags = ["-C", "link-arg=-fuse-ld=mold"]
```

## Troubleshooting

### "binary not found" error
Make sure you're using the correct binary name:
```bash
cargo run --bin dumpasn1  # Correct
cargo run --bin dump_asn1 # Wrong
```

### Compilation fails
```bash
# Check Rust version
rustc --version

# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

### Can't find binaries after install
```bash
# Check if cargo bin is in PATH
echo $PATH | grep cargo

# Add to PATH (add to ~/.bashrc or ~/.zshrc)
export PATH="$HOME/.cargo/bin:$PATH"
```

## Development Workflow

### Typical Development Cycle
```bash
# 1. Make changes to src/dumpasn1.rs or src/dumpcbor.rs

# 2. Quick check for errors
cargo check

# 3. Test the changes
cargo run --bin dumpasn1 -- test.der

# 4. Build release version when ready
cargo build --release

# 5. Test release version
./target/release/dumpasn1 test.der
```

### Adding Dependencies (if needed in future)
Edit `Cargo.toml`:
```toml
[dependencies]
clap = "4.0"  # Example: command-line parser
```

Then rebuild:
```bash
cargo build
```

## Publishing (Optional)

### To crates.io
```bash
# Login to crates.io
cargo login [YOUR_API_TOKEN]

# Publish
cargo publish
```

### Create Distribution Package
```bash
# Build release versions
cargo build --release

# Create tarball
tar czf asn1-cbor-tools-v0.1.0-linux-x64.tar.gz \
    -C target/release \
    dumpasn1 dumpcbor

# Or create zip for Windows
zip asn1-cbor-tools-v0.1.0-windows-x64.zip \
    target/release/dumpasn1.exe \
    target/release/dumpcbor.exe
```

## Performance Benchmarking

```bash
# Use hyperfine (install with: cargo install hyperfine)
hyperfine 'target/release/dumpasn1 large.der'
hyperfine 'target/release/dumpcbor large.cbor'
```

## Summary

The key points for building multiple executables in one Cargo project:

1. **Define multiple `[[bin]]` sections** in `Cargo.toml`
2. **Each binary has its own source file** in `src/`
3. **Build all**: `cargo build --release`
4. **Build specific**: `cargo build --release --bin <name>`
5. **Run specific**: `cargo run --bin <name> -- [args]`
6. **Install**: `cargo install --path .`

This approach keeps related tools in one project while producing separate executables.
