const HEADER: [u8; 22] = [
    0, 1, 0, 0, 0, 255, 255, 255, 255, 1, 0, 0, 0, 0, 0, 0, 0, 6, 1, 0, 0, 0,
];
const TRAILING_BYTE: u8 = 11;

/// Adds MS-NRBF Header and a 7-bit encoded length-prefixed string wrapping the bytes.
pub fn add_header(bytes: &[u8]) -> Vec<u8> {
    let mut length_data = [0u8; 5];
    let encoded_len = encode_leb128(bytes.len(), &mut length_data);

    let mut new_bytes = Vec::with_capacity(HEADER.len() + encoded_len + bytes.len() + 1);

    new_bytes.extend_from_slice(&HEADER);
    new_bytes.extend_from_slice(&length_data[..encoded_len]);
    new_bytes.extend_from_slice(bytes);
    new_bytes.push(TRAILING_BYTE);

    new_bytes
}

/// Safely removes the MS-NRBF Header and parses the 7-bit length prefix.
/// Returns None if the payload is malformed or truncated instead of panicking.
pub fn remove_header(bytes: &[u8]) -> Option<Vec<u8>> {
    if bytes.len() < HEADER.len() + 2
        || !bytes.starts_with(&HEADER)
        || *bytes.last()? != TRAILING_BYTE
    {
        return None;
    }

    let payload = &bytes[HEADER.len()..bytes.len() - 1];

    let mut value: usize = 0;
    let mut shift = 0;
    let mut length_count = 0;

    for &byte in payload.iter().take(5) {
        length_count += 1;
        value |= ((byte & 0x7F) as usize) << shift;
        shift += 7;

        if (byte & 0x80) == 0 {
            let string_bytes = payload.get(length_count..length_count + value)?;
            return Some(string_bytes.to_vec());
        }
    }

    None // Length prefix was longer than 5 bytes (Malformed MS-NRBF)
}

/// Standardized high-performance 7-bit encoded unsigned integer write (LEB128)
/// Writes into a stack buffer to bypass heap allocations entirely. Returns bytes written.
fn encode_leb128(mut val: usize, buf: &mut [u8; 5]) -> usize {
    val = val.min(0x7FFFFFFF); // Enforce MS-NRBF string length ceiling
    let mut idx = 0;

    loop {
        let byte = (val & 0x7F) as u8;
        val >>= 7;
        if val == 0 {
            buf[idx] = byte;
            return idx + 1;
        } else {
            buf[idx] = byte | 0x80;
            idx += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leb128_max_value_test() {
        let mut buf = [0u8; 5];
        let bytes_written = encode_leb128(2147483647, &mut buf);
        assert_eq!(&buf[..bytes_written], &[255, 255, 255, 255, 7]);
    }

    #[test]
    fn structural_malformed_test() {
        // Passing garbage arrays should return None safely instead of throwing panic
        assert_eq!(remove_header(&[0, 1, 2]), None);
        assert_eq!(remove_header(&HEADER), None);
    }

    #[test]
    fn reversible_test() {
        let original_bytes = "Reversible Test".as_bytes();
        let bytes_with_headers = add_header(original_bytes);
        let bytes_without_headers = remove_header(&bytes_with_headers).unwrap();

        assert_eq!(original_bytes, bytes_without_headers);
    }
}
