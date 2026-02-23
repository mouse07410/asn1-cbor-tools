// CBOR dumper in Rust
// Based on the concepts and approach from dumpasn1.c
// Dumps CBOR-encoded data in a human-readable format

use std::fs::File;
use std::io::{self, Read, BufReader};
use std::env;

// CBOR major types
const MAJOR_UNSIGNED: u8 = 0;
const MAJOR_NEGATIVE: u8 = 1;
const MAJOR_BYTES: u8 = 2;
const MAJOR_TEXT: u8 = 3;
const MAJOR_ARRAY: u8 = 4;
const MAJOR_MAP: u8 = 5;
const MAJOR_TAG: u8 = 6;
const MAJOR_SIMPLE: u8 = 7;

// Additional info values
const AI_1BYTE: u8 = 24;
const AI_2BYTES: u8 = 25;
const AI_4BYTES: u8 = 26;
const AI_8BYTES: u8 = 27;
const AI_INDEFINITE: u8 = 31;

// Simple values
const SIMPLE_FALSE: u8 = 20;
const SIMPLE_TRUE: u8 = 21;
const SIMPLE_NULL: u8 = 22;
const SIMPLE_UNDEFINED: u8 = 23;

// Well-known CBOR tags
const TAG_DATETIME: u64 = 0;
const TAG_EPOCH: u64 = 1;
const TAG_BIGNUM_POS: u64 = 2;
const TAG_BIGNUM_NEG: u64 = 3;
const TAG_DECIMAL: u64 = 4;
const TAG_BIGFLOAT: u64 = 5;
const TAG_BASE64URL: u64 = 21;
const TAG_BASE64: u64 = 22;
const TAG_BASE16: u64 = 23;
const TAG_CBOR: u64 = 24;
const TAG_URI: u64 = 32;
const TAG_BASE64URL_ENC: u64 = 33;
const TAG_BASE64_ENC: u64 = 34;
const TAG_REGEX: u64 = 35;
const TAG_MIME: u64 = 36;
const TAG_SELF_DESCRIBE: u64 = 55799;

/// Structure to hold information about a CBOR item
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct CborItem {
    major_type: u8,
    additional_info: u8,
    value: CborValue,
    raw_bytes: Vec<u8>,
}

/// CBOR value types
#[derive(Debug, Clone)]
enum CborValue {
    Unsigned(u64),
    Negative(i64),
    Bytes(Vec<u8>),
    Text(String),
    Array(Vec<CborItem>),
    Map(Vec<(CborItem, CborItem)>),
    Tag(u64, Box<CborItem>),
    Simple(u8),
    Boolean(bool),
    Null,
    Undefined,
    Float16(f32),
    Float32(f32),
    Float64(f64),
    Break,
}

impl CborItem {
    fn new(major_type: u8, additional_info: u8, value: CborValue) -> Self {
        CborItem {
            major_type,
            additional_info,
            value,
            raw_bytes: Vec::new(),
        }
    }
}

/// Configuration options for the dumper
#[derive(Debug)]
struct Config {
    print_hex: bool,
    max_bytes_display: usize,
    max_nest_level: usize,
    decode_nested: bool,
    show_offsets: bool,
    verbose: bool,
    compact: bool,
    print_all_data: bool,
    hex_values: bool,
    show_types: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            print_hex: false,
            max_bytes_display: 384,
            max_nest_level: 100,
            decode_nested: true,
            show_offsets: false,
            verbose: false,
            compact: false,
            print_all_data: false,
            hex_values: false,
            show_types: true,
        }
    }
}

/// Main dumper state
struct CborDumper {
    config: Config,
    no_errors: usize,
    no_warnings: usize,
    offset: usize,
}

impl CborDumper {
    fn new(config: Config) -> Self {
        CborDumper {
            config,
            no_errors: 0,
            no_warnings: 0,
            offset: 0,
        }
    }

