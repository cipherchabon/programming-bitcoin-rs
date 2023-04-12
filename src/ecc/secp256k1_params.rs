use num::{BigUint, Num};

use super::{element::FFElement, point::ECPoint};

// Recommended 256-bit Elliptic Curve Domain Parameters
const A: u32 = 0;
const B: u32 = 7;

// Finite field prime order
const P: &str = "fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f";

// G = (Gx, Gy)
const GX: &str = "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
const GY: &str = "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8";

const N: &str = "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141";

pub struct Secp256k1Params;

impl Secp256k1Params {
    pub fn a() -> BigUint {
        BigUint::from(A)
    }

    pub fn b() -> BigUint {
        BigUint::from(B)
    }

    pub fn p() -> BigUint {
        BigUint::from_str_radix(P, 16).unwrap()
    }

    pub fn n() -> BigUint {
        BigUint::from_str_radix(N, 16).unwrap()
    }

    pub fn g() -> ECPoint {
        let x = FFElement::new_secp256k1(&BigUint::from_str_radix(GX, 16).unwrap());
        let y = FFElement::new_secp256k1(&BigUint::from_str_radix(GY, 16).unwrap());
        ECPoint::new_secp256k1(&x, &y).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secp256k1_params() {
        assert_eq!(Secp256k1Params::a(), BigUint::from(A));
        assert_eq!(Secp256k1Params::b(), BigUint::from(B));
        assert_eq!(
            Secp256k1Params::p(),
            BigUint::from_str_radix(P, 16).unwrap()
        );
        assert_eq!(
            Secp256k1Params::n(),
            BigUint::from_str_radix(N, 16).unwrap()
        );
        assert_eq!(
            Secp256k1Params::g(),
            ECPoint::new_secp256k1(
                &FFElement::new_secp256k1(&BigUint::from_str_radix(GX, 16).unwrap()),
                &FFElement::new_secp256k1(&BigUint::from_str_radix(GY, 16).unwrap())
            )
            .unwrap()
        );
    }
}
