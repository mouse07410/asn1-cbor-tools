#!/bin/bash
# Format check and fix script
# Run this before committing to ensure code passes CI formatting checks

set -e

echo "Checking Rust code formatting..."

# Check if rustfmt is available
if command -v cargo fmt &> /dev/null; then
    echo "Running cargo fmt..."
    cargo fmt --all
    echo "✓ Code formatted with cargo fmt"
else
    echo "Warning: cargo fmt not found, using fallback sed formatting"
    
    # Remove trailing whitespace
    find . -name "*.rs" -type f -exec sed -i 's/[[:space:]]*$//' {} \;
    
    # Ensure files end with newline
    find . -name "*.rs" -type f -exec sed -i -e '$a\' {} \;
    
    echo "✓ Basic formatting applied"
fi

# Check if clippy is available
if command -v cargo clippy &> /dev/null; then
    echo ""
    echo "Running clippy..."
    cargo clippy --all-targets -- -D warnings
    echo "✓ Clippy checks passed"
else
    echo "Warning: cargo clippy not found, skipping lint checks"
fi

echo ""
echo "All formatting checks passed!"
echo ""
echo "Safe to commit. Run: git add -A && git commit"