    /// Get the name of a well-known tag
    fn tag_name(&self, tag: u64) -> Option<&'static str> {
        match tag {
            TAG_DATETIME => Some("date/time string"),
            TAG_EPOCH => Some("epoch-based date/time"),
            TAG_BIGNUM_POS => Some("positive bignum"),
            TAG_BIGNUM_NEG => Some("negative bignum"),
            TAG_DECIMAL => Some("decimal fraction"),
            TAG_BIGFLOAT => Some("bigfloat"),
            TAG_BASE64URL => Some("base64url encoding"),
            TAG_BASE64 => Some("base64 encoding"),
            TAG_BASE16 => Some("base16 encoding"),
            TAG_CBOR => Some("encoded CBOR data item"),
            TAG_URI => Some("URI"),
            TAG_BASE64URL_ENC => Some("base64url"),
            TAG_BASE64_ENC => Some("base64"),
            TAG_REGEX => Some("regular expression"),
            TAG_MIME => Some("MIME message"),
            TAG_SELF_DESCRIBE => Some("self-describe CBOR"),
            _ => None,
        }
    }

    /// Read additional info value (length or value)
    fn read_additional<R: Read>(&mut self, reader: &mut R, ai: u8) -> io::Result<u64> {
        match ai {
            0..=23 => Ok(ai as u64),
            AI_1BYTE => {
                let mut buf = [0u8; 1];
                reader.read_exact(&mut buf)?;
                self.offset += 1;
                Ok(buf[0] as u64)
            }
            AI_2BYTES => {
                let mut buf = [0u8; 2];
                reader.read_exact(&mut buf)?;
                self.offset += 2;
                Ok(u16::from_be_bytes(buf) as u64)
            }
            AI_4BYTES => {
                let mut buf = [0u8; 4];
                reader.read_exact(&mut buf)?;
                self.offset += 4;
                Ok(u32::from_be_bytes(buf) as u64)
            }
            AI_8BYTES => {
                let mut buf = [0u8; 8];
                reader.read_exact(&mut buf)?;
                self.offset += 8;
                Ok(u64::from_be_bytes(buf))
            }
            AI_INDEFINITE => Ok(u64::MAX), // Marker for indefinite length
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid additional info")),
        }
    }

    /// Read a CBOR item
    fn read_item<R: Read>(&mut self, reader: &mut R) -> io::Result<Option<CborItem>> {
        let mut initial_byte = [0u8; 1];
        if reader.read(&mut initial_byte)? == 0 {
            return Ok(None); // EOF
        }

        let byte = initial_byte[0];
        let major_type = (byte >> 5) & 0x07;
        let additional_info = byte & 0x1F;
        self.offset += 1;

        let value = match major_type {
            MAJOR_UNSIGNED => {
                let val = self.read_additional(reader, additional_info)?;
                CborValue::Unsigned(val)
            }
            MAJOR_NEGATIVE => {
                let val = self.read_additional(reader, additional_info)?;
                // CBOR negative int is -1 - n
                CborValue::Negative(-1 - (val as i64))
            }
            MAJOR_BYTES => {
                if additional_info == AI_INDEFINITE {
                    // Indefinite-length byte string
                    let mut chunks = Vec::new();
                    while let Some(chunk) = self.read_item(reader)? {
                        if let CborValue::Break = chunk.value {
                            break;
                        }
                        if let CborValue::Bytes(b) = chunk.value {
                            chunks.extend(b);
                        } else {
                            self.no_errors += 1;
                            eprintln!("Error: Non-byte-string chunk in indefinite byte string");
                        }
                    }
                    CborValue::Bytes(chunks)
                } else {
                    let length = self.read_additional(reader, additional_info)? as usize;
                    let mut bytes = vec![0u8; length];
                    reader.read_exact(&mut bytes)?;
                    self.offset += length;
                    CborValue::Bytes(bytes)
                }
            }
            MAJOR_TEXT => {
                if additional_info == AI_INDEFINITE {
                    // Indefinite-length text string
                    let mut text = String::new();
                    while let Some(chunk) = self.read_item(reader)? {
                        if let CborValue::Break = chunk.value {
                            break;
                        }
                        if let CborValue::Text(t) = chunk.value {
                            text.push_str(&t);
                        } else {
                            self.no_errors += 1;
                            eprintln!("Error: Non-text-string chunk in indefinite text string");
                        }
                    }
                    CborValue::Text(text)
                } else {
                    let length = self.read_additional(reader, additional_info)? as usize;
                    let mut bytes = vec![0u8; length];
                    reader.read_exact(&mut bytes)?;
                    self.offset += length;
                    match String::from_utf8(bytes) {
                        Ok(s) => CborValue::Text(s),
                        Err(e) => {
                            self.no_errors += 1;
                            CborValue::Text(format!("<invalid UTF-8: {}>", e))
                        }
                    }
                }
            }
            MAJOR_ARRAY => {
                if additional_info == AI_INDEFINITE {
                    // Indefinite-length array
                    let mut items = Vec::new();
                    while let Some(item) = self.read_item(reader)? {
                        if let CborValue::Break = item.value {
                            break;
                        }
                        items.push(item);
                    }
                    CborValue::Array(items)
                } else {
                    let length = self.read_additional(reader, additional_info)? as usize;
                    let mut items = Vec::new();
                    for _ in 0..length {
                        if let Some(item) = self.read_item(reader)? {
                            items.push(item);
                        } else {
                            self.no_errors += 1;
                            eprintln!("Error: Unexpected EOF in array");
                            break;
                        }
                    }
                    CborValue::Array(items)
                }
            }
            MAJOR_MAP => {
                if additional_info == AI_INDEFINITE {
                    // Indefinite-length map
                    let mut pairs = Vec::new();
                    while let Some(key) = self.read_item(reader)? {
                        if let CborValue::Break = key.value {
                            break;
                        }
                        if let Some(value) = self.read_item(reader)? {
                            pairs.push((key, value));
                        } else {
                            self.no_errors += 1;
                            eprintln!("Error: Missing value in map");
                            break;
                        }
                    }
                    CborValue::Map(pairs)
                } else {
                    let length = self.read_additional(reader, additional_info)? as usize;
                    let mut pairs = Vec::new();
                    for _ in 0..length {
                        if let Some(key) = self.read_item(reader)? {
                            if let Some(value) = self.read_item(reader)? {
                                pairs.push((key, value));
                            } else {
                                self.no_errors += 1;
                                eprintln!("Error: Missing value in map");
                                break;
                            }
                        } else {
                            self.no_errors += 1;
                            eprintln!("Error: Unexpected EOF in map");
                            break;
                        }
                    }
                    CborValue::Map(pairs)
                }
            }
            MAJOR_TAG => {
                let tag = self.read_additional(reader, additional_info)?;
                if let Some(tagged_item) = self.read_item(reader)? {
                    CborValue::Tag(tag, Box::new(tagged_item))
                } else {
                    self.no_errors += 1;
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Missing tagged value"));
                }
            }
            MAJOR_SIMPLE => {
                match additional_info {
                    SIMPLE_FALSE => CborValue::Boolean(false),
                    SIMPLE_TRUE => CborValue::Boolean(true),
                    SIMPLE_NULL => CborValue::Null,
                    SIMPLE_UNDEFINED => CborValue::Undefined,
                    25 => {
                        // Float16
                        let mut buf = [0u8; 2];
                        reader.read_exact(&mut buf)?;
                        self.offset += 2;
                        let val = f16_to_f32(u16::from_be_bytes(buf));
                        CborValue::Float16(val)
                    }
                    26 => {
                        // Float32
                        let mut buf = [0u8; 4];
                        reader.read_exact(&mut buf)?;
                        self.offset += 4;
                        CborValue::Float32(f32::from_be_bytes(buf))
                    }
                    27 => {
                        // Float64
                        let mut buf = [0u8; 8];
                        reader.read_exact(&mut buf)?;
                        self.offset += 8;
                        CborValue::Float64(f64::from_be_bytes(buf))
                    }
                    AI_INDEFINITE => CborValue::Break,
                    _ => {
                        if additional_info < 24 {
                            CborValue::Simple(additional_info)
                        } else {
                            let val = self.read_additional(reader, additional_info)? as u8;
                            CborValue::Simple(val)
                        }
                    }
                }
            }
            _ => {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid major type"));
            }
        };

        Ok(Some(CborItem::new(major_type, additional_info, value)))
    }

    /// Print indentation
    fn print_indent(&self, level: usize) {
        if self.config.show_offsets {
            if self.config.hex_values {
                print!("[{:04X}] ", self.offset);
            } else {
                print!("[{:4}] ", self.offset);
            }
        }

        if !self.config.compact {
            for _ in 0..level {
                print!("  ");
            }
        }
    }

    /// Print hex dump of bytes
    fn print_hex_dump(&self, bytes: &[u8], max_bytes: usize) {
        let display_bytes = bytes.len().min(max_bytes);

        for (i, byte) in bytes.iter().take(display_bytes).enumerate() {
            if i > 0 && i % 16 == 0 {
                print!("\n    ");
            }
            print!("{:02X} ", byte);
        }

        if bytes.len() > display_bytes {
            print!("\n    ... ({} more bytes)", bytes.len() - display_bytes);
        }
    }

    /// Print a CBOR item
    fn print_item(&mut self, item: &CborItem, level: usize) -> io::Result<()> {
        if level > self.config.max_nest_level {
            self.print_indent(level);
            println!("<max nesting level exceeded>");
            return Ok(());
        }

        self.print_indent(level);

        let type_prefix = if self.config.show_types {
            match &item.value {
                CborValue::Unsigned(_) => "unsigned",
                CborValue::Negative(_) => "negative",
                CborValue::Bytes(_) => "bytes",
                CborValue::Text(_) => "text",
                CborValue::Array(_) => "array",
                CborValue::Map(_) => "map",
                CborValue::Tag(_, _) => "tag",
                CborValue::Boolean(_) => "bool",
                CborValue::Null => "null",
                CborValue::Undefined => "undefined",
                CborValue::Float16(_) => "float16",
                CborValue::Float32(_) => "float32",
                CborValue::Float64(_) => "float64",
                _ => "",
            }
        } else {
            ""
        };

        match &item.value {
            CborValue::Unsigned(n) => {
                if self.config.show_types {
                    println!("{}({})", type_prefix, n);
                } else {
                    println!("{}", n);
                }
            }
            CborValue::Negative(n) => {
                if self.config.show_types {
                    println!("{}({})", type_prefix, n);
                } else {
                    println!("{}", n);
                }
            }
            CborValue::Bytes(bytes) => {
                if self.config.show_types {
                    println!("{}({} bytes)", type_prefix, bytes.len());
                } else {
                    println!("<{} bytes>", bytes.len());
                }
                if self.config.print_hex || bytes.len() <= 64 {
                    self.print_indent(level);
                    print!("  ");
                    let max = if self.config.print_all_data {
                        usize::MAX
                    } else {
                        self.config.max_bytes_display
                    };
                    self.print_hex_dump(bytes, max);
                    println!();
                }
            }
            CborValue::Text(s) => {
                if s.len() > 80 && !self.config.print_all_data {
                    if self.config.show_types {
                        println!("{}: \"{}...\" ({} chars total)", type_prefix, &s[..80], s.len());
                    } else {
                        println!("\"{}...\"", &s[..80]);
                    }
                } else if self.config.show_types {
                    println!("{}: \"{}\"", type_prefix, s);
                } else {
                    println!("\"{}\"", s);
                }
            }
            CborValue::Array(items) => {
                if self.config.show_types {
                    println!("{}({} items) [", type_prefix, items.len());
                } else {
                    println!("[");
                }
                for (i, sub_item) in items.iter().enumerate() {
                    self.print_item(sub_item, level + 1)?;
                    if i < items.len() - 1 && !self.config.compact {
                        self.print_indent(level + 1);
                        println!(",");
                    }
                }
                self.print_indent(level);
                println!("]");
            }
            CborValue::Map(pairs) => {
                if self.config.show_types {
                    println!("{}({} pairs) {{", type_prefix, pairs.len());
                } else {
                    println!("{{");
                }
                for (i, (key, value)) in pairs.iter().enumerate() {
                    self.print_item(key, level + 1)?;
                    self.print_indent(level + 1);
                    println!("=>");
                    self.print_item(value, level + 1)?;
                    if i < pairs.len() - 1 && !self.config.compact {
                        self.print_indent(level + 1);
                        println!(",");
                    }
                }
                self.print_indent(level);
                println!("}}");
            }
            CborValue::Tag(tag, tagged_item) => {
                if let Some(name) = self.tag_name(*tag) {
                    if self.config.show_types {
                        println!("{} {} ({}) {{", type_prefix, tag, name);
                    } else {
                        println!("tag({}) {{", name);
                    }
                } else if self.config.show_types {
                    println!("{} {} {{", type_prefix, tag);
                } else {
                    println!("tag({}) {{", tag);
                }
                self.print_item(tagged_item, level + 1)?;
                self.print_indent(level);
                println!("}}");
            }
            CborValue::Simple(n) => {
                if self.config.show_types {
                    println!("simple({})", n);
                } else {
                    println!("simple:{}", n);
                }
            }
            CborValue::Boolean(b) => {
                if self.config.show_types {
                    println!("{}: {}", type_prefix, b);
                } else {
                    println!("{}", b);
                }
            }
            CborValue::Null => {
                println!("{}", type_prefix);
            }
            CborValue::Undefined => {
                println!("{}", type_prefix);
            }
            CborValue::Float16(f) => {
                if self.config.show_types {
                    println!("{}: {}", type_prefix, f);
                } else {
                    println!("{}", f);
                }
            }
            CborValue::Float32(f) => {
                if self.config.show_types {
                    println!("{}: {}", type_prefix, f);
                } else {
                    println!("{}", f);
                }
            }
            CborValue::Float64(f) => {
                if self.config.show_types {
                    println!("{}: {}", type_prefix, f);
                } else {
                    println!("{}", f);
                }
            }
            CborValue::Break => {
                println!("break");
            }
        }

        Ok(())
    }

    /// Main entry point to dump CBOR data
    fn dump_cbor<R: Read>(&mut self, reader: &mut R) -> io::Result<()> {
        let mut item_count = 0;

        while let Some(item) = self.read_item(reader)? {
            if item_count > 0 {
                println!();
            }
            self.print_item(&item, 0)?;
            item_count += 1;
        }

        println!("\nParsing complete. {} item(s) found.", item_count);
        if self.no_errors > 0 {
            println!("Errors: {}", self.no_errors);
        }
        if self.no_warnings > 0 {
            println!("Warnings: {}", self.no_warnings);
        }

        Ok(())
    }
}

