use num::{BigUint, Integer, ToPrimitive, Zero};

use super::hash256::hash256;

const BASE58_ALPHABET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

pub fn encode_base58(bytes: &[u8]) -> String {
    let mut result = String::new();
    let mut leading_zeros = 0;
    // The purpose of this loop is to determine how many of the bytes at the
    // front are 0 bytes. We want to add them back at the end.
    for byte in bytes {
        if *byte == 0 {
            leading_zeros += 1;
        } else {
            break;
        }
    }
    let mut num = BigUint::from_bytes_be(bytes);
    // This is the loop that figures out what Base58 digit to use.
    while num > BigUint::zero() {
        let (div, rem) = num.div_rem(&BigUint::from(58u8));
        num = div;
        result.push(
            BASE58_ALPHABET
                .chars()
                .nth(rem.to_u8().unwrap() as usize)
                .unwrap(),
        );
    }
    for _ in 0..leading_zeros {
        result.push(BASE58_ALPHABET.chars().nth(0).unwrap());
    }
    result.chars().rev().collect()
}

/// Encode bytes with a checksum
pub fn encode_base58_checksum(bytes: &[u8]) -> String {
    let mut result = bytes.to_vec();
    let hash = hash256(bytes);
    result.extend_from_slice(&hash[0..4]);
    encode_base58(&result)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_encode_base58() {
        let values = vec![
            (
                "7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d",
                "9MA8fRQrT4u8Zj8ZRd6MAiiyaxb2Y1CMpvVkHQu5hVM6",
            ),
            (
                "eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c",
                "4fE3H2E6XMp4SsxtwinF7w9a34ooUrwWe4WsW1458Pd",
            ),
            (
                "c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6",
                "EQJsjkd6JaGwxrjEhfeqPenqHwrBmPQZjJGNSCHBkcF7",
            ),
        ];

        for (value, expected) in values {
            let value: Vec<u8> = hex::decode(value).unwrap();
            let result = encode_base58(&value);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_encode_base58_checksum() {
        let value = "7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d";
        let value: Vec<u8> = hex::decode(value).unwrap();

        assert_eq!(
            encode_base58_checksum(&value),
            "wdA2ffYs5cudrdkhFm5Ym94AuLvavacapuDBL2CAcvqYPkcvi"
        );
    }
}
