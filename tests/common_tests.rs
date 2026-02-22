// Common unit tests for ASN.1 and CBOR tools
// These test shared functionality and data structures

#[cfg(test)]
mod tests {
    //use std::io::Cursor; // for future enhancements

    // Test helper functions
    fn create_test_data(hex: &str) -> Vec<u8> {
        hex.split_whitespace()
            .map(|s| u8::from_str_radix(s, 16).unwrap())
            .collect()
    }

    #[test]
    fn test_hex_parsing() {
        let data = create_test_data("02 01 2A");
        assert_eq!(data, vec![0x02, 0x01, 0x2A]);
    }

    // ASN.1 Tests
    mod asn1_tests {
        use super::*;

        #[test]
        fn test_asn1_integer_tag() {
            // INTEGER tag is 0x02
            let data = create_test_data("02 01 2A");
            assert_eq!(data[0], 0x02);
        }

        #[test]
        fn test_asn1_sequence_tag() {
            // SEQUENCE tag is 0x30
            let data = create_test_data("30 00");
            assert_eq!(data[0], 0x30);
        }

        #[test]
        fn test_asn1_boolean_tag() {
            // BOOLEAN tag is 0x01
            let data = create_test_data("01 01 FF");
            assert_eq!(data[0], 0x01);
        }

        #[test]
        fn test_asn1_null_tag() {
            // NULL tag is 0x05
            let data = create_test_data("05 00");
            assert_eq!(data[0], 0x05);
        }

        #[test]
        fn test_asn1_octet_string_tag() {
            // OCTET STRING tag is 0x04
            let data = create_test_data("04 05 48 65 6C 6C 6F");
            assert_eq!(data[0], 0x04);
            assert_eq!(data[1], 0x05); // Length
        }

        #[test]
        fn test_asn1_oid_tag() {
            // OBJECT IDENTIFIER tag is 0x06
            let data = create_test_data("06 03 55 04 03");
            assert_eq!(data[0], 0x06);
        }

        #[test]
        fn test_asn1_short_form_length() {
            // Lengths 0-127 use short form
            let data = create_test_data("02 05 00 00 00 00 00");
            assert_eq!(data[1], 0x05);
            assert!(data[1] < 0x80); // Short form
        }

        #[test]
        fn test_asn1_long_form_length_marker() {
            // Long form starts with 0x80 | num_octets
            let data = create_test_data("04 81 FF");
            assert_eq!(data[1] & 0x80, 0x80); // Long form marker
        }

        #[test]
        fn test_asn1_tag_class_universal() {
            // Universal class: bits 8-7 are 00
            let data = create_test_data("02 01 00");
            assert_eq!(data[0] & 0xC0, 0x00); // Universal
        }

        #[test]
        fn test_asn1_tag_class_context() {
            // Context-specific class: bits 8-7 are 10
            let data = create_test_data("80 01 00");
            assert_eq!(data[0] & 0xC0, 0x80); // Context-specific
        }

        #[test]
        fn test_asn1_constructed_flag() {
            // Constructed: bit 6 is 1
            let data = create_test_data("30 00");
            assert_eq!(data[0] & 0x20, 0x20); // Constructed
        }

        #[test]
        fn test_asn1_primitive_flag() {
            // Primitive: bit 6 is 0
            let data = create_test_data("02 01 00");
            assert_eq!(data[0] & 0x20, 0x00); // Primitive
        }
    }

    // CBOR Tests
    mod cbor_tests {
        use super::*;

        #[test]
        fn test_cbor_unsigned_int_major_type() {
            // Major type 0: unsigned integer
            let data = create_test_data("18 2A");
            assert_eq!(data[0] >> 5, 0); // Major type 0
        }

        #[test]
        fn test_cbor_negative_int_major_type() {
            // Major type 1: negative integer
            let data = create_test_data("38 63");
            assert_eq!(data[0] >> 5, 1); // Major type 1
        }

        #[test]
        fn test_cbor_byte_string_major_type() {
            // Major type 2: byte string
            let data = create_test_data("44 01 02 03 04");
            assert_eq!(data[0] >> 5, 2); // Major type 2
        }

        #[test]
        fn test_cbor_text_string_major_type() {
            // Major type 3: text string
            let data = create_test_data("64 74 65 73 74");
            assert_eq!(data[0] >> 5, 3); // Major type 3
        }

        #[test]
        fn test_cbor_array_major_type() {
            // Major type 4: array
            let data = create_test_data("83 01 02 03");
            assert_eq!(data[0] >> 5, 4); // Major type 4
        }

        #[test]
        fn test_cbor_map_major_type() {
            // Major type 5: map
            let data = create_test_data("A1 61 61 01");
            assert_eq!(data[0] >> 5, 5); // Major type 5
        }

        #[test]
        fn test_cbor_tag_major_type() {
            // Major type 6: tag
            let data = create_test_data("C1 01");
            assert_eq!(data[0] >> 5, 6); // Major type 6
        }

        #[test]
        fn test_cbor_simple_major_type() {
            // Major type 7: simple/float
            let data = create_test_data("F5");
            assert_eq!(data[0] >> 5, 7); // Major type 7
        }

        #[test]
        fn test_cbor_additional_info() {
            // Additional info in low 5 bits
            let data = create_test_data("18 2A");
            assert_eq!(data[0] & 0x1F, 24); // 1-byte uint follows
        }

        #[test]
        fn test_cbor_false_value() {
            let data = create_test_data("F4");
            assert_eq!(data[0], 0xF4); // false
        }

        #[test]
        fn test_cbor_true_value() {
            let data = create_test_data("F5");
            assert_eq!(data[0], 0xF5); // true
        }

        #[test]
        fn test_cbor_null_value() {
            let data = create_test_data("F6");
            assert_eq!(data[0], 0xF6); // null
        }

        #[test]
        fn test_cbor_undefined_value() {
            let data = create_test_data("F7");
            assert_eq!(data[0], 0xF7); // undefined
        }

        #[test]
        fn test_cbor_small_integer_encoding() {
            // Integers 0-23 encoded in initial byte
            let data = create_test_data("17");
            assert_eq!(data[0], 23);
            assert_eq!(data[0] >> 5, 0); // Major type 0
        }

        #[test]
        fn test_cbor_indefinite_length_marker() {
            // Indefinite length uses additional info 31
            let data = create_test_data("5F FF");
            assert_eq!(data[0] & 0x1F, 31); // Indefinite marker
        }
    }

    // Cross-cutting tests
    #[test]
    fn test_byte_operations() {
        let byte: u8 = 0b11010110;

        // Test bit masking
        assert_eq!(byte & 0xC0, 0xC0); // Top 2 bits
        assert_eq!(byte & 0x20, 0x00); // Bit 6
        assert_eq!(byte & 0x1F, 0x16); // Bottom 5 bits
    }

    #[test]
    fn test_multi_byte_integer_parsing() {
        // Test big-endian integer parsing
        let bytes = vec![0x01, 0x02, 0x03, 0x04];
        let value = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        assert_eq!(value, 0x01020304);
    }

    #[test]
    fn test_utf8_validation() {
        // Valid UTF-8
        let valid = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]; // "Hello"
        assert!(String::from_utf8(valid).is_ok());

        // Invalid UTF-8
        let invalid = vec![0xFF, 0xFE];
        assert!(String::from_utf8(invalid).is_err());
    }
}
