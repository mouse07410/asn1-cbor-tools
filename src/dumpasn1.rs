// ASN.1 DER dumper in Rust
// Based on dumpasn1.c by Peter Gutmann
// This is a translation of the core concepts and approach to Rust

use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read, Seek};

// Constants for ASN.1 tag classes
const CLASS_MASK: u8 = 0xC0;
const UNIVERSAL: u8 = 0x00;
const APPLICATION: u8 = 0x40;
const CONTEXT: u8 = 0x80;
const PRIVATE: u8 = 0xC0;

// Constants for encoding type
const FORM_MASK: u8 = 0x20;
const CONSTRUCTED: u8 = 0x20;

// Universal tag values
const TAG_MASK: u8 = 0x1F;
const EOC: u8 = 0x00;
const BOOLEAN: u8 = 0x01;
const INTEGER: u8 = 0x02;
const BITSTRING: u8 = 0x03;
const OCTETSTRING: u8 = 0x04;
const NULLTAG: u8 = 0x05;
const OID: u8 = 0x06;
const OBJDESCRIPTOR: u8 = 0x07;
const EXTERNAL: u8 = 0x08;
const REAL: u8 = 0x09;
const ENUMERATED: u8 = 0x0A;
const EMBEDDED_PDV: u8 = 0x0B;
const UTF8STRING: u8 = 0x0C;
const SEQUENCE: u8 = 0x10;
const SET: u8 = 0x11;
const NUMERICSTRING: u8 = 0x12;
const PRINTABLESTRING: u8 = 0x13;
const T61STRING: u8 = 0x14;
const VIDEOTEXSTRING: u8 = 0x15;
const IA5STRING: u8 = 0x16;
const UTCTIME: u8 = 0x17;
const GENERALIZEDTIME: u8 = 0x18;
const GRAPHICSTRING: u8 = 0x19;
const VISIBLESTRING: u8 = 0x1A;
const GENERALSTRING: u8 = 0x1B;
const UNIVERSALSTRING: u8 = 0x1C;
const BMPSTRING: u8 = 0x1E;

// Length encoding
const LEN_XTND: u8 = 0x80;
const LEN_MASK: u8 = 0x7F;

/// Structure to hold information about an ASN.1 item
#[derive(Debug, Clone)]
struct Asn1Item {
    id: u8,              // Tag class + primitive/constructed
    tag: u8,             // Tag number
    length: i64,         // Data length
    indefinite: bool,    // Item has indefinite length
    non_canonical: bool, // Non-canonical length encoding used
    header: Vec<u8>,     // Tag+length data
    header_size: usize,  // Size of tag+length
}

impl Asn1Item {
    fn new() -> Self {
        Asn1Item {
            id: 0,
            tag: 0,
            length: 0,
            indefinite: false,
            non_canonical: false,
            header: Vec::new(),
            header_size: 0,
        }
    }
}

/// Configuration options for the dumper
#[derive(Debug, Clone)]
struct Config {
    print_dots: bool,
    do_pure: bool,
    dump_header: u8,
    extra_oid_info: bool,
    do_hex_values: bool,
    zero_length_allowed: bool,
    dump_text: bool,
    print_all_data: bool,
    check_encaps: bool,
    check_charset: bool,
    raw_time_string: bool,
    shallow_indent: bool,
    output_width: usize,
    max_nest_level: usize,
    do_outline_only: bool,
    verbose: bool,
    print_offset: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            print_dots: false,
            do_pure: false,
            dump_header: 0,
            extra_oid_info: false,
            do_hex_values: false,
            zero_length_allowed: false,
            dump_text: false,
            print_all_data: false,
            check_encaps: true,
            check_charset: true,
            raw_time_string: false,
            shallow_indent: false,
            output_width: 80,
            max_nest_level: 100,
            do_outline_only: false,
            verbose: false,
            print_offset: true,
        }
    }
}

