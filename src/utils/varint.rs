/// Varint is shorthand for variable integer, which is a way to encode an
/// integer into bytes that range from 0 to 2^64 – 1.
use std::io::{Cursor, Read, Write};

/// Reads a varint from a cursor
pub fn read_varint(cursor: &mut Cursor<Vec<u8>>) -> Result<u64, std::io::Error> {
    let mut buf = [0; 1];
    cursor.read_exact(&mut buf)?;
    let i = buf[0];

    match i {
        // 0xfd means the next two bytes are the number
        0xfd => {
            let mut buf = [0; 2];
            cursor.read_exact(&mut buf)?;
            Ok(u64::from_le_bytes([buf[0], buf[1], 0, 0, 0, 0, 0, 0]))
        }
        // 0xfe means the next four bytes are the number
        0xfe => {
            let mut buf = [0; 4];
            cursor.read_exact(&mut buf)?;
            Ok(u64::from_le_bytes([
                buf[0], buf[1], buf[2], buf[3], 0, 0, 0, 0,
            ]))
        }
        // 0xff means the next eight bytes are the number
        0xff => {
            let mut buf = [0; 8];
            cursor.read_exact(&mut buf)?;
            Ok(u64::from_le_bytes(buf))
        }
        // anything else is just the integer
        _ => Ok(u64::from(i)),
    }
}

/// Encodes a u64 into a varint
pub fn encode_varint(i: u64) -> Result<Vec<u8>, std::io::Error> {
    let mut buffer = Vec::new();

    if i < 0xfd {
        // If the number is below 253, encode that number as a single byte (e.g.,
        // 100 → 0x64).
        buffer.write_all(&[i as u8])?;
    } else if i >= 0xfd && i <= 0xffff {
        // If the number is between 253 and 2^16 – 1, start with the 253 byte (fd)
        // and then encode the number in 2 bytes in little-endian (e.g., 255 →
        // 0xfdff00, 555 → 0xfd2b02).
        buffer.write_all(&[0xfd])?;
        buffer.write_all(&(i as u16).to_le_bytes())?;
    } else if i >= 0x10000 && i <= 0xffffffff {
        // If the number is between 2^16 and 2^32 – 1, start with the 254 byte (fe)
        // and then encode the number in 4 bytes in little-endian (e.g., 70015 →
        // 0xfe7f110100).
        buffer.write_all(&[0xfe])?;
        buffer.write_all(&(i as u32).to_le_bytes())?;
    } else if i >= 0x100000000 && i <= (u64::MAX - 1) {
        // If the number is between 2^32 and 2^64 – 1, start with the 255 byte (ff)
        // and then encode the number in 8 bytes in little-endian (e.g.,
        // 18005558675309 → 0xff6dc7ed3e60100000)
        buffer.write_all(&[0xff])?;
        buffer.write_all(&(i as u64).to_le_bytes())?;
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Integer too large",
        ));
    }

    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_read_varint() {
        let test_cases = vec![
            (vec![0x01], 1),
            (vec![0xfd, 0x02, 0x00], 2),
            (vec![0xfe, 0x03, 0x00, 0x00, 0x00], 3),
            (
                vec![0xff, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
                4,
            ),
            (
                vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
                18446744073709551615,
            ),
        ];

        for (input, expected_output) in test_cases {
            let mut cursor = Cursor::new(input);
            let result = read_varint(&mut cursor).unwrap();
            assert_eq!(result, expected_output);
        }
    }

    #[test]
    fn test_encode_varint() {
        let test_cases = vec![
            (100, vec![0x64]),
            (255, vec![0xfd, 0xff, 0x00]),
            (555, vec![0xfd, 0x2b, 0x02]),
            (70015, vec![0xfe, 0x7f, 0x11, 0x01, 0x00]),
            (
                18005558675309,
                vec![0xff, 0x6d, 0xc7, 0xed, 0x3e, 0x60, 0x10, 0x00, 0x00],
            ),
        ];

        for (input, expected_output) in test_cases {
            let result = encode_varint(input).unwrap();
            assert_eq!(result, expected_output);
        }
    }
}
