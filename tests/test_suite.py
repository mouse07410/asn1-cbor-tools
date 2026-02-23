#!/usr/bin/env python3
"""
Comprehensive test suite for ASN.1 and CBOR dumpers.
Generates test files and validates output.
"""

import os
import sys
import subprocess
import tempfile
import json
from pathlib import Path

try:
    from pyasn1.codec.der import encoder as asn1_encoder
    from pyasn1.type import univ, char
    PYASN1_AVAILABLE = True
except ImportError:
    PYASN1_AVAILABLE = False
    print("Warning: pyasn1 not installed. ASN.1 tests will be skipped.")
    print("Install with: pip install pyasn1")

try:
    import cbor2
    CBOR2_AVAILABLE = True
except ImportError:
    CBOR2_AVAILABLE = False
    print("Warning: cbor2 not installed. CBOR tests will be skipped.")
    print("Install with: pip install cbor2")

class Colors:
    GREEN = '\033[92m'
    RED = '\033[91m'
    YELLOW = '\033[93m'
    BLUE = '\033[94m'
    END = '\033[0m'

class TestRunner:
    def __init__(self, dumpasn1_path, dumpcbor_path):
        self.dumpasn1 = dumpasn1_path
        self.dumpcbor = dumpcbor_path
        self.passed = 0
        self.failed = 0
        self.skipped = 0

    def run_command(self, cmd, input_file):
        """Run command and return stdout, stderr, and return code"""
        try:
            result = subprocess.run(
                [cmd, input_file],
                capture_output=True,
                text=True,
                timeout=5
            )
            return result.stdout, result.stderr, result.returncode
        except subprocess.TimeoutExpired:
            return "", "Timeout", -1
        except Exception as e:
            return "", str(e), -1

    def test_asn1(self, name, data, expected_strings):
        """Test ASN.1 encoding"""
        if not PYASN1_AVAILABLE:
            print(f"{Colors.YELLOW}SKIP{Colors.END} {name} (pyasn1 not available)")
            self.skipped += 1
            return

        with tempfile.NamedTemporaryFile(mode='wb', suffix='.der', delete=False) as f:
            der = asn1_encoder.encode(data)
            f.write(der)
            temp_file = f.name

        try:
            stdout, stderr, rc = self.run_command(self.dumpasn1, temp_file)
            
            if rc != 0:
                print(f"{Colors.RED}FAIL{Colors.END} {name} - Non-zero exit code: {rc}")
                print(f"  stderr: {stderr}")
                self.failed += 1
                return

            all_found = all(s in stdout for s in expected_strings)
            if all_found:
                print(f"{Colors.GREEN}PASS{Colors.END} {name}")
                self.passed += 1
            else:
                print(f"{Colors.RED}FAIL{Colors.END} {name}")
                print(f"  Expected strings: {expected_strings}")
                print(f"  Got output:\n{stdout}")
                self.failed += 1
        finally:
            os.unlink(temp_file)

    def test_cbor(self, name, data, expected_strings):
        """Test CBOR encoding"""
        if not CBOR2_AVAILABLE:
            print(f"{Colors.YELLOW}SKIP{Colors.END} {name} (cbor2 not available)")
            self.skipped += 1
            return

        with tempfile.NamedTemporaryFile(mode='wb', suffix='.cbor', delete=False) as f:
            cbor2.dump(data, f)
            temp_file = f.name

        try:
            stdout, stderr, rc = self.run_command(self.dumpcbor, temp_file)
            
            if rc != 0:
                print(f"{Colors.RED}FAIL{Colors.END} {name} - Non-zero exit code: {rc}")
                print(f"  stderr: {stderr}")
                self.failed += 1
                return

            all_found = all(s.lower() in stdout.lower() for s in expected_strings)
            if all_found:
                print(f"{Colors.GREEN}PASS{Colors.END} {name}")
                self.passed += 1
            else:
                print(f"{Colors.RED}FAIL{Colors.END} {name}")
                print(f"  Expected strings: {expected_strings}")
                print(f"  Got output:\n{stdout}")
                self.failed += 1
        finally:
            os.unlink(temp_file)

    def run_all_tests(self):
        """Run all test suites"""
        print(f"\n{Colors.BLUE}=== ASN.1 Tests ==={Colors.END}\n")
        self.run_asn1_tests()
        
        print(f"\n{Colors.BLUE}=== CBOR Tests ==={Colors.END}\n")
        self.run_cbor_tests()
        
        print(f"\n{Colors.BLUE}=== Summary ==={Colors.END}")
        print(f"Passed:  {Colors.GREEN}{self.passed}{Colors.END}")
        print(f"Failed:  {Colors.RED}{self.failed}{Colors.END}")
        print(f"Skipped: {Colors.YELLOW}{self.skipped}{Colors.END}")
        print(f"Total:   {self.passed + self.failed + self.skipped}")
        
        return 0 if self.failed == 0 else 1

    def run_asn1_tests(self):
        """Run ASN.1 test cases"""
        if not PYASN1_AVAILABLE:
            return

        # Test 1: Simple integer
        self.test_asn1(
            "ASN.1 Integer (42)",
            univ.Integer(42),
            ["INTEGER", "42"]
        )

        # Test 2: Negative integer
        self.test_asn1(
            "ASN.1 Integer (-100)",
            univ.Integer(-100),
            ["INTEGER", "-100"]
        )

        # Test 3: Boolean true
        self.test_asn1(
            "ASN.1 Boolean (true)",
            univ.Boolean(True),
            ["BOOLEAN", "TRUE"]
        )

        # Test 4: Boolean false
        self.test_asn1(
            "ASN.1 Boolean (false)",
            univ.Boolean(False),
            ["BOOLEAN", "FALSE"]
        )

        # Test 5: NULL
        self.test_asn1(
            "ASN.1 NULL",
            univ.Null(),
            ["NULL"]
        )

        # Test 6: OCTET STRING
        self.test_asn1(
            "ASN.1 OCTET STRING",
            univ.OctetString(b'Hello, World!'),
            ["OCTET STRING", "Hello"]
        )

        # Test 7: UTF8 String
        self.test_asn1(
            "ASN.1 UTF8String",
            char.UTF8String('Testing'),
            ["UTF8String", "Testing"]
        )

        # Test 8: SEQUENCE
        seq = univ.Sequence()
        seq.setComponentByPosition(0, univ.Integer(1))
        seq.setComponentByPosition(1, univ.Integer(2))
        seq.setComponentByPosition(2, univ.Integer(3))
        self.test_asn1(
            "ASN.1 SEQUENCE",
            seq,
            ["SEQUENCE", "INTEGER"]
        )

        # Test 9: Nested SEQUENCE
        inner_seq = univ.Sequence()
        inner_seq.setComponentByPosition(0, univ.Integer(42))
        outer_seq = univ.Sequence()
        outer_seq.setComponentByPosition(0, inner_seq)
        self.test_asn1(
            "ASN.1 Nested SEQUENCE",
            outer_seq,
            ["SEQUENCE", "INTEGER"]
        )

        # Test 10: OBJECT IDENTIFIER
        self.test_asn1(
            "ASN.1 OBJECT IDENTIFIER",
            univ.ObjectIdentifier('2.5.4.3'),
            ["OBJECT IDENTIFIER", "2.5.4.3"]
        )

    def run_cbor_tests(self):
        """Run CBOR test cases"""
        if not CBOR2_AVAILABLE:
            return

        # Test 1: Unsigned integer
        self.test_cbor(
            "CBOR Unsigned Integer (42)",
            42,
            ["unsigned", "42"]
        )

        # Test 2: Negative integer
        self.test_cbor(
            "CBOR Negative Integer (-100)",
            -100,
            ["negative", "-100"]
        )

        # Test 3: Text string
        self.test_cbor(
            "CBOR Text String",
            "Hello, World!",
            ["text", "Hello"]
        )

        # Test 4: Byte string
        self.test_cbor(
            "CBOR Byte String",
            b'\x01\x02\x03\x04',
            ["bytes"]
        )

        # Test 5: Array
        self.test_cbor(
            "CBOR Array",
            [1, 2, 3, 4, 5],
            ["array", "5"]
        )

        # Test 6: Map
        self.test_cbor(
            "CBOR Map",
            {'name': 'Alice', 'age': 30},
            ["map", "name", "Alice"]
        )

        # Test 7: Nested structures
        self.test_cbor(
            "CBOR Nested Structure",
            {
                'user': {
                    'name': 'Bob',
                    'roles': ['admin', 'user']
                }
            },
            ["map", "user", "name", "Bob", "array"]
        )

        # Test 8: Boolean true
        self.test_cbor(
            "CBOR Boolean (true)",
            True,
            ["true"]
        )

        # Test 9: Boolean false
        self.test_cbor(
            "CBOR Boolean (false)",
            False,
            ["false"]
        )

        # Test 10: None/null
        self.test_cbor(
            "CBOR Null",
            None,
            ["null"]
        )

        # Test 11: Float
        self.test_cbor(
            "CBOR Float",
            3.14159,
            ["float", "3.14"]
        )

        # Test 12: Mixed array
        self.test_cbor(
            "CBOR Mixed Array",
            [1, "hello", True, None, 3.14],
            ["array", "text", "hello", "true", "null"]
        )

        # Test 13: Empty structures
        self.test_cbor(
            "CBOR Empty Array",
            [],
            ["array", "0"]
        )

        self.test_cbor(
            "CBOR Empty Map",
            {},
            ["map", "0"]
        )

