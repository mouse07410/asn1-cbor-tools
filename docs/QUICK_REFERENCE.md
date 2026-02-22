# Quick Reference Card

## dumpasn1 - Quick Options

```
dumpasn1 [OPTIONS] file.der

Essential:
  -h, --help          Show help
  -v                  Verbose output
  -a                  Print all data (no 384-byte limit)
  -p                  Pure mode (no offsets)
  -o                  Outline only (structure without content)
  -x                  Hex offsets/sizes
  -l N                Max nesting level (default: 100)

Debugging:
  -d                  Dump header (tag+length in hex)
  -dd                 Dump header + 24 bytes
  -t                  Show text alongside hex
  --oid-info          Extra OID information

Display:
  -i                  Shallow indent (1 space)
  -w N                Output width (default: 80)
  --dots              Print alignment dots
  --no-offset         Hide offset column

Parsing:
  -c                  Don't check charset in OCTET STRING
  -e                  Don't check for encapsulated data
  -z                  Allow zero-length items
  -r                  Raw time strings
```

## dumpcbor - Quick Options

```
dumpcbor [OPTIONS] file.cbor

Essential:
  -h, --help          Show help
  -v                  Verbose output
  -x                  Always show hex for byte strings
  -o                  Show byte offsets
  -a                  Print all data (no 384-byte limit)
  -c                  Compact mode (minimal whitespace)
  -l N                Max nesting level (default: 100)

Display:
  -t                  No type names (values only)
  -m N                Max bytes per string (default: 384)
  --hex-offsets       Offsets in hexadecimal

Parsing:
  --no-decode-nested  Don't decode nested CBOR
```

## Common Command Patterns

### ASN.1 Certificate Inspection
```bash
# Quick view
dumpasn1 cert.der

# Detailed with OID info
dumpasn1 -v --oid-info cert.der

# Structure only
dumpasn1 -o cert.der

# Debug encoding
dumpasn1 -dd cert.der
```

### CBOR Data Analysis
```bash
# Quick view
dumpcbor data.cbor

# With hex and offsets
dumpcbor -x -o data.cbor

# Compact view
dumpcbor -c -t data.cbor

# Full detail
dumpcbor -v -a -x data.cbor
```

### Large File Handling
```bash
# ASN.1: First 3 levels only
dumpasn1 -l 3 large.der

# CBOR: Structure outline
dumpcbor -c -l 5 -m 64 large.cbor
```

### Side-by-Side Comparison
```bash
# ASN.1 pure mode
dumpasn1 -p file.der > asn1.txt

# CBOR compact mode
dumpcbor -c -t file.cbor > cbor.txt
```

## Option Combinations

### Maximum Detail
```bash
# ASN.1
dumpasn1 -v -a -dd --oid-info file.der

# CBOR
dumpcbor -v -a -x -o --hex-offsets file.cbor
```

### Minimal Output
```bash
# ASN.1
dumpasn1 -p -o file.der

# CBOR
dumpcbor -c -t file.cbor
```

### Debugging Mode
```bash
# ASN.1
dumpasn1 -v -dd -x file.der

# CBOR
dumpcbor -v -o -x --hex-offsets file.cbor
```

### Documentation Mode
```bash
# ASN.1
dumpasn1 -p -w 120 --dots file.der

# CBOR
dumpcbor -t file.cbor
```

## File Format Quick Check

### Is it ASN.1 DER?
```bash
# Check first few bytes
hexdump -C file.der | head -5

# Common DER starts:
# 30 XX XX XX = SEQUENCE
# 02 XX       = INTEGER
# 04 XX       = OCTET STRING
# 06 XX       = OBJECT IDENTIFIER
```

### Is it CBOR?
```bash
# Check first few bytes
hexdump -C file.cbor | head -5

# Common CBOR starts:
# A1..BF      = Map
# 81..9F      = Array
# 61..7B      = Text string
# 41..5B      = Byte string
# D8 18 XX... = Self-describe CBOR (tag 55799)
```

## Getting Help

```bash
# Full help
dumpasn1 --help
dumpcbor --help

# Man-style view (if available)
dumpasn1 --help | less
dumpcbor --help | less
```

## Exit Codes

- **0** = Success
- **1** = Error (bad arguments, file not found, parse error)

## Tips

1. **Start simple**: Try without options first
2. **Use -v for debugging**: See what the parser is doing
3. **Limit depth on large files**: Use `-l` to prevent huge output
4. **Compare with standard tools**:
   - ASN.1: `openssl asn1parse -inform DER -in file.der`
   - CBOR: `python3 -m cbor2.tool --pretty file.cbor`

## Creating Test Files

### ASN.1
```bash
# From certificate
openssl x509 -in cert.pem -outform DER -out cert.der

# Simple test
echo "02 01 2A" | xxd -r -p > test.der  # INTEGER 42
```

### CBOR
```bash
# With Python
python3 << EOF
import cbor2, sys
cbor2.dump({'test': 42}, sys.stdout.buffer)
EOF > test.cbor
```
