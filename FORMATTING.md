# Code Formatting Guide

This project enforces consistent code formatting using `rustfmt` and `clippy`.

## Quick Fix

Before committing, run:

```bash
# Format code
make fmt

# Or manually
cargo fmt --all

# Or use the script
./scripts/format.sh
```

## CI Formatting Checks

The CI pipeline will fail if code is not properly formatted. The checks include:

1. **rustfmt** - Code must be formatted according to `rustfmt.toml`
2. **clippy** - Code must pass linting without warnings

## Common Formatting Issues

### 1. Trailing Whitespace

**Problem**: Lines with trailing spaces or tabs
```rust
let x = 42;    // <- trailing spaces here
```

**Fix**: Remove trailing whitespace
```rust
let x = 42;
```

### 2. Inconsistent Indentation

**Problem**: Mixed tabs and spaces
```rust
fn main() {
	let x = 1;  // <- tab used here
    let y = 2;  // <- spaces used here
}
```

**Fix**: Use consistent spacing (4 spaces)
```rust
fn main() {
    let x = 1;
    let y = 2;
}
```

### 3. Missing Newline at EOF

**Problem**: File doesn't end with a newline
```rust
fn main() {
    println!("Hello");
}[EOF - no newline]
```

**Fix**: Ensure single newline at end
```rust
fn main() {
    println!("Hello");
}
[newline here]
```

### 4. Multiple Blank Lines

**Problem**: Too many consecutive blank lines
```rust
fn foo() {}


fn bar() {}  // <- 2 blank lines above
```

**Fix**: Single blank line maximum
```rust
fn foo() {}

fn bar() {}
```

## Running Format Checks Locally

### Check Without Modifying

```bash
# Check if formatting is needed
cargo fmt --all -- --check

# Or use make
make fmt-check
```

Exit code:
- `0` = No formatting needed
- `1` = Formatting required

### Apply Formatting

```bash
# Format all code
cargo fmt --all

# Or use make
make fmt
```

### Check Linting

```bash
# Run clippy
cargo clippy --all-targets -- -D warnings

# Or use make
make clippy
```

## Pre-Commit Workflow

Recommended workflow before committing:

```bash
# 1. Format code
make fmt

# 2. Run clippy
make clippy

# 3. Run tests
make test

# 4. Check everything passes
make fmt-check

# 5. Commit
git add -A
git commit -m "Your message"
```

## Automated Script

Use the provided script:

```bash
./scripts/format.sh
```

This will:
- Format code with `cargo fmt`
- Run clippy checks
- Report any issues

## Configuration Files

### rustfmt.toml

Project-wide formatting rules:
- 4 spaces for indentation
- 100 character line width
- Trailing comma for vertical lists
- Alphabetical import ordering
- Unix newlines

### .editorconfig (Optional)

You can create `.editorconfig` for editor support:

```ini
root = true

[*]
charset = utf-8
end_of_line = lf
insert_final_newline = true
trim_trailing_whitespace = true

[*.rs]
indent_style = space
indent_size = 4
max_line_length = 100

[*.toml]
indent_style = space
indent_size = 2

[*.yml]
indent_style = space
indent_size = 2

[Makefile]
indent_style = tab
```

## IDE Setup

### VS Code

Install extensions:
- `rust-analyzer` - Rust language support
- `Better TOML` - TOML file support

Settings (`.vscode/settings.json`):
```json
{
  "editor.formatOnSave": true,
  "rust-analyzer.rustfmt.extraArgs": ["--config-path", "rustfmt.toml"],
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer",
    "editor.formatOnSave": true
  }
}
```

### IntelliJ IDEA / CLion

Settings → Languages & Frameworks → Rust:
- Enable "Run rustfmt on save"
- Set rustfmt config path to `rustfmt.toml`

### Vim / Neovim

Using `rust.vim`:
```vim
let g:rustfmt_autosave = 1
let g:rustfmt_options = '--config-path rustfmt.toml'
```

Using `coc.nvim` with `coc-rust-analyzer`:
```json
{
  "rust-analyzer.rustfmt.extraArgs": ["--config-path", "rustfmt.toml"]
}
```

## Git Hooks

### Pre-commit Hook

Create `.git/hooks/pre-commit`:

```bash
#!/bin/bash
# Pre-commit hook to check formatting

echo "Running pre-commit checks..."

# Check formatting
if ! cargo fmt --all -- --check &> /dev/null; then
    echo "Error: Code is not formatted. Run 'cargo fmt' to fix."
    exit 1
fi

# Run clippy
if ! cargo clippy --all-targets -- -D warnings &> /dev/null; then
    echo "Error: Clippy found issues. Run 'cargo clippy' to see details."
    exit 1
fi

echo "Pre-commit checks passed!"
```

Make it executable:
```bash
chmod +x .git/hooks/pre-commit
```

## Troubleshooting

### "cargo fmt" not found

Install Rust toolchain:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup component add rustfmt
```

### Format check fails in CI but passes locally

1. Pull latest changes
2. Run `cargo fmt --all`
3. Commit the formatting changes
4. Push again

### Different rustfmt versions

Check version:
```bash
cargo fmt --version
```

CI uses the version specified in `rust-toolchain.toml` (if present) or latest stable.

Update local rustfmt:
```bash
rustup update stable
rustup component add rustfmt --toolchain stable
```

## Common CI Errors

### Error: "left behind trailing whitespace"

**Fix**:
```bash
find . -name "*.rs" -type f -exec sed -i 's/[[:space:]]*$//' {} \;
cargo fmt --all
```

### Error: "requires a newline at the end of the file"

**Fix**:
```bash
find . -name "*.rs" -type f -exec sed -i -e '$a\' {} \;
```

### Error: "Diff in /path/to/file.rs"

This means rustfmt would change the file. **Fix**:
```bash
cargo fmt --all
git add -A
git commit --amend --no-edit
git push --force-with-lease
```

## Best Practices

1. **Format before committing** - Always run `cargo fmt` before `git commit`
2. **Use editor integration** - Set up format-on-save in your editor
3. **Check CI locally** - Run `cargo fmt --check` before pushing
4. **Keep dependencies updated** - Update rustfmt regularly
5. **Follow project style** - Respect the `rustfmt.toml` configuration

## Resources

- [rustfmt documentation](https://github.com/rust-lang/rustfmt)
- [rustfmt configuration options](https://rust-lang.github.io/rustfmt/)
- [clippy lints](https://rust-lang.github.io/rust-clippy/master/)
- [Rust style guide](https://doc.rust-lang.org/1.0.0/style/)

## Summary

✅ **Always format before committing**: `make fmt`
✅ **Check formatting**: `make fmt-check`
✅ **Run clippy**: `make clippy`
✅ **Use the script**: `./scripts/format.sh`
✅ **Set up editor integration** for automatic formatting

Following these guidelines ensures your code will pass CI formatting checks.