/// Main dumper state
struct Asn1Dumper {
    config: Config,
    no_errors: usize,
    no_warnings: usize,
    f_pos: usize,
}

impl Asn1Dumper {
    fn new(config: Config) -> Self {
        Asn1Dumper {
            config,
            no_errors: 0,
            no_warnings: 0,
            f_pos: 0,
        }
    }

    /// Get descriptive string for universal tags
    fn tag_name(&self, tag: u8) -> &'static str {
        match tag {
            EOC => "End-of-contents octets",
            BOOLEAN => "BOOLEAN",
            INTEGER => "INTEGER",
            BITSTRING => "BIT STRING",
            OCTETSTRING => "OCTET STRING",
            NULLTAG => "NULL",
            OID => "OBJECT IDENTIFIER",
            OBJDESCRIPTOR => "ObjectDescriptor",
            EXTERNAL => "EXTERNAL",
            REAL => "REAL",
            ENUMERATED => "ENUMERATED",
            EMBEDDED_PDV => "EMBEDDED PDV",
            UTF8STRING => "UTF8String",
            SEQUENCE => "SEQUENCE",
            SET => "SET",
            NUMERICSTRING => "NumericString",
            PRINTABLESTRING => "PrintableString",
            T61STRING => "TeletexString",
            VIDEOTEXSTRING => "VideotexString",
            IA5STRING => "IA5String",
            UTCTIME => "UTCTime",
            GENERALIZEDTIME => "GeneralizedTime",
            GRAPHICSTRING => "GraphicString",
            VISIBLESTRING => "VisibleString",
            GENERALSTRING => "GeneralString",
            UNIVERSALSTRING => "UniversalString",
            BMPSTRING => "BMPString",
            _ => "Unknown",
        }
    }

    /// Read an ASN.1 item (tag + length)
    fn get_item<R: Read>(&mut self, reader: &mut R) -> io::Result<Option<Asn1Item>> {
        let mut item = Asn1Item::new();
        let mut header = Vec::new();

        // Read tag byte
        let mut tag_byte = [0u8; 1];
        if reader.read(&mut tag_byte)? == 0 {
            return Ok(None); // EOF
        }

        let tag = tag_byte[0];
        header.push(tag);
        item.id = tag & !TAG_MASK;
        let mut tag_num = (tag & TAG_MASK) as u32;

        // Handle long form tag
        if tag_num == TAG_MASK as u32 {
            tag_num = 0;
            loop {
                let mut byte = [0u8; 1];
                reader.read_exact(&mut byte)?;
                header.push(byte[0]);
                tag_num = (tag_num << 7) | ((byte[0] & 0x7F) as u32);
                self.f_pos += 1;

                if (byte[0] & LEN_XTND) == 0 || header.len() >= 5 {
                    break;
                }
            }
        }

        item.tag = tag_num as u8;

        // Read length byte
        let mut len_byte = [0u8; 1];
        reader.read_exact(&mut len_byte)?;
        header.push(len_byte[0]);
        self.f_pos += 2; // Tag + length byte

        let length = len_byte[0];

        if (length & LEN_XTND) != 0 {
            // Long form or indefinite length
            let num_octets = (length & LEN_MASK) as usize;

            if num_octets == 0 {
                // Indefinite length
                item.indefinite = true;
                item.length = 0;
            } else if num_octets > 4 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Length too long",
                ));
            } else {
                // Definite long form
                item.length = 0;
                for _ in 0..num_octets {
                    let mut byte = [0u8; 1];
                    reader.read_exact(&mut byte)?;
                    header.push(byte[0]);
                    item.length = (item.length << 8) | (byte[0] as i64);
                }
                self.f_pos += num_octets;

                // Check for non-canonical encoding
                if item.length < 128 {
                    item.non_canonical = true;
                }
            }
        } else {
            // Short form length
            item.length = length as i64;
        }

        item.header = header;
        item.header_size = item.header.len();

        Ok(Some(item))
    }

    /// Print indentation
    fn print_indent(&self, level: usize) {
        if !self.config.do_pure && self.config.print_offset {
            if self.config.do_hex_values {
                print!("{:04X} {:04X}: ", self.f_pos, 0);
            } else {
                print!("{:4} {:4}: ", self.f_pos, 0);
            }
        }

        for _ in 0..level {
            if self.config.shallow_indent {
                print!(" ");
            } else {
                print!("  ");
            }
        }

        if self.config.print_dots && level > 0 {
            print!(". ");
        }
    }

    /// Print hex dump of data
    fn dump_hex<R: Read>(&mut self, reader: &mut R, length: i64, level: usize) -> io::Result<()> {
        let bytes_to_read = length.min(if self.config.print_all_data {
            length
        } else {
            384
        });
        let mut buffer = vec![0u8; bytes_to_read as usize];
        reader.read_exact(&mut buffer)?;

        print!(" ");
        for (i, byte) in buffer.iter().enumerate() {
            if i > 0 && i % 16 == 0 {
                println!();
                self.print_indent(level);
                print!("  ");
            }
            print!("{:02X} ", byte);
        }

        if length > bytes_to_read && !self.config.print_all_data {
            println!("\n  ... ({} more bytes)", length - bytes_to_read);
            // Skip remaining bytes
            let mut remaining = vec![0u8; (length - bytes_to_read) as usize];
            reader.read_exact(&mut remaining)?;
        }

        self.f_pos += length as usize;
        println!();
        Ok(())
    }

    /// Print string data
    fn print_string<R: Read>(
        &mut self,
        reader: &mut R,
        length: i64,
        _level: usize,
    ) -> io::Result<()> {
        let bytes_to_read = length.min(if self.config.print_all_data {
            length
        } else {
            384
        });
        let mut buffer = vec![0u8; bytes_to_read as usize];
        reader.read_exact(&mut buffer)?;

        print!(" '");
        for byte in &buffer {
            let ch = *byte as char;
            if ch.is_ascii() && !ch.is_control() {
                print!("{}", ch);
            } else {
                print!(".");
            }
        }
        print!("'");

        if length > bytes_to_read && !self.config.print_all_data {
            println!("\n  ... ({} more bytes)", length - bytes_to_read);
            // Skip remaining bytes
            let mut remaining = vec![0u8; (length - bytes_to_read) as usize];
            reader.read_exact(&mut remaining)?;
        }

        self.f_pos += length as usize;
        println!();
        Ok(())
    }

    /// Print integer value
    fn print_integer<R: Read>(
        &mut self,
        reader: &mut R,
        length: i64,
        level: usize,
    ) -> io::Result<()> {
        if length > 8 {
            // Too large for native integer, print as hex
            self.dump_hex(reader, length, level)
        } else {
            let mut buffer = vec![0u8; length as usize];
            reader.read_exact(&mut buffer)?;

            // Convert to signed integer
            let mut value: i64 = 0;
            let is_negative = (buffer[0] & 0x80) != 0;

            for byte in &buffer {
                value = (value << 8) | (*byte as i64);
            }

            // Handle sign extension for negative numbers
            if is_negative {
                let shift = (8 - length) * 8;
                value = (value << shift) >> shift;
            }

            println!(" {}", value);
            self.f_pos += length as usize;
            Ok(())
        }
    }

    /// Print OID
    fn print_oid<R: Read>(&mut self, reader: &mut R, length: i64, _level: usize) -> io::Result<()> {
        let mut buffer = vec![0u8; length as usize];
        reader.read_exact(&mut buffer)?;

        if buffer.is_empty() {
            println!(" (empty)");
            return Ok(());
        }

        print!(" ");

        // First byte encodes first two components
        let first = buffer[0] / 40;
        let second = buffer[0] % 40;
        print!("{}.{}", first, second);

        // Decode remaining components
        let mut i = 1;
        while i < buffer.len() {
            let mut value: u64 = 0;
            loop {
                if i >= buffer.len() {
                    break;
                }
                let byte = buffer[i];
                i += 1;
                value = (value << 7) | ((byte & 0x7F) as u64);
                if (byte & 0x80) == 0 {
                    break;
                }
            }
            print!(".{}", value);
        }

        println!();
        self.f_pos += length as usize;
        Ok(())
    }

    /// Print a constructed object
    fn print_constructed<R: Read + Seek>(
        &mut self,
        reader: &mut R,
        level: usize,
        item: &Asn1Item,
    ) -> io::Result<()> {
        if item.length == 0 && !item.indefinite {
            println!(" {{}}");
            return Ok(());
        }

        println!(" {{");

        if item.indefinite {
            // Indefinite length - read until EOC
            while let Some(sub_item) = self.get_item(reader)? {
                if sub_item.tag == EOC && sub_item.length == 0 {
                    break;
                }
                self.print_asn1_object(reader, &sub_item, level + 1)?;
            }
        } else {
            // Definite length
            let end_pos = self.f_pos + item.length as usize;

            while self.f_pos < end_pos {
                if let Some(sub_item) = self.get_item(reader)? {
                    self.print_asn1_object(reader, &sub_item, level + 1)?;
                } else {
                    break;
                }
            }
        }

        self.print_indent(level);
        println!("}}");
        Ok(())
    }

    /// Print a single ASN.1 object
    fn print_asn1_object<R: Read + Seek>(
        &mut self,
        reader: &mut R,
        item: &Asn1Item,
        level: usize,
    ) -> io::Result<()> {
        if level > self.config.max_nest_level {
            return Ok(());
        }

        self.print_indent(level);

        // Print tag class if not UNIVERSAL
        let class = item.id & CLASS_MASK;
        if class != UNIVERSAL {
            let class_name = match class {
                APPLICATION => "APPLICATION",
                CONTEXT => "",
                PRIVATE => "PRIVATE",
                _ => "UNIVERSAL",
            };

            if !class_name.is_empty() {
                print!("[{} {}]", class_name, item.tag);
            } else {
                print!("[{}]", item.tag);
            }
        } else {
            // Universal tag
            print!("{}", self.tag_name(item.tag));
        }

        // Handle constructed vs primitive
        if (item.id & FORM_MASK) == CONSTRUCTED {
            self.print_constructed(reader, level, item)?;
        } else {
            // Primitive type
            match item.tag {
                BOOLEAN => {
                    let mut byte = [0u8; 1];
                    reader.read_exact(&mut byte)?;
                    println!(" {}", if byte[0] != 0 { "TRUE" } else { "FALSE" });
                    self.f_pos += 1;
                }
                INTEGER | ENUMERATED => {
                    self.print_integer(reader, item.length, level)?;
                }
                BITSTRING => {
                    // Read unused bits byte
                    let mut unused = [0u8; 1];
                    reader.read_exact(&mut unused)?;
                    if unused[0] != 0 {
                        print!(" ({} unused bits)", unused[0]);
                    }
                    self.f_pos += 1;
                    self.dump_hex(reader, item.length - 1, level)?;
                }
                OCTETSTRING => {
                    // Try to detect if it's text
                    if self.config.check_charset && item.length > 0 && item.length < 1024 {
                        self.print_string(reader, item.length, level)?;
                    } else {
                        self.dump_hex(reader, item.length, level)?;
                    }
                }
                NULLTAG => {
                    println!();
                }
                OID => {
                    self.print_oid(reader, item.length, level)?;
                }
                UTF8STRING | PRINTABLESTRING | IA5STRING | VISIBLESTRING | GENERALSTRING
                | NUMERICSTRING | T61STRING | VIDEOTEXSTRING => {
                    self.print_string(reader, item.length, level)?;
                }
                UTCTIME | GENERALIZEDTIME => {
                    self.print_string(reader, item.length, level)?;
                }
                BMPSTRING | UNIVERSALSTRING => {
                    self.print_string(reader, item.length, level)?;
                }
                _ => {
                    self.dump_hex(reader, item.length, level)?;
                }
            }
        }

        Ok(())
    }

    /// Main entry point to dump ASN.1 data
    fn dump_asn1<R: Read + Seek>(&mut self, reader: &mut R) -> io::Result<()> {
        while let Some(item) = self.get_item(reader)? {
            self.print_asn1_object(reader, &item, 0)?;
        }

        println!("\nParsing complete.");
        if self.no_errors > 0 {
            println!("Errors: {}", self.no_errors);
        }
        if self.no_warnings > 0 {
            println!("Warnings: {}", self.no_warnings);
        }

        Ok(())
    }
}

