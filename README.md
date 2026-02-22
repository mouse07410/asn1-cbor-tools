# ASN.1 and CBOR Dumpers in Rust

[![CI](https://github.com/yourusername/asn1-cbor-tools/workflows/CI/badge.svg)](https://github.com/yourusername/asn1-cbor-tools/actions)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

Command-line tools for dumping encoded binary data structures in human-readable format.

This Cargo project builds two separate executables:
- **dumpasn1** - Dumps DER-encoded ASN.1 data
- **dumpcbor** - Dumps CBOR-encoded data

Both programs are based on the concepts and approach from Peter Gutmann's classic `dumpasn1.c` program.

## Quick Start

```bash
# Build both tools
cargo build --release

# Run them
./target/release/dumpasn1 certificate.der
./target/release/dumpcbor data.cbor

# Or install to ~/.cargo/bin
cargo install --path .
dumpasn1 certificate.der
dumpcbor data.cbor
```

## Building

### Using Cargo (Recommended)

```bash
# Build both binaries in release mode
cargo build --release

# Build specific binary
cargo build --release --bin dumpasn1
cargo build --release --bin dumpcbor

# Install to ~/.cargo/bin
cargo install --path .
```

### Using Make

```bash
# Build both tools
make

# Install to ~/.cargo/bin
make install

# Install to /usr/local/bin (requires sudo)
make install-system

# Show all targets
make help
```

See [BUILD.md](BUILD.md) for detailed build instructions.

## Features

### dumpasn1.rs (ASN.1 DER Dumper)

- Parses and displays DER-encoded ASN.1 structures
- Supports all standard ASN.1 universal tags
- Handles both primitive and constructed types
- Decodes OIDs (Object Identifiers)
- Displays integers, booleans, strings, sequences, sets, etc.
- Shows nested structure with proper indentation
- Detects and displays non-canonical encodings
- Handles indefinite-length encoding

### dumpcbor.rs (CBOR Dumper)

- Parses and displays CBOR-encoded data (RFC 8949)
- Supports all CBOR major types:
  - Unsigned/negative integers
  - Byte strings and text strings
  - Arrays and maps
  - Tagged values
  - Simple values (bool, null, undefined)
  - Floating-point numbers (half, single, double precision)
- Handles indefinite-length items
- Recognizes well-known CBOR tags
- Shows nested structure with proper indentation
- Optional hex dump of byte strings

## Building

See [BUILD.md](BUILD.md) for comprehensive build instructions.

Quick build:
```bash
# Build both tools in release mode
cargo build --release

# Or using Make
make
```

Binaries will be in `target/release/`:
- `target/release/dumpasn1`
- `target/release/dumpcbor`

## Installation

```bash
# Install to ~/.cargo/bin
cargo install --path .

# Or copy to system directory
sudo cp target/release/{dumpasn1,dumpcbor} /usr/local/bin/
```

## Usage

### dumpasn1

Basic usage:
```bash
./dumpasn1 <file.der>
```

**Common Options:**
```bash
./dumpasn1 --help                    # Show all options
./dumpasn1 certificate.der           # Basic dump
./dumpasn1 -p cert.der               # Pure mode (no offsets)
./dumpasn1 -x -a cert.der            # Hex offsets, print all data
./dumpasn1 -o -l 5 large.der         # Outline only, max 5 levels
./dumpasn1 -v --oid-info cert.der    # Verbose with OID information
```

**All Options:**
- `-h, --help` - Show help message
- `-a, --print-all` - Print all data in long blocks (default: 384 bytes)
- `-c, --no-check-charset` - Don't interpret OCTET STRINGs as text
- `-d, --dump-header` - Dump hex header (tag+length)
- `-dd` - Dump hex header + first 24 bytes of content
- `-e, --no-check-encaps` - Don't check for encapsulated data
- `-f <file>` - Specify input file
- `-i, --shallow-indent` - Use 1 space indent instead of 2
- `-l <level>` - Maximum nesting level (default: 100)
- `-o, --outline` - Only show constructed object outline
- `-p, --pure` - Pure display mode (no offset info)
- `-r, --raw-time` - Print time as raw string
- `-t, --text` - Dump text alongside hex for OCTET STRINGs
- `-v, --verbose` - Verbose output
- `-w <width>` - Set output width (default: 80)
- `-x, --hex-values` - Display offsets in hexadecimal
- `-z, --zero-length` - Allow zero-length items
- `--dots` - Print dots to align columns
- `--no-offset` - Don't print offset information
- `--oid-info` - Print extra OID information

Example output:
```
Dumping ASN.1 file: certificate.der

SEQUENCE {
  SEQUENCE {
    [0] {
      INTEGER 2
    }
    INTEGER 123456789
    SEQUENCE {
      OBJECT IDENTIFIER 1.2.840.113549.1.1.11
      NULL
    }
    ...
  }
}

Parsing complete.
```

### dumpcbor

Basic usage:
```bash
./dumpcbor <file.cbor>
```

**Common Options:**
```bash
./dumpcbor --help                    # Show all options
./dumpcbor data.cbor                 # Basic dump
./dumpcbor --hex message.cbor        # Always show hex for byte strings
./dumpcbor -o -x data.cbor           # Show offsets and hex
./dumpcbor -c -l 3 large.cbor        # Compact mode, max 3 levels
./dumpcbor -v --hex-offsets data.cbor # Verbose with hex offsets
```

**All Options:**
- `-h, --help` - Show help message
- `-a, --print-all` - Print all data in long byte strings (default: 384 bytes)
- `-c, --compact` - Compact output with minimal whitespace
- `-f <file>` - Specify input file
- `-l <level>` - Maximum nesting level (default: 100)
- `-m <bytes>` - Maximum bytes to display for byte strings (default: 384)
- `-o, --offsets` - Show byte offsets for each item
- `-t, --no-types` - Don't show type names, only values
- `-v, --verbose` - Verbose output
- `-x, --hex` - Always show hex dump for byte strings
- `--hex-offsets` - Display offsets in hexadecimal
- `--no-decode-nested` - Don't decode nested CBOR in byte strings

Example output:
```
Dumping CBOR file: data.cbor

map(3 pairs) {
  text: "name"
  =>
  text: "Alice"
  ,
  text: "age"
  =>
  unsigned(30)
  ,
  text: "tags"
  =>
  array(2 items) [
    text: "developer"
    ,
    text: "rust"
  ]
}

Parsing complete. 1 item(s) found.
```

## Key Concepts from dumpasn1.c

Both programs follow these design principles from the original C code:

1. **Recursive parsing**: Structured types (SEQUENCE/SET for ASN.1, arrays/maps for CBOR) are parsed recursively
2. **Indentation-based display**: Nested structures are shown with increasing indentation
3. **Type identification**: Each item's type is clearly labeled
4. **Length handling**: Both definite and indefinite lengths are supported
5. **Error tolerance**: Programs try to parse as much as possible even with malformed data
6. **Hex fallback**: Unknown or binary data is displayed in hexadecimal

## ASN.1 Tag Reference

Common universal ASN.1 tags:
- 0x01: BOOLEAN
- 0x02: INTEGER
- 0x03: BIT STRING
- 0x04: OCTET STRING
- 0x05: NULL
- 0x06: OBJECT IDENTIFIER
- 0x09: REAL
- 0x0A: ENUMERATED
- 0x0C: UTF8String
- 0x10: SEQUENCE
- 0x11: SET
- 0x13: PrintableString
- 0x16: IA5String
- 0x17: UTCTime
- 0x18: GeneralizedTime

## CBOR Major Types

- 0: Unsigned integer
- 1: Negative integer
- 2: Byte string
- 3: Text string (UTF-8)
- 4: Array
- 5: Map
- 6: Tagged value
- 7: Simple value/float

## Well-known CBOR Tags

- 0: Date/time string (RFC 3339)
- 1: Epoch-based date/time
- 2: Positive bignum
- 3: Negative bignum
- 21: Base64url encoding expected
- 22: Base64 encoding expected
- 23: Base16 encoding expected
- 32: URI
- 55799: Self-describe CBOR

## Differences from Original dumpasn1.c

These Rust implementations focus on core functionality and make some simplifications:

1. **No config file**: Original dumpasn1.c uses a config file for OID descriptions; these versions don't
2. **Simplified options**: Fewer command-line options for simplicity
3. **Memory safety**: Rust's ownership system prevents buffer overflows and memory errors
4. **Modern types**: Uses Rust's standard types and error handling
5. **CBOR support**: dumpcbor.rs adds support for the CBOR format which wasn't in the original

## Testing

You can create test files:

```bash
# Create a simple ASN.1 DER file with OpenSSL
openssl req -new -x509 -days 365 -nodes -out test.crt -keyout test.key
openssl x509 -in test.crt -outform DER -out test.der
./dumpasn1 test.der

# Create a CBOR file with Python
python3 -c "import cbor2; import sys; cbor2.dump({'name': 'test', 'value': 42}, sys.stdout.buffer)" > test.cbor
./dumpcbor test.cbor
```

## Error Handling

Both programs:
- Report errors and warnings at the end of parsing
- Continue parsing after encountering errors when possible
- Validate data structure as much as possible
- Display diagnostic information for malformed data

## License

These programs follow the same liberal license as the original dumpasn1.c:

> You can use this code in whatever way you want, as long as you don't try to claim you wrote it.

## Credits

- Original dumpasn1.c by Peter Gutmann and contributors
- Rust implementation inspired by the original's design and approach
- CBOR specification: RFC 8949

## Testing

The project includes comprehensive tests at multiple levels:

```bash
# Run unit tests
cargo test

# Run integration tests
./tests/integration_test.sh

# Run Python test suite (requires: pip install -r requirements.txt)
python3 tests/test_suite.py

# Run all tests
make test-all
```

See [TESTING.md](TESTING.md) for detailed testing documentation.

## CI/CD

The project uses GitHub Actions for continuous integration:
- ✅ Automated testing on Linux, Windows, and macOS
- ✅ Code formatting checks (rustfmt)
- ✅ Linting (clippy)
- ✅ Automated releases on version tags
- ✅ Cross-platform binary builds

## Documentation

- [BUILD.md](BUILD.md) - Detailed build and installation guide
- [docs/CLI_REFERENCE.md](docs/CLI_REFERENCE.md) - Complete CLI options reference
- [docs/QUICK_REFERENCE.md](docs/QUICK_REFERENCE.md) - Quick reference card
- [docs/EXAMPLES.md](docs/EXAMPLES.md) - Usage examples and test cases

## Further Reading

- ASN.1: ITU-T X.680 series
- DER: ITU-T X.690
- CBOR: RFC 8949
- Original dumpasn1: http://www.cs.auckland.ac.nz/~pgut001/dumpasn1.c
