# ASN.1/CBOR Tools - Cargo Project Summary

## What You Have

A complete Rust Cargo project that builds TWO separate executables from two source files:

1. **dumpasn1** - ASN.1 DER dumper
2. **dumpcbor** - CBOR dumper

## Project Structure

```
asn1-cbor-tools/
├── Cargo.toml              # Project manifest defining two binary targets
├── Makefile                # Alternative build system using make
├── README.md               # Main project documentation
├── BUILD.md                # Comprehensive build guide
├── LICENSE                 # MIT License
├── .gitignore              # Git ignore rules
├── src/
│   ├── dumpasn1.rs        # ASN.1 dumper source (standalone executable)
│   └── dumpcbor.rs        # CBOR dumper source (standalone executable)
└── docs/
    ├── CLI_REFERENCE.md   # Complete CLI options reference
    ├── QUICK_REFERENCE.md # Quick command reference
    └── EXAMPLES.md        # Usage examples and tests
```

## Key Cargo.toml Configuration

```toml
# This is what makes it work - multiple binary targets
[[bin]]
name = "dumpasn1"
path = "src/dumpasn1.rs"

[[bin]]
name = "dumpcbor"
path = "src/dumpcbor.rs"
```

Each `[[bin]]` section tells Cargo to:
- Compile that source file as a separate executable
- Give it the specified name
- Allow building/running them independently or together

## How to Build

### Option 1: Using Cargo (Standard)

```bash
# Build both executables
cargo build --release

# Result:
# - target/release/dumpasn1
# - target/release/dumpcbor
```

### Option 2: Using Make

```bash
# Build both executables
make

# Install to ~/.cargo/bin
make install

# Install to /usr/local/bin
make install-system
```

### Build Specific Binary

```bash
# Only build dumpasn1
cargo build --release --bin dumpasn1

# Only build dumpcbor
cargo build --release --bin dumpcbor
```

## How to Run

### Before Installing

```bash
# Direct execution
./target/release/dumpasn1 file.der
./target/release/dumpcbor file.cbor

# Using cargo run
cargo run --bin dumpasn1 -- file.der
cargo run --bin dumpcbor -- file.cbor
```

### After Installing

```bash
# Install to ~/.cargo/bin
cargo install --path .

# Then use anywhere (if ~/.cargo/bin is in PATH)
dumpasn1 certificate.der
dumpcbor message.cbor
```

## CLI Features Implemented

Both tools have comprehensive CLI options:

### dumpasn1 Options
- `-h, --help` - Show help
- `-a` - Print all data
- `-p` - Pure mode (no offsets)
- `-o` - Outline mode
- `-v` - Verbose
- `-x` - Hex offsets
- `-l N` - Max nesting level
- `-d, -dd` - Dump headers
- And many more (see CLI_REFERENCE.md)

### dumpcbor Options
- `-h, --help` - Show help
- `-x` - Always show hex
- `-o` - Show offsets
- `-a` - Print all data
- `-c` - Compact mode
- `-v` - Verbose
- `-l N` - Max nesting level
- And more (see CLI_REFERENCE.md)

## What Makes This Special

### Multiple Binaries in One Project
- Traditional approach: separate Cargo project per binary
- This approach: one project, multiple binaries
- Benefits:
  - Shared documentation
  - Single repository
  - Consistent versioning
  - Easier maintenance
  - Related tools stay together

### No External Dependencies
- Uses only Rust standard library
- No `cargo.lock` conflicts
- Fast compilation
- Small binaries
- Portable

### Production-Ready Features
- Comprehensive CLI parsing
- Proper error handling
- Help messages
- Multiple output modes
- Configurable behavior

## File Sizes (Approximate)

After `cargo build --release` with optimization:
- dumpasn1: ~350-450 KB (stripped)
- dumpcbor: ~300-400 KB (stripped)

Debug builds are larger (~2-3 MB each).

## Cargo Commands Quick Reference

```bash
# Building
cargo build                          # Debug build (both)
cargo build --release               # Release build (both)
cargo build --bin dumpasn1          # Debug build (specific)
cargo build --release --bin dumpcbor # Release build (specific)

# Running
cargo run --bin dumpasn1 -- [ARGS]
cargo run --bin dumpcbor -- [ARGS]

# Installing
cargo install --path .              # Install both to ~/.cargo/bin
cargo install --path . --bin dumpasn1  # Install only dumpasn1

# Checking
cargo check                         # Quick syntax check
cargo clippy                        # Lint (if installed)
cargo fmt                          # Format code (if installed)

# Cleaning
cargo clean                        # Remove build artifacts
```