/// Convert IEEE 754 half-precision float to single-precision
fn f16_to_f32(bits: u16) -> f32 {
    let sign = ((bits >> 15) & 1) as u32;
    let exp = ((bits >> 10) & 0x1F) as u32;
    let mant = (bits & 0x3FF) as u32;

    if exp == 0 {
        if mant == 0 {
            // Zero
            f32::from_bits(sign << 31)
        } else {
            // Subnormal
            let new_exp = 127 - 15;
            let new_mant = mant << 13;
            f32::from_bits((sign << 31) | (new_exp << 23) | new_mant)
        }
    } else if exp == 0x1F {
        if mant == 0 {
            // Infinity
            f32::from_bits((sign << 31) | (0xFF << 23))
        } else {
            // NaN
            f32::from_bits((sign << 31) | (0xFF << 23) | (mant << 13))
        }
    } else {
        // Normalized
        let new_exp = exp + 127 - 15;
        let new_mant = mant << 13;
        f32::from_bits((sign << 31) | (new_exp << 23) | new_mant)
    }
}

fn print_help(program_name: &str) {
    println!("CBOR Dumper - Rust Implementation");
    println!("Based on the concepts from dumpasn1.c by Peter Gutmann\n");
    println!("Usage: {} [OPTIONS] <input_file>", program_name);
    println!("\nDumps CBOR-encoded data (RFC 8949) in a human-readable format.\n");
    println!("OPTIONS:");
    println!("  -h, --help              Show this help message and exit");
    println!("  -a, --print-all         Print all data in long byte strings (not just first 384 bytes)");
    println!("  -c, --compact           Compact output mode with minimal whitespace");
    println!("  -f <file>               Read input from <file> (alternative to positional argument)");
    println!("  -l <level>              Maximum nesting level to display (default: 100)");
    println!("  -m <bytes>              Maximum bytes to display for byte strings (default: 384)");
    println!("  -o, --offsets           Show byte offsets for each item");
    println!("  -t, --no-types          Don't show type names, only values");
    println!("  -v, --verbose           Verbose output with extra information");
    println!("  -x, --hex               Always show hex dump for byte strings");
    println!("  --hex-offsets           Display offsets in hexadecimal instead of decimal");
    println!("  --no-decode-nested      Don't try to decode nested CBOR in byte strings");
    println!("\nEXAMPLES:");
    println!("  {} data.cbor", program_name);
    println!("  {} --hex --offsets message.cbor     # Show hex and offsets", program_name);
    println!("  {} -c -l 3 large.cbor               # Compact mode, max 3 levels deep", program_name);
    println!("\nThe input file should contain binary CBOR-encoded data.");
    println!("\nCBOR MAJOR TYPES:");
    println!("  0: Unsigned integer       4: Array");
    println!("  1: Negative integer       5: Map");
    println!("  2: Byte string            6: Tagged value");
    println!("  3: Text string (UTF-8)    7: Simple value/float");
}

