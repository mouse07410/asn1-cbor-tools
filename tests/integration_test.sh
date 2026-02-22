#!/bin/bash
# Integration test script for ASN.1/CBOR tools

set -e  # Exit on error

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

TESTS_PASSED=0
TESTS_FAILED=0

# Determine binary paths
if [ -f "target/release/dumpasn1" ]; then
    DUMPASN1="target/release/dumpasn1"
    DUMPCBOR="target/release/dumpcbor"
elif [ -f "target/debug/dumpasn1" ]; then
    DUMPASN1="target/debug/dumpasn1"
    DUMPCBOR="target/debug/dumpcbor"
else
    echo -e "${RED}Error: Binaries not found. Run 'cargo build' first.${NC}"
    exit 1
fi

echo "Using binaries:"
echo "  - $DUMPASN1"
echo "  - $DUMPCBOR"
echo ""

# Create temp directory
TEST_DIR=$(mktemp -d)
trap "rm -rf $TEST_DIR" EXIT

cd $TEST_DIR

test_passed() {
    echo -e "${GREEN}✓${NC} $1"
    TESTS_PASSED=$((TESTS_PASSED + 1))
}

test_failed() {
    echo -e "${RED}✗${NC} $1"
    TESTS_FAILED=$((TESTS_FAILED + 1))
}

echo "Running integration tests..."
echo ""

# Test 1: ASN.1 Integer
echo "Test 1: ASN.1 INTEGER"
echo "02 01 2A" | xxd -r -p > test_integer.der
if $DUMPASN1 test_integer.der | grep -q "INTEGER"; then
    test_passed "ASN.1 integer parsing"
else
    test_failed "ASN.1 integer parsing"
fi

# Test 2: ASN.1 SEQUENCE
echo "Test 2: ASN.1 SEQUENCE"
echo "30 06 02 01 01 02 01 02" | xxd -r -p > test_sequence.der
if $DUMPASN1 test_sequence.der | grep -q "SEQUENCE"; then
    test_passed "ASN.1 sequence parsing"
else
    test_failed "ASN.1 sequence parsing"
fi

# Test 3: ASN.1 OCTET STRING
echo "Test 3: ASN.1 OCTET STRING"
echo "04 05 48 65 6C 6C 6F" | xxd -r -p > test_octetstring.der
if $DUMPASN1 test_octetstring.der | grep -q "OCTET STRING"; then
    test_passed "ASN.1 octet string parsing"
else
    test_failed "ASN.1 octet string parsing"
fi

# Test 4: ASN.1 NULL
echo "Test 4: ASN.1 NULL"
echo "05 00" | xxd -r -p > test_null.der
if $DUMPASN1 test_null.der | grep -q "NULL"; then
    test_passed "ASN.1 null parsing"
else
    test_failed "ASN.1 null parsing"
fi

# Test 5: ASN.1 BOOLEAN
echo "Test 5: ASN.1 BOOLEAN"
echo "01 01 FF" | xxd -r -p > test_bool.der
if $DUMPASN1 test_bool.der | grep -q "BOOLEAN"; then
    test_passed "ASN.1 boolean parsing"
else
    test_failed "ASN.1 boolean parsing"
fi

# Test 6: ASN.1 OID
echo "Test 6: ASN.1 OBJECT IDENTIFIER"
echo "06 03 55 04 03" | xxd -r -p > test_oid.der
if $DUMPASN1 test_oid.der | grep -q "OBJECT IDENTIFIER"; then
    test_passed "ASN.1 OID parsing"
else
    test_failed "ASN.1 OID parsing"
fi

# Test 7: ASN.1 Help option
echo "Test 7: ASN.1 --help option"
if $DUMPASN1 --help | grep -q "Usage:"; then
    test_passed "ASN.1 help option"
else
    test_failed "ASN.1 help option"
fi

# Test 8: ASN.1 Invalid file
echo "Test 8: ASN.1 error handling (invalid file)"
if $DUMPASN1 nonexistent_file.der 2>&1 | grep -qi "error"; then
    test_passed "ASN.1 error handling"
else
    test_failed "ASN.1 error handling"
fi

echo ""
echo "CBOR Tests:"
echo ""

# Test 9: CBOR unsigned integer
echo "Test 9: CBOR unsigned integer"
echo "18 2A" | xxd -r -p > test_cbor_uint.cbor
if $DUMPCBOR test_cbor_uint.cbor | grep -q "unsigned\|42"; then
    test_passed "CBOR unsigned integer"
else
    test_failed "CBOR unsigned integer"
fi

# Test 10: CBOR text string
echo "Test 10: CBOR text string"
echo "65 68 65 6C 6C 6F" | xxd -r -p > test_cbor_text.cbor
if $DUMPCBOR test_cbor_text.cbor | grep -q "text\|hello"; then
    test_passed "CBOR text string"
else
    test_failed "CBOR text string"
fi

# Test 11: CBOR array
echo "Test 11: CBOR array"
echo "83 01 02 03" | xxd -r -p > test_cbor_array.cbor
if $DUMPCBOR test_cbor_array.cbor | grep -q "array"; then
    test_passed "CBOR array"
else
    test_failed "CBOR array"
fi

# Test 12: CBOR map
echo "Test 12: CBOR map"
echo "A1 61 61 01" | xxd -r -p > test_cbor_map.cbor
if $DUMPCBOR test_cbor_map.cbor | grep -q "map"; then
    test_passed "CBOR map"
else
    test_failed "CBOR map"
fi

# Test 13: CBOR boolean true
echo "Test 13: CBOR boolean"
echo "F5" | xxd -r -p > test_cbor_bool.cbor
if $DUMPCBOR test_cbor_bool.cbor | grep -q "true"; then
    test_passed "CBOR boolean"
else
    test_failed "CBOR boolean"
fi

# Test 14: CBOR null
echo "Test 14: CBOR null"
echo "F6" | xxd -r -p > test_cbor_null.cbor
if $DUMPCBOR test_cbor_null.cbor | grep -q "null"; then
    test_passed "CBOR null"
else
    test_failed "CBOR null"
fi

# Test 15: CBOR Help option
echo "Test 15: CBOR --help option"
if $DUMPCBOR --help | grep -q "Usage:"; then
    test_passed "CBOR help option"
else
    test_failed "CBOR help option"
fi

# Test 16: CBOR error handling
echo "Test 16: CBOR error handling (invalid file)"
if $DUMPCBOR nonexistent_file.cbor 2>&1 | grep -qi "error"; then
    test_passed "CBOR error handling"
else
    test_failed "CBOR error handling"
fi

# Test 17: ASN.1 command line options
echo "Test 17: ASN.1 command line options"
echo "02 01 2A" | xxd -r -p > test_opts.der
if $DUMPASN1 -v test_opts.der | grep -q "Dumping\|INTEGER"; then
    test_passed "ASN.1 -v option"
else
    test_failed "ASN.1 -v option"
fi

# Test 18: CBOR command line options
echo "Test 18: CBOR command line options"
echo "18 2A" | xxd -r -p > test_cbor_opts.cbor
if $DUMPCBOR -v test_cbor_opts.cbor | grep -q "Dumping\|unsigned\|42"; then
    test_passed "CBOR -v option"
else
    test_failed "CBOR -v option"
fi

# Summary
echo ""
echo "=================================="
echo "Test Results:"
echo "  Passed: $TESTS_PASSED"
echo "  Failed: $TESTS_FAILED"
echo "=================================="

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "${GREEN}All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed!${NC}"
    exit 1
fi
