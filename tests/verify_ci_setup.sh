#!/bin/bash
# Quick test to verify CI setup is working

set -e

echo "=== CI Setup Verification ==="
echo ""

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "Error: Not in project root (Cargo.toml not found)"
    exit 1
fi

echo "✓ Found Cargo.toml"

# Build the project
echo ""
echo "Building project..."
cargo build --release

# Check if binaries exist
echo ""
echo "Checking binaries..."
if [ -f "target/release/dumpasn1" ]; then
    echo "✓ dumpasn1 binary exists"
else
    echo "✗ dumpasn1 binary NOT found"
    exit 1
fi

if [ -f "target/release/dumpcbor" ]; then
    echo "✓ dumpcbor binary exists"
else
    echo "✗ dumpcbor binary NOT found"
    exit 1
fi

# Test basic execution
echo ""
echo "Testing basic execution..."

# Create test file
echo "02 01 2A" | xxd -r -p > /tmp/test_integer.der

# Test dumpasn1
if ./target/release/dumpasn1 /tmp/test_integer.der > /dev/null 2>&1; then
    echo "✓ dumpasn1 executes successfully"
else
    echo "✗ dumpasn1 failed to execute"
    exit 1
fi

# Test dumpcbor
echo "18 2A" | xxd -r -p > /tmp/test_cbor.cbor
if ./target/release/dumpcbor /tmp/test_cbor.cbor > /dev/null 2>&1; then
    echo "✓ dumpcbor executes successfully"
else
    echo "✗ dumpcbor failed to execute"
    exit 1
fi

# Test help flags
echo ""
echo "Testing help flags..."
if ./target/release/dumpasn1 --help > /dev/null 2>&1; then
    echo "✓ dumpasn1 --help works"
else
    echo "✗ dumpasn1 --help failed"
    exit 1
fi

if ./target/release/dumpcbor --help > /dev/null 2>&1; then
    echo "✓ dumpcbor --help works"
else
    echo "✗ dumpcbor --help failed"
    exit 1
fi

# Cleanup
rm -f /tmp/test_integer.der /tmp/test_cbor.cbor

echo ""
echo "=== All checks passed! ==="
echo ""
echo "Ready to run:"
echo "  - cargo test"
echo "  - ./tests/integration_test.sh"
echo "  - python3 tests/test_suite.py"