fn parse_args_from(args: &[String]) -> Result<(Config, Option<String>), String> {
    if args.len() < 2 {
        return Err("No input file specified".to_string());
    }

    let mut config = Config::default();
    let mut input_file: Option<String> = None;
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];

        match arg.as_str() {
            "-h" | "--help" => {
                print_help(&args[0]);
                std::process::exit(0);
            }
            "-a" | "--print-all" => {
                config.print_all_data = true;
            }
            "-c" | "--compact" => {
                config.compact = true;
            }
            "-f" => {
                i += 1;
                if i >= args.len() {
                    return Err("Missing filename after -f".to_string());
                }
                input_file = Some(args[i].clone());
            }
            "-l" | "--max-level" => {
                i += 1;
                if i >= args.len() {
                    return Err("Missing value after -l".to_string());
                }
                config.max_nest_level = args[i].parse()
                    .map_err(|_| format!("Invalid number for max level: {}", args[i]))?;
            }
            "-m" | "--max-bytes" => {
                i += 1;
                if i >= args.len() {
                    return Err("Missing value after -m".to_string());
                }
                config.max_bytes_display = args[i].parse()
                    .map_err(|_| format!("Invalid number for max bytes: {}", args[i]))?;
            }
            "-o" | "--offsets" => {
                config.show_offsets = true;
            }
            "-t" | "--no-types" => {
                config.show_types = false;
            }
            "-v" | "--verbose" => {
                config.verbose = true;
            }
            "-x" | "--hex" => {
                config.print_hex = true;
            }
            "--hex-offsets" => {
                config.hex_values = true;
            }
            "--no-decode-nested" => {
                config.decode_nested = false;
            }
            _ => {
                if arg.starts_with('-') {
                    return Err(format!("Unknown option: {}", arg));
                }
                // Positional argument - input file
                if let Some(existing) = &input_file {
                    return Err(format!("Multiple input files specified: {} and {}",
                                      existing, arg));
                } else {
                    input_file = Some(arg.clone());
                }
            }
        }
        i += 1;
    }

    Ok((config, input_file))
}

