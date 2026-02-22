# Testing Guide

This project includes comprehensive testing at multiple levels:

## Test Types

### 1. Unit Tests (Rust)
- Located in: `tests/common_tests.rs`
- Tests data structure parsing and encoding validation
- Run with: `cargo test`

### 2. Integration Tests (Bash)
- Located in: `tests/integration_test.sh`
- Tests actual binary execution with various inputs
- Run with: `./tests/integration_test.sh` or `make test-basic`

### 3. Comprehensive Test Suite (Python)
- Located in: `tests/test_suite.py`
- Generates test files and validates output
- Run with: `python3 tests/test_suite.py`

### 4. Continuous Integration (GitHub Actions)
- Located in: `.github/workflows/ci.yml`
- Runs automatically on push/PR
- Tests on Linux, Windows, macOS

## Running Tests

### Quick Test (Unit Tests Only)
```bash
cargo test
```

### Integration Tests
```bash
# Build first
cargo build --release

# Run bash integration tests
./tests/integration_test.sh

# Or use make
make test-basic
```

### Comprehensive Python Tests
```bash
# Install dependencies
pip install pyasn1 cbor2

# Run tests
python3 tests/test_suite.py
```

### All Tests
```bash
# Build
cargo build --release

# Run unit tests
cargo test

# Run integration tests
./tests/integration_test.sh

# Run Python tests
python3 tests/test_suite.py
```

## Test Coverage

### ASN.1 Tests Cover:
- ✅ INTEGER (positive, negative, large)
- ✅ BOOLEAN (true, false)
- ✅ NULL
- ✅ OCTET STRING
- ✅ BIT STRING
- ✅ OBJECT IDENTIFIER
- ✅ UTF8String and other string types
- ✅ SEQUENCE (simple and nested)
- ✅ SET
- ✅ Tag class identification (UNIVERSAL, CONTEXT, APPLICATION, PRIVATE)
- ✅ Constructed vs primitive flags
- ✅ Short and long form length encoding
- ✅ Error handling for invalid files
- ✅ Command-line options (--help, -v, etc.)

### CBOR Tests Cover:
- ✅ Unsigned integers (small and large)
- ✅ Negative integers
- ✅ Text strings (UTF-8)
- ✅ Byte strings
- ✅ Arrays (simple, nested, mixed types)
- ✅ Maps (simple, nested)
- ✅ Tagged values
- ✅ Boolean (true, false)
- ✅ Null and undefined
- ✅ Floating-point numbers (float16, float32, float64)
- ✅ Major type identification
- ✅ Additional info parsing
- ✅ Indefinite-length encoding
- ✅ Error handling for invalid files
- ✅ Command-line options (--help, -v, etc.)

## CI/CD Pipeline

### GitHub Actions Workflows

#### CI Workflow (`.github/workflows/ci.yml`)
Runs on every push and pull request:

1. **Test Suite**
   - Runs on: Ubuntu, Windows, macOS
   - Rust versions: stable, beta
   - Executes: `cargo test`

2. **Build Binaries**
   - Builds for multiple targets
   - Uploads artifacts for download

3. **Linting**
   - Checks code formatting with `rustfmt`
   - Runs `clippy` for lint warnings

4. **Integration Tests**
   - Runs bash integration tests on Linux
   - Validates actual binary behavior

5. **Code Coverage**
   - Generates coverage reports
   - Uploads to Codecov (optional)

#### Release Workflow (`.github/workflows/release.yml`)
Triggers on version tags (`v*`):

1. Creates GitHub release
2. Builds binaries for:
   - Linux (glibc and musl)
   - Windows (MSVC)
   - macOS (x86_64 and ARM64)
3. Strips binaries for smaller size
4. Creates compressed archives
5. Uploads to GitHub release

### Running CI Locally

You can run similar checks locally:

```bash
# Format check
cargo fmt --all -- --check

# Clippy
cargo clippy --all-targets --all-features -- -D warnings

# Tests
cargo test

# Build all targets
cargo build --release
```

## Test Data Generation

### Creating ASN.1 Test Files

Using Python (pyasn1):
```python
from pyasn1.codec.der import encoder
from pyasn1.type import univ

# Simple integer
integer = univ.Integer(42)
with open('test.der', 'wb') as f:
    f.write(encoder.encode(integer))
```

Using OpenSSL:
```bash
# Generate a certificate
openssl req -new -x509 -days 365 -nodes -out cert.pem -keyout key.pem
openssl x509 -in cert.pem -outform DER -out cert.der
```

Using hex:
```bash
# INTEGER 42
echo "02 01 2A" | xxd -r -p > integer.der

# SEQUENCE with two integers
echo "30 06 02 01 01 02 01 02" | xxd -r -p > sequence.der
```

### Creating CBOR Test Files

Using Python (cbor2):
```python
import cbor2

# Simple map
data = {'name': 'Alice', 'age': 30}
with open('test.cbor', 'wb') as f:
    cbor2.dump(data, f)
```

Using hex:
```bash
# Unsigned integer 42
echo "18 2A" | xxd -r -p > integer.cbor

# Array [1, 2, 3]
echo "83 01 02 03" | xxd -r -p > array.cbor
```