## Make Commands Quick Reference

```bash
make                    # Build release versions
make install            # Install to ~/.cargo/bin
make install-system     # Install to /usr/local/bin (sudo)
make clean              # Clean build artifacts
make test-basic         # Run basic tests
make help               # Show all make targets
```

## Example Usage

### ASN.1 Certificate
```bash
# Get a certificate
openssl x509 -in cert.pem -outform DER -out cert.der

# Dump it
./target/release/dumpasn1 cert.der

# Or with options
./target/release/dumpasn1 -v --oid-info cert.der
```

### CBOR Data
```bash
# Create CBOR file
python3 << EOF
import cbor2, sys
data = {'name': 'Alice', 'age': 30}
cbor2.dump(data, sys.stdout.buffer)
EOF > person.cbor

# Dump it
./target/release/dumpcbor person.cbor

# Or with options
./target/release/dumpcbor -x -o person.cbor
```

## Cross-Compilation

Build for other platforms:

```bash
# Linux to Windows
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu

# Linux to macOS (requires special setup)
rustup target add x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin

# For ARM (Raspberry Pi, etc.)
rustup target add armv7-unknown-linux-gnueabihf
cargo build --release --target armv7-unknown-linux-gnueabihf
```

## Distribution

### Create Release Package

```bash
# Build optimized versions
cargo build --release

# Create tarball
tar czf asn1-cbor-tools-v0.1.0-linux-x64.tar.gz \
    -C target/release dumpasn1 dumpcbor

# Or create zip
cd target/release
zip ../../asn1-cbor-tools-v0.1.0-linux-x64.zip dumpasn1 dumpcbor
cd ../..
```

## Testing

### Manual Tests
```bash
# ASN.1 integer test
echo "02 01 2A" | xxd -r -p > test.der
./target/release/dumpasn1 test.der
# Expected: INTEGER 42

# CBOR test
python3 -c "import cbor2, sys; cbor2.dump(42, sys.stdout.buffer)" > test.cbor
./target/release/dumpcbor test.cbor
# Expected: unsigned(42)
```

### Using Make
```bash
make test-basic
```

## Troubleshooting

### Rust Not Installed
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Cargo Commands Not Found
```bash
# Add to PATH (add to ~/.bashrc or ~/.zshrc)
export PATH="$HOME/.cargo/bin:$PATH"
```

### Binary Name Typos
```bash
# Correct
cargo run --bin dumpasn1

# Wrong (will fail)
cargo run --bin dump_asn1
cargo run --bin dump-asn1
```

## Why This Approach?

### Advantages of Multiple Binaries in One Project

1. **Related Tools Together**: ASN.1 and CBOR are both encoding formats
2. **Shared Documentation**: One README, one BUILD.md
3. **Consistent Interface**: Both tools use similar CLI patterns
4. **Easier Maintenance**: Update both tools together
5. **Single Version**: Both tools share version number
6. **Simpler CI/CD**: One build pipeline for both

### When NOT to Use This Approach

- Tools are completely unrelated
- Different release cycles needed
- Different dependency requirements
- One tool is library, other is binary
- Want to publish separately to crates.io

## Next Steps

1. **Extract and Build**:
   ```bash
   tar xzf asn1-cbor-tools.tar.gz
   cd asn1-cbor-tools
   cargo build --release
   ```

2. **Try the Tools**:
   ```bash
   ./target/release/dumpasn1 --help
   ./target/release/dumpcbor --help
   ```

3. **Install System-Wide**:
   ```bash
   cargo install --path .
   # Or
   make install-system
   ```

4. **Read Documentation**:
   - `BUILD.md` - Build details
   - `docs/CLI_REFERENCE.md` - All options
   - `docs/QUICK_REFERENCE.md` - Quick lookup
   - `docs/EXAMPLES.md` - Usage examples

## Summary

You now have a complete, production-ready Rust Cargo project that:

✅ Builds two separate executables from one project
✅ Includes comprehensive CLI argument parsing with --help
✅ Has detailed documentation
✅ Supports multiple build methods (cargo, make)
✅ Includes examples and tests
✅ Uses only standard library (no external dependencies)
✅ Optimized release builds
✅ Ready for distribution
✅ Cross-compilation ready

The key innovation is the `[[bin]]` sections in Cargo.toml that tell Cargo to build multiple independent executables from different source files while keeping them in a single cohesive project.
