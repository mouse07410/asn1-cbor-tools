# Makefile for ASN.1/CBOR Tools
# Alternative to using Cargo directly

.PHONY: all build release debug clean install uninstall test help check fmt clippy test-all test-integration test-python test-coverage test-watch install-system uninstall-system test-basic

# Default target
all: release

# Build both tools in release mode
release:
	@echo "Building release versions..."
	cargo build --release
	@echo "Built binaries in target/release/"

# Build both tools in debug mode
debug:
	@echo "Building debug versions..."
	cargo build
	@echo "Built binaries in target/debug/"

# Alias for release
build: release

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean

# Install to ~/.cargo/bin
install: release
	@echo "Installing to ~/.cargo/bin..."
	cargo install --path .
	@echo "Installed dumpasn1 and dumpcbor"
	@echo "Make sure ~/.cargo/bin is in your PATH"

# Install to system directory (requires sudo)
install-system: release
	@echo "Installing to /usr/local/bin (requires sudo)..."
	sudo cp target/release/dumpasn1 /usr/local/bin/
	sudo cp target/release/dumpcbor /usr/local/bin/
	@echo "Installed to /usr/local/bin/"

# Uninstall from ~/.cargo/bin
uninstall:
	@echo "Uninstalling from ~/.cargo/bin..."
	cargo uninstall asn1-cbor-tools || true
	@echo "Uninstalled"

# Uninstall from system directory
uninstall-system:
	@echo "Uninstalling from /usr/local/bin..."
	sudo rm -f /usr/local/bin/dumpasn1
	sudo rm -f /usr/local/bin/dumpcbor
	@echo "Uninstalled from /usr/local/bin/"

# Quick syntax check
check:
	@echo "Checking code..."
	cargo check

# Format code
fmt:
	@echo "Formatting code..."
	cargo fmt

# Run linter
clippy:
	@echo "Running clippy..."
	cargo clippy -- -D warnings

# Run tests (if any exist)
test:
	@echo "Running unit tests..."
	cargo test

# Run all tests (unit + integration + Python)
test-all: test test-integration test-python
	@echo ""
	@echo "All tests completed!"

# Run integration tests
test-integration: release
	@echo "Running integration tests..."
	@chmod +x tests/integration_test.sh
	@tests/integration_test.sh

# Run Python test suite
test-python: release
	@echo "Running Python test suite..."
	@python3 tests/test_suite.py

# Run tests with coverage (requires tarpaulin)
test-coverage:
	@echo "Generating test coverage..."
	cargo install cargo-tarpaulin || true
	cargo tarpaulin --out Html --output-dir coverage

# Watch tests (requires cargo-watch)
test-watch:
	@echo "Watching for changes and running tests..."
	cargo install cargo-watch || true
	cargo watch -x test

# Create test files and run basic tests
test-basic: release
	@echo "Creating test files and running basic tests..."
	@echo "Testing dumpasn1..."
	@echo "02 01 2A" | xxd -r -p > test_integer.der && \
		./target/release/dumpasn1 test_integer.der && \
		rm -f test_integer.der
	@echo "\nTesting dumpcbor..."
	@python3 -c "import cbor2, sys; cbor2.dump({'test': 42}, sys.stdout.buffer)" > test.cbor && \
		./target/release/dumpcbor test.cbor && \
		rm -f test.cbor
	@echo "\nBasic tests passed!"

# Show help
help:
	@echo "ASN.1/CBOR Tools - Makefile targets:"
	@echo ""
	@echo "  make                 - Build release versions (default)"
	@echo "  make release         - Build optimized release versions"
	@echo "  make debug           - Build debug versions"
	@echo "  make build           - Alias for 'make release'"
	@echo ""
	@echo "  make install         - Install to ~/.cargo/bin"
	@echo "  make install-system  - Install to /usr/local/bin (needs sudo)"
	@echo "  make uninstall       - Uninstall from ~/.cargo/bin"
	@echo "  make uninstall-system - Uninstall from /usr/local/bin"
	@echo ""
	@echo "  make clean           - Remove build artifacts"
	@echo "  make check           - Quick syntax check"
	@echo "  make fmt             - Format code with rustfmt"
	@echo "  make clippy          - Run clippy linter"
	@echo ""
	@echo "  make test            - Run unit tests"
	@echo "  make test-all        - Run all tests (unit + integration + Python)"
	@echo "  make test-integration - Run integration tests (bash)"
	@echo "  make test-python     - Run Python test suite"
	@echo "  make test-basic      - Quick integration test"
	@echo "  make test-coverage   - Generate code coverage report"
	@echo "  make test-watch      - Watch files and run tests on changes"
	@echo ""
	@echo "  make help            - Show this help message"
	@echo ""
	@echo "After building, binaries are in:"
	@echo "  target/release/dumpasn1"
	@echo "  target/release/dumpcbor"
	@echo ""
	@echo "Or use cargo directly:"
	@echo "  cargo build --release"
	@echo "  cargo run --bin dumpasn1 -- <file>"
	@echo "  cargo run --bin dumpcbor -- <file>"
	@echo "  cargo test"
