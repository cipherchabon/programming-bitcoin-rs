use core::fmt;

use num::BigUint;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Signature {
    r: BigUint,
    s: BigUint,
}

impl Signature {
    pub fn new(r: &BigUint, s: &BigUint) -> Self {
        Self {
            r: r.clone(),
            s: s.clone(),
        }
    }

    pub fn r(&self) -> &BigUint {
        &self.r
    }

    pub fn s(&self) -> &BigUint {
        &self.s
    }
}

impl Signature {
    /// DER encode the signature
    pub fn der(&self) -> Vec<u8> {
        let mut rbin = self.r.to_bytes_be();

        // remove leading zeros
        while !rbin.is_empty() && rbin[0] == 0 {
            rbin.remove(0);
        }

        // if rbin has a high bit, add a \x00
        let rbin = if rbin.is_empty() || rbin[0] & 0x80 == 0x80 {
            let mut rbin = rbin.to_vec();
            rbin.insert(0, 0);
            rbin
        } else {
            rbin.to_vec()
        };

        let mut result = vec![2, rbin.len() as u8];
        result.extend_from_slice(&rbin);

        let mut sbin = self.s.to_bytes_be();

        // remove leading zeross
        while !sbin.is_empty() && sbin[0] == 0 {
            sbin.remove(0);
        }

        // if sbin has a high bit, add a \x00
        let sbin = if sbin.is_empty() || sbin[0] & 0x80 == 0x80 {
            let mut sbin = sbin.to_vec();
            sbin.insert(0, 0);
            sbin
        } else {
            sbin.to_vec()
        };

        result.extend_from_slice(&[2, sbin.len() as u8]);
        result.extend_from_slice(&sbin);

        let mut der = vec![0x30, result.len() as u8];
        der.extend_from_slice(&result);

        der
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Signature({:x},{:x})", self.r, self.s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::Num;

    #[test]
    fn test_der() {
        let r = BigUint::from_str_radix(
            "37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6",
            16,
        )
        .unwrap();

        let s = BigUint::from_str_radix(
            "8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec",
            16,
        )
        .unwrap();

        let sig = Signature::new(&r, &s);

        assert_eq!(
            sig.der(),
            hex::decode(
                "3045022037206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6\
                0221008ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec"
            )
            .unwrap()
        );
    }
}
