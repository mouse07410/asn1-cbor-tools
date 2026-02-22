# Command-Line Options Reference

## dumpasn1 - ASN.1 DER Dumper

### Synopsis
```
dumpasn1 [OPTIONS] <input_file>
dumpasn1 -f <input_file> [OPTIONS]
```

### Description
Dumps ASN.1 DER-encoded data in a human-readable format. Parses tag-length-value structures and displays them with proper indentation and type identification.

### Options

#### General Options

**-h, --help**
- Show help message and exit
- Displays all available options with descriptions

**-f \<file\>**
- Read input from specified file
- Alternative to using positional argument

**-v, --verbose**
- Enable verbose output mode
- Shows configuration settings and extra parsing information

#### Display Control Options

**-p, --pure**
- Pure display mode
- Suppresses offset and length information on the left side
- Shows only the ASN.1 structure

**-o, --outline**
- Display only constructed object outline
- Skips displaying primitive object content
- Useful for getting a high-level view of structure

**-i, --shallow-indent**
- Use shallow indentation (1 space per level instead of 2)
- Saves horizontal space for deeply nested structures

**-w \<width\>**
- Set output width in characters (default: 80)
- Controls line wrapping and formatting

**--dots**
- Print dots to visually align columns
- Makes it easier to track nesting levels

**--no-offset**
- Don't print offset information
- Cleaner output when offsets aren't needed

#### Data Display Options

**-a, --print-all**
- Print all data in long data blocks
- By default, only first 384 bytes are shown
- Use when you need to see complete content

**-x, --hex-values**
- Display size and offset values in hexadecimal
- Default is decimal notation

**-d, --dump-header**
- Dump hex header (tag + length bytes) before object content
- Useful for debugging encoding issues

**-dd**
- Enhanced header dump mode
- Dumps hex header plus first 24 bytes of content

**-t, --text**
- Dump text alongside hex data for OCTET STRINGs
- Shows both hex and ASCII representation

**-r, --raw-time**
- Print time values as raw strings
- Instead of formatted date/time output

#### Parsing Control Options

**-c, --no-check-charset**
- Don't try to interpret OCTET STRINGs as character strings
- Treats all OCTET STRINGs as binary data

**-e, --no-check-encaps**
- Don't check for encapsulated data in BIT/OCTET STRINGs
- Some ASN.1 structures contain nested encoded objects

**-z, --zero-length**
- Allow zero-length items
- Normally flagged as errors unless explicitly allowed

**-l \<level\>, --max-level \<level\>**
- Set maximum nesting level for display (default: 100)
- Items beyond this depth won't be displayed

**--oid-info**
- Print extra information about Object Identifiers
- Shows detailed OID descriptions when available

### Examples

```bash
# Basic usage
dumpasn1 certificate.der

# Pure mode with hex offsets
dumpasn1 -p -x certificate.der

# Outline view of structure only, max 5 levels
dumpasn1 -o -l 5 large_file.der

# Verbose with full data display
dumpasn1 -v -a certificate.der

# Dump headers for debugging
dumpasn1 -dd certificate.der

# Compact display without offsets
dumpasn1 -p --no-offset certificate.der

# Wide display with text alongside hex
dumpasn1 -w 120 -t certificate.der
```

### Exit Status
- 0: Success
- 1: Error (invalid arguments, file not found, parse error)

---

## dumpcbor - CBOR Dumper

### Synopsis
```
dumpcbor [OPTIONS] <input_file>
dumpcbor -f <input_file> [OPTIONS]
```

### Description
Dumps CBOR-encoded data (RFC 8949) in a human-readable format. Parses all CBOR major types including integers, strings, arrays, maps, tagged values, and floating-point numbers.

### Options

#### General Options

**-h, --help**
- Show help message and exit
- Displays all available options and CBOR type information

**-f \<file\>**
- Read input from specified file
- Alternative to using positional argument

**-v, --verbose**
- Enable verbose output mode
- Shows configuration settings and detailed parsing information

#### Display Control Options

**-c, --compact**
- Compact output mode
- Minimizes whitespace and indentation
- Useful for machine processing or when space is limited

**-t, --no-types**
- Don't show type names
- Displays only values without type prefixes
- Example: `42` instead of `unsigned(42)`

**-o, --offsets**
- Show byte offsets for each item
- Displays position in file for each value
- Useful for debugging and reference

**--hex-offsets**
- Display offsets in hexadecimal format
- Instead of decimal (requires -o/--offsets)

#### Data Display Options

**-x, --hex**
- Always show hex dump for byte strings
- By default, hex is shown only for strings ≤ 64 bytes
- Forces hex display for all byte strings

**-a, --print-all**
- Print all data in long byte strings
- Default limit is 384 bytes
- Use when you need to see complete content

**-m \<bytes\>, --max-bytes \<bytes\>**
- Set maximum bytes to display for byte strings (default: 384)
- Controls truncation of long byte strings

#### Parsing Control Options