fn parse_args() -> Result<(Config, Option<String>), String> {
    let args: Vec<String> = env::args().collect();
    parse_args_from(&args)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(slice: &[&str]) -> Vec<String> {
        slice.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_parse_single_input_file() {
        let result = parse_args_from(&args(&["dumpcbor", "input.cbor"]));
        let (_, file) = result.expect("should succeed");
        assert_eq!(file, Some("input.cbor".to_string()));
    }

    #[test]
    fn test_parse_multiple_input_files_errors() {
        let result = parse_args_from(&args(&["dumpcbor", "first.cbor", "second.cbor"]));
        let err = result.expect_err("should fail with multiple files");
        assert!(
            err.contains("Multiple input files specified"),
            "unexpected error message: {err}"
        );
        assert!(err.contains("first.cbor"), "error should name the first file: {err}");
        assert!(err.contains("second.cbor"), "error should name the second file: {err}");
    }

    #[test]
    fn test_parse_no_args_errors() {
        let result = parse_args_from(&args(&["dumpcbor"]));
        let err = result.expect_err("should fail with no args");
        assert!(err.contains("No input file specified"), "unexpected error: {err}");
    }

    #[test]
    fn test_parse_flags_with_file() {
        let result = parse_args_from(&args(&["dumpcbor", "-v", "--print-all", "input.cbor"]));
        let (config, file) = result.expect("should succeed");
        assert!(config.verbose);
        assert!(config.print_all_data);
        assert_eq!(file, Some("input.cbor".to_string()));
    }

    #[test]
    fn test_parse_f_flag_sets_input_file() {
        let result = parse_args_from(&args(&["dumpcbor", "-f", "via_flag.cbor"]));
        let (_, file) = result.expect("should succeed");
        assert_eq!(file, Some("via_flag.cbor".to_string()));
    }

    #[test]
    fn test_parse_unknown_option_errors() {
        let result = parse_args_from(&args(&["dumpcbor", "--unknown"]));
        let err = result.expect_err("should fail on unknown option");
        assert!(err.contains("Unknown option"), "unexpected error: {err}");
    }
}

fn main() -> io::Result<()> {
    let (config, filename) = match parse_args() {
        Ok((cfg, file)) => (cfg, file),
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("\nUse --help for usage information");
            std::process::exit(1);
        }
    };

    let filename = match filename {
        Some(f) => f,
        None => {
            eprintln!("Error: No input file specified");
            eprintln!("\nUse --help for usage information");
            std::process::exit(1);
        }
    };

    let file = File::open(&filename).map_err(|e| {
        eprintln!("Error opening file '{}': {}", filename, e);
        e
    })?;
    let mut reader = BufReader::new(file);

    let mut dumper = CborDumper::new(config);

    if dumper.config.verbose {
        println!("Dumping CBOR file: {}", filename);
        println!("Configuration:");
        println!("  Print all data: {}", dumper.config.print_all_data);
        println!("  Show hex: {}", dumper.config.print_hex);
        println!("  Show offsets: {}", dumper.config.show_offsets);
        println!("  Max nesting level: {}", dumper.config.max_nest_level);
        println!("  Max bytes display: {}", dumper.config.max_bytes_display);
        println!();
    } else if !dumper.config.compact {
        println!("Dumping CBOR file: {}\n", filename);
    }

    dumper.dump_cbor(&mut reader)?;

    Ok(())
}

