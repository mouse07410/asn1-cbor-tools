# Examples and Testing

## Creating Test Files

### ASN.1 Test Data

#### Simple Integer
```bash
# Using Python with pyasn1
python3 << 'EOF'
from pyasn1.codec.der import encoder
from pyasn1.type import univ
import sys

# Simple integer
integer = univ.Integer(42)
der = encoder.encode(integer)
sys.stdout.buffer.write(der)
EOF > test_integer.der

./dumpasn1 test_integer.der
```

Expected output:
```
INTEGER 42
```

#### Sequence with Multiple Types
```bash
python3 << 'EOF'
from pyasn1.codec.der import encoder
from pyasn1.type import univ, char
import sys

# Create a sequence
seq = univ.Sequence()
seq.setComponentByPosition(0, univ.Integer(123))
seq.setComponentByPosition(1, char.UTF8String('Hello'))
seq.setComponentByPosition(2, univ.Boolean(True))

der = encoder.encode(seq)
sys.stdout.buffer.write(der)
EOF > test_sequence.der

./dumpasn1 test_sequence.der
```

Expected output:
```
SEQUENCE {
  INTEGER 123
  UTF8String 'Hello'
  BOOLEAN TRUE
}
```

#### Object Identifier
```bash
python3 << 'EOF'
from pyasn1.codec.der import encoder
from pyasn1.type import univ
import sys

# OID for SHA-256: 2.16.840.1.101.3.4.2.1
oid = univ.ObjectIdentifier('2.16.840.1.101.3.4.2.1')
der = encoder.encode(oid)
sys.stdout.buffer.write(der)
EOF > test_oid.der

./dumpasn1 test_oid.der
```

Expected output:
```
OBJECT IDENTIFIER 2.16.840.1.101.3.4.2.1
```

### CBOR Test Data

#### Simple Map
```bash
python3 << 'EOF'
import cbor2
import sys

data = {
    'name': 'Alice',
    'age': 30,
    'active': True
}
cbor2.dump(data, sys.stdout.buffer)
EOF > test_map.cbor

./dumpcbor test_map.cbor
```

Expected output:
```
map(3 pairs) {
  text: "name"
  =>
  text: "Alice"
  ,
  text: "age"
  =>
  unsigned(30)
  ,
  text: "active"
  =>
  bool: true
}
```

#### Array with Mixed Types
```bash
python3 << 'EOF'
import cbor2
import sys

data = [1, "hello", True, None, 3.14]
cbor2.dump(data, sys.stdout.buffer)
EOF > test_array.cbor

./dumpcbor test_array.cbor
```

Expected output:
```
array(5 items) [
  unsigned(1)
  ,
  text: "hello"
  ,
  bool: true
  ,
  null
  ,
  float64: 3.14
]
```

#### Nested Structure
```bash
python3 << 'EOF'
import cbor2
import sys

data = {
    'user': {
        'name': 'Bob',
        'roles': ['admin', 'user']
    },
    'timestamp': 1234567890
}
cbor2.dump(data, sys.stdout.buffer)
EOF > test_nested.cbor

./dumpcbor test_nested.cbor
```

Expected output:
```
map(2 pairs) {
  text: "user"
  =>
  map(2 pairs) {
    text: "name"
    =>
    text: "Bob"
    ,
    text: "roles"
    =>
    array(2 items) [
      text: "admin"
      ,
      text: "user"
    ]
  }
  ,
  text: "timestamp"
  =>
  unsigned(1234567890)
}
```

#### Tagged Values
```bash
python3 << 'EOF'
import cbor2
import sys
from datetime import datetime

# CBOR tag 1 is epoch-based datetime
data = cbor2.CBORTag(1, 1609459200)  # 2021-01-01 00:00:00 UTC
cbor2.dump(data, sys.stdout.buffer)
EOF > test_tagged.cbor

./dumpcbor test_tagged.cbor
```

Expected output:
```
tag 1 (epoch-based date/time) {
  unsigned(1609459200)
}
```

#### Byte String
```bash
python3 << 'EOF'
import cbor2
import sys

data = b'\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0a\x0b\x0c\x0d\x0e\x0f'
cbor2.dump(data, sys.stdout.buffer)
EOF > test_bytes.cbor

./dumpcbor test_bytes.cbor
```

Expected output (without --hex):
```
bytes(16 bytes)
```

With `--hex` flag:
```
bytes(16 bytes)
  00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F
```

## Real-World Examples

### X.509 Certificate (ASN.1)
```bash
# Get a certificate from a website
echo | openssl s_client -connect google.com:443 2>/dev/null | \
    openssl x509 -outform DER > google.der

./dumpasn1 google.der
```

### JWT Payload (could be CBOR in CBOR Web Tokens)
```bash
python3 << 'EOF'
import cbor2
import sys
import time

# CWT (CBOR Web Token) payload
payload = {
    1: "example.com",  # iss (issuer)
    2: "user123",      # sub (subject) 
    4: int(time.time() + 3600),  # exp (expiration)
    6: int(time.time()),  # iat (issued at)
}
cbor2.dump(payload, sys.stdout.buffer)
EOF > jwt_payload.cbor

./dumpcbor jwt_payload.cbor
```

## Common Use Cases

### Debugging ASN.1 Structures
- X.509 certificates
- PKCS#7/CMS messages
- LDAP messages
- SNMP packets
- Kerberos tickets

### Debugging CBOR Structures
- CBOR Web Tokens (CWT)
- CoAP payloads
- COSE (CBOR Object Signing and Encryption)
- Sensor data in IoT applications
- Efficient data serialization

## Performance Testing

```bash
# Create a large ASN.1 file
python3 << 'EOF'
from pyasn1.codec.der import encoder
from pyasn1.type import univ
import sys

# Create a large sequence
seq = univ.Sequence()
for i in range(1000):
    seq.setComponentByPosition(i, univ.Integer(i))

der = encoder.encode(seq)
sys.stdout.buffer.write(der)
EOF > large_test.der

time ./dumpasn1 large_test.der > /dev/null

# Create a large CBOR file
python3 << 'EOF'
import cbor2
import sys

data = [i for i in range(1000)]
cbor2.dump(data, sys.stdout.buffer)
EOF > large_test.cbor

time ./dumpcbor large_test.cbor > /dev/null
```

## Troubleshooting

### ASN.1 Issues

**"Invalid major type" error**: The file may not be valid DER-encoded ASN.1. Try:
- Check if it's PEM-encoded (begins with `-----BEGIN`)
- Verify with: `openssl asn1parse -inform DER -in file.der`

**"Length too long" error**: The length encoding is invalid or corrupted.

### CBOR Issues

**"Invalid UTF-8" in text strings**: The CBOR encoder used may have encoded invalid UTF-8.

**"Missing value in map" error**: The CBOR data is malformed - maps must have paired keys/values.

## Dependencies for Testing

To run the test examples, you'll need:

```bash
# For ASN.1 tests
pip3 install pyasn1

# For CBOR tests
pip3 install cbor2

# For OpenSSL examples
# openssl should be pre-installed on most systems
```

## Comparing with Other Tools

```bash
# Compare ASN.1 output with OpenSSL
openssl asn1parse -inform DER -in test.der
./dumpasn1 test.der

# Compare CBOR output with cbor2
python3 -m cbor2.tool --pretty test.cbor
./dumpcbor test.cbor
```