fn print_help(program_name: &str) {
    println!("ASN.1 DER Dumper - Rust Implementation");
    println!("Based on dumpasn1.c by Peter Gutmann\n");
    println!("Usage: {} [OPTIONS] <input_file>", program_name);
    println!("\nDumps ASN.1 DER-encoded data in a human-readable format.\n");
    println!("OPTIONS:");
    println!("  -h, --help              Show this help message and exit");
    println!(
        "  -a, --print-all         Print all data in long data blocks (not just first 384 bytes)"
    );
    println!("  -c, --no-check-charset  Don't try to interpret OCTET STRINGs as character strings");
    println!("  -d, --dump-header       Dump hex header (tag+length) before object content");
    println!("  -dd                     Dump hex header + first 24 bytes of content");
    println!(
        "  -e, --no-check-encaps   Don't try to interpret OCTET/BIT STRINGs as encapsulated data"
    );
    println!(
        "  -f <file>               Read input from <file> (alternative to positional argument)"
    );
    println!("  -i, --shallow-indent    Use shallow indenting (1 space instead of 2)");
    println!("  -l <length>             Maximum nesting level for which to display output (default: 100)");
    println!(
        "  -o, --outline           Only display constructed object outline, skip primitive content"
    );
    println!("  -p, --pure              Pure display mode: no offset/length information on left");
    println!("  -r, --raw-time          Print time values as raw string instead of formatted");
    println!("  -t, --text              Dump text alongside hex data for OCTET STRINGs");
    println!("  -v, --verbose           Verbose output with extra information");
    println!("  -w <width>              Set output width in characters (default: 80)");
    println!("  -x, --hex-values        Display size and offset in hex, not decimal");
    println!("  -z, --zero-length       Allow zero-length items (normally flagged as errors)");
    println!("  --dots                  Print dots to align columns");
    println!("  --no-offset             Don't print offset information");
    println!("  --oid-info              Print extra information about OIDs");
    println!("\nEXAMPLES:");
    println!("  {} certificate.der", program_name);
    println!(
        "  {} -p -x cert.der                # Pure mode with hex offsets",
        program_name
    );
    println!(
        "  {} --outline --max-level 5 large.der  # Show only top 5 levels",
        program_name
    );
    println!("\nThe input file should contain binary DER-encoded ASN.1 data.");
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
            "-c" | "--no-check-charset" => {
                config.check_charset = false;
            }
            "-d" | "--dump-header" => {
                config.dump_header = 1;
            }
            "-dd" => {
                config.dump_header = 2;
            }
            "-e" | "--no-check-encaps" => {
                config.check_encaps = false;
            }
            "-f" => {
                i += 1;
                if i >= args.len() {
                    return Err("Missing filename after -f".to_string());
                }
                input_file = Some(args[i].clone());
            }
            "-i" | "--shallow-indent" => {
                config.shallow_indent = true;
            }
            "-l" | "--max-level" => {
                i += 1;
                if i >= args.len() {
                    return Err("Missing value after -l".to_string());
                }
                config.max_nest_level = args[i]
                    .parse()
                    .map_err(|_| format!("Invalid number for max level: {}", args[i]))?;
            }
            "-o" | "--outline" => {
                config.do_outline_only = true;
            }
            "-p" | "--pure" => {
                config.do_pure = true;
            }
            "-r" | "--raw-time" => {
                config.raw_time_string = true;
            }
            "-t" | "--text" => {
                config.dump_text = true;
            }
            "-v" | "--verbose" => {
                config.verbose = true;
            }
            "-w" | "--width" => {
                i += 1;
                if i >= args.len() {
                    return Err("Missing value after -w".to_string());
                }
                config.output_width = args[i]
                    .parse()
                    .map_err(|_| format!("Invalid number for width: {}", args[i]))?;
            }
            "-x" | "--hex-values" => {
                config.do_hex_values = true;
            }
            "-z" | "--zero-length" => {
                config.zero_length_allowed = true;
            }
            "--dots" => {
                config.print_dots = true;
            }
            "--no-offset" => {
                config.print_offset = false;
            }
            "--oid-info" => {
                config.extra_oid_info = true;
            }
            _ => {
                if arg.starts_with('-') {
                    return Err(format!("Unknown option: {}", arg));
                }
                // Positional argument - input file
                if let Some(existing) = &input_file {
                    return Err(format!(
                        "Multiple input files specified: {} and {}",
                        existing, arg
                    ));
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

    let mut dumper = Asn1Dumper::new(config);

    if dumper.config.verbose {
        println!("Dumping ASN.1 file: {}", filename);
        println!("Configuration:");
        println!("  Print all data: {}", dumper.config.print_all_data);
        println!("  Check charset: {}", dumper.config.check_charset);
        println!("  Check encapsulation: {}", dumper.config.check_encaps);
        println!("  Max nesting level: {}", dumper.config.max_nest_level);
        println!();
    } else if !dumper.config.do_pure {
        println!("Dumping ASN.1 file: {}\n", filename);
    }

    dumper.dump_asn1(&mut reader)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(slice: &[&str]) -> Vec<String> {
        slice.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_parse_single_input_file() {
        let result = parse_args_from(&args(&["dumpasn1", "input.der"]));
        let (_, file) = result.expect("should succeed");
        assert_eq!(file, Some("input.der".to_string()));
    }

    #[test]
    fn test_parse_multiple_input_files_errors() {
        let result = parse_args_from(&args(&["dumpasn1", "first.der", "second.der"]));
        let err = result.expect_err("should fail with multiple files");
        assert!(
            err.contains("Multiple input files specified"),
            "unexpected error message: {err}"
        );
        assert!(
            err.contains("first.der"),
            "error should name the first file: {err}"
        );
        assert!(
            err.contains("second.der"),
            "error should name the second file: {err}"
        );
    }

    #[test]
    fn test_parse_no_args_errors() {
        let result = parse_args_from(&args(&["dumpasn1"]));
        let err = result.expect_err("should fail with no args");
        assert!(
            err.contains("No input file specified"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn test_parse_flags_with_file() {
        let result = parse_args_from(&args(&["dumpasn1", "-v", "--print-all", "input.der"]));
        let (config, file) = result.expect("should succeed");
        assert!(config.verbose);
        assert!(config.print_all_data);
        assert_eq!(file, Some("input.der".to_string()));
    }

    #[test]
    fn test_parse_f_flag_sets_input_file() {
        let result = parse_args_from(&args(&["dumpasn1", "-f", "via_flag.der"]));
        let (_, file) = result.expect("should succeed");
        assert_eq!(file, Some("via_flag.der".to_string()));
    }

    #[test]
    fn test_parse_unknown_option_errors() {
        let result = parse_args_from(&args(&["dumpasn1", "--unknown"]));
        let err = result.expect_err("should fail on unknown option");
        assert!(err.contains("Unknown option"), "unexpected error: {err}");
    }
}
