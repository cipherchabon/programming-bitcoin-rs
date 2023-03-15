use sha2::{Digest, Sha256};

/// Two rounds of sha256
pub fn hash256(bytes: &[u8]) -> [u8; 32] {
    let hash = Sha256::digest(&Sha256::digest(bytes));
    let mut result: [u8; 32] = [0; 32];
    result.copy_from_slice(&hash);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash256() {
        let value = "7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d";
        let value: Vec<u8> = hex::decode(value).unwrap();
        let result = hash256(&value);
        assert_eq!(
            hex::encode(result),
            "ea5ea40fc1b5dd6002295bf06cae70663742a5d12f5760c27f4f9339aa28e442"
        );
    }
}