**-l \<level\>, --max-level \<level\>**
- Set maximum nesting level to display (default: 100)
- Items beyond this depth won't be displayed
- Useful for limiting output from deeply nested structures

**--no-decode-nested**
- Don't attempt to decode nested CBOR in byte strings
- Some CBOR data contains CBOR-encoded byte strings
- Use this to prevent automatic nested decoding

### Examples

```bash
# Basic usage
dumpcbor message.cbor

# Show hex and offsets
dumpcbor --hex --offsets message.cbor

# Compact mode, max 3 levels deep
dumpcbor -c -l 3 large_data.cbor

# Verbose with hex offsets
dumpcbor -v --hex-offsets data.cbor

# Print all data with decimal offsets
dumpcbor -a -o data.cbor

# No type names, compact format
dumpcbor -t -c data.cbor

# Show hex for all byte strings, limit to 512 bytes each
dumpcbor -x -m 512 binary_data.cbor
```

### CBOR Major Types
The following major types are supported:

- **0**: Unsigned integer (0 to 2^64-1)
- **1**: Negative integer (-1 to -2^64)
- **2**: Byte string (binary data)
- **3**: Text string (UTF-8 encoded)
- **4**: Array (ordered list of items)
- **5**: Map (key-value pairs)
- **6**: Tagged value (semantic tagging)
- **7**: Simple value/float (bool, null, undefined, floats)

### Well-Known Tags
Common CBOR tags recognized by the dumper:

- **0**: Date/time string (RFC 3339)
- **1**: Epoch-based date/time (Unix timestamp)
- **2**: Positive bignum
- **3**: Negative bignum
- **4**: Decimal fraction
- **5**: Bigfloat
- **21**: Base64url encoding expected
- **22**: Base64 encoding expected
- **23**: Base16 encoding expected
- **24**: Encoded CBOR data item
- **32**: URI
- **55799**: Self-describe CBOR (magic number)

### Exit Status
- 0: Success
- 1: Error (invalid arguments, file not found, parse error)

---

## Common Usage Patterns

### Comparing ASN.1 and CBOR Output Styles

**ASN.1 Verbose**:
```bash
dumpasn1 -v -a certificate.der > cert_full.txt
```

**ASN.1 Outline**:
```bash
dumpasn1 -o certificate.der > cert_outline.txt
```

**CBOR Detailed**:
```bash
dumpcbor -v -x -o data.cbor > data_detailed.txt
```

**CBOR Compact**:
```bash
dumpcbor -c -t data.cbor > data_compact.txt
```

### Debugging Encoded Data

**Find encoding errors in ASN.1**:
```bash
dumpasn1 -v -dd suspect.der 2>&1 | grep -i error
```

**Inspect specific byte ranges in CBOR**:
```bash
dumpcbor -o --hex-offsets data.cbor | grep "^\[00A4\]"
```

### Processing Large Files

**ASN.1 - First few levels only**:
```bash
dumpasn1 -l 3 large.der
```

**CBOR - Outline with limited data**:
```bash
dumpcbor -l 5 -m 64 large.cbor
```

### Extracting Structure Information

**ASN.1 structure only**:
```bash
dumpasn1 -p -o certificate.der
```

**CBOR structure only**:
```bash
dumpcbor -t -c message.cbor
```

## Environment

Both programs read binary data from files and write text output to stdout. Error messages go to stderr.

### Input Requirements

**dumpasn1**:
- Binary DER-encoded ASN.1 data
- Not PEM-encoded (no -----BEGIN/END----- markers)
- Can be extracted from certificates, keys, PKCS files

**dumpcbor**:
- Binary CBOR-encoded data (RFC 8949 / RFC 7049)
- Can be created with CBOR libraries in various languages
- Used in CoAP, COSE, CWT, and IoT protocols

### Output Redirection

```bash
# Save output to file
dumpasn1 cert.der > output.txt

# Both stdout and stderr
dumpasn1 cert.der > output.txt 2>&1

# Pipe to pager
dumpasn1 large.der | less

# Pipe to grep
dumpcbor data.cbor | grep "text:"
```

## Tips and Best Practices

### For ASN.1

1. **Start with outline mode** (`-o`) on large files to understand structure
2. **Use `-dd` to debug** encoding issues by seeing raw bytes
3. **Enable OID info** (`--oid-info`) when working with certificates
4. **Pure mode** (`-p`) gives cleaner output for documentation

### For CBOR

1. **Use verbose mode** (`-v`) first to see configuration
2. **Offsets are helpful** (`-o`) when cross-referencing with hex editors
3. **Compact mode** (`-c`) is useful for scripts and automation
4. **Show hex** (`-x`) for byte strings to see actual binary content

### Performance

Both programs are designed to handle:
- Files up to several MB efficiently
- Deeply nested structures (configurable limit)
- Malformed data (best-effort parsing)

For very large files (>10 MB), consider using `-l` to limit nesting depth.