def find_binaries():
    """Find the binary executables"""
    possible_paths = [
        ('target/release/dumpasn1', 'target/release/dumpcbor'),
        ('target/debug/dumpasn1', 'target/debug/dumpcbor'),
        ('./dumpasn1', './dumpcbor'),
    ]
    
    for asn1_path, cbor_path in possible_paths:
        if os.path.isfile(asn1_path) and os.path.isfile(cbor_path):
            return asn1_path, cbor_path
    
    return None, None

def main():
    print("ASN.1/CBOR Tools Test Suite")
    print("=" * 50)
    
    # Find binaries
    dumpasn1, dumpcbor = find_binaries()
    
    if not dumpasn1 or not dumpcbor:
        print(f"{Colors.RED}Error: Could not find binaries.{Colors.END}")
        print("Please run 'cargo build' first.")
        return 1
    
    print(f"Using binaries:")
    print(f"  ASN.1: {dumpasn1}")
    print(f"  CBOR:  {dumpcbor}")
    
    # Check dependencies
    if not PYASN1_AVAILABLE:
        print(f"\n{Colors.YELLOW}Warning: pyasn1 not installed{Colors.END}")
    if not CBOR2_AVAILABLE:
        print(f"{Colors.YELLOW}Warning: cbor2 not installed{Colors.END}")
    
    if not PYASN1_AVAILABLE and not CBOR2_AVAILABLE:
        print(f"\n{Colors.RED}Error: No test dependencies available.{Colors.END}")
        print("Install with: pip install pyasn1 cbor2")
        return 1
    
    # Run tests
    runner = TestRunner(dumpasn1, dumpcbor)
    return runner.run_all_tests()

if __name__ == '__main__':
    sys.exit(main())
