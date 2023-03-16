use ripemd::{Digest, Ripemd160};
use sha2::Sha256;

/// Two rounds of sha256 and ripemd160
pub fn hash160(bytes: &[u8]) -> [u8; 20] {
    let mut hasher = Ripemd160::new();
    hasher.update(Sha256::digest(bytes));
    let result = hasher.finalize();
    let mut hash: [u8; 20] = [0; 20];
    hash.copy_from_slice(&result);
    hash
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hash160() {
        let value = "7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d";
        let value: Vec<u8> = hex::decode(value).unwrap();
        let result = super::hash160(&value);
        assert_eq!(
            hex::encode(result),
            "9cb1656f99c65ce3be73ddef9041b9bbeaa42369"
        );
    }
}