## Manual Testing

### Test Basic Functionality
```bash
# Build first
cargo build --release

# Test ASN.1 dumper
echo "02 01 2A" | xxd -r -p > test.der
./target/release/dumpasn1 test.der

# Test CBOR dumper
echo "18 2A" | xxd -r -p > test.cbor
./target/release/dumpcbor test.cbor
```

### Test Command-Line Options
```bash
# Test help
./target/release/dumpasn1 --help
./target/release/dumpcbor --help

# Test verbose mode
./target/release/dumpasn1 -v test.der
./target/release/dumpcbor -v test.cbor

# Test various options
./target/release/dumpasn1 -p -x test.der
./target/release/dumpcbor -x -o test.cbor
```

### Test Error Handling
```bash
# Non-existent file
./target/release/dumpasn1 nonexistent.der
./target/release/dumpcbor nonexistent.cbor

# Invalid data
echo "FF FF FF FF" | xxd -r -p > invalid.der
./target/release/dumpasn1 invalid.der
```

## Benchmark Testing

### Performance Testing
```bash
# Create large test files
python3 << 'EOF'
from pyasn1.codec.der import encoder
from pyasn1.type import univ

seq = univ.Sequence()
for i in range(10000):
    seq.setComponentByPosition(i, univ.Integer(i))

with open('large.der', 'wb') as f:
    f.write(encoder.encode(seq))
EOF

# Time execution
time ./target/release/dumpasn1 large.der > /dev/null
```

Using hyperfine (if installed):
```bash
hyperfine './target/release/dumpasn1 large.der'
hyperfine './target/release/dumpcbor large.cbor'
```

## Continuous Testing During Development

### Watch Mode (requires cargo-watch)
```bash
# Install cargo-watch
cargo install cargo-watch

# Run tests on file changes
cargo watch -x test

# Run specific test
cargo watch -x 'test asn1_tests'
```

### Quick Feedback Loop
```bash
# Check syntax quickly
cargo check

# Run specific test file
cargo test --test common_tests

# Run with output
cargo test -- --nocapture
```

## Test Requirements

### Rust Tests
- No external dependencies
- Uses standard library only

### Integration Tests (Bash)
- Requires: `bash`, `xxd`
- Optional: `python3` for enhanced tests

### Python Tests
- Requires: `python3` (3.7+)
- Dependencies:
  ```bash
  pip install pyasn1 cbor2
  ```

### CI Requirements
- Automatically installs all dependencies
- Caches Cargo registry and build artifacts
- Runs on multiple platforms

## Troubleshooting Tests

### Tests Fail to Find Binaries
```bash
# Make sure you built first
cargo build --release

# Or use debug build
cargo build
# Then run: ./tests/integration_test.sh
```

### Python Tests Skip All Tests
```bash
# Install dependencies
pip install pyasn1 cbor2

# Verify installation
python3 -c "import pyasn1, cbor2; print('OK')"
```

### CI Fails on GitHub
- Check GitHub Actions logs
- Ensure all tests pass locally first
- Verify `.github/workflows/` files are committed

### Clippy Warnings
```bash
# Fix automatically when possible
cargo clippy --fix

# Or fix manually and check
cargo clippy --all-targets -- -D warnings
```

## Adding New Tests

### Adding a Unit Test
Edit `tests/common_tests.rs`:
```rust
#[test]
fn test_my_feature() {
    // Your test code
    assert_eq!(expected, actual);
}
```

### Adding an Integration Test
Edit `tests/integration_test.sh`:
```bash
# Test N: Description
echo "Test N: My test"
# Create test file
echo "..." | xxd -r -p > test_file
# Run and verify
if $BINARY test_file | grep -q "expected"; then
    test_passed "My test"
else
    test_failed "My test"
fi
```

### Adding a Python Test
Edit `tests/test_suite.py`:
```python
def run_asn1_tests(self):
    # ... existing tests ...
    
    # New test
    self.test_asn1(
        "My Test",
        univ.Integer(123),
        ["INTEGER", "123"]
    )
```

## Test Metrics

Current test coverage:
- Unit tests: 40+ assertions
- Integration tests: 18+ test cases
- Python tests: 25+ test cases
- Total: 80+ automated tests

CI build matrix:
- 3 operating systems (Linux, Windows, macOS)
- 2 Rust versions (stable, beta)
- 6 build configurations
- 18 CI jobs per commit

## Best Practices

1. **Write tests for bug fixes**: Add a test that reproduces the bug before fixing it
2. **Test edge cases**: Empty inputs, very large inputs, invalid inputs
3. **Test error handling**: Ensure errors are caught and reported properly
4. **Keep tests fast**: Unit tests should run in milliseconds
5. **Make tests deterministic**: Avoid randomness or time-dependent behavior
6. **Use descriptive names**: Test names should explain what they test
7. **One assertion per test**: Makes failures easier to diagnose

## Resources

- [Rust Testing Documentation](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [pyasn1 Documentation](https://pyasn1.readthedocs.io/)
- [CBOR2 Documentation](https://cbor2.readthedocs.io/)
