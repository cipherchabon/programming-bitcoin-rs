use num::{BigUint, Num};

use crate::elliptic_curve::{curve::EllipticCurve, element::FFElement, finite_field::FiniteField};

// secp256k1 elliptic curve parameters
const A: u32 = 0;
const B: u32 = 7;

// Finite field prime order
const P: &str = "fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f";

const N: &str = "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141";

// G = (Gx, Gy)
const Gx: &str = "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
const Gy: &str = "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Secp256k1 {
    curve: EllipticCurve,
}

impl Secp256k1 {
    pub fn new() -> Self {
        let order = BigUint::from_str_radix(P, 16).unwrap();
        let field = FiniteField::new(&order);
        let a = FFElement::new(&BigUint::from(A), &field);
        let b = FFElement::new(&BigUint::from(B), &field);
        let curve = EllipticCurve::new(a, b);

        Self { curve }
    }

    pub fn curve(&self) -> &EllipticCurve {
        &self.curve
    }
}

#[cfg(test)]
mod tests {

    use crate::elliptic_curve::point::ECPoint;

    use super::*;

    #[test]
    fn test_secp256k1_parameters() {
        let order = BigUint::from_str_radix(P, 16).unwrap();
        let field = FiniteField::new(&order);

        let x = FFElement::new(&BigUint::from_str_radix(Gx, 16).unwrap(), &field);
        let y = FFElement::new(&BigUint::from_str_radix(Gy, 16).unwrap(), &field);

        let zero = FFElement::new(&BigUint::from(0u32), &field);
        let seven = FFElement::new(&BigUint::from(7u32), &field);
        let curve = EllipticCurve::new(zero, seven);

        let g = ECPoint::new(&x, &y, &curve).unwrap();

        let n = BigUint::from_str_radix(N, 16).unwrap();

        assert_eq!(g * n, ECPoint::infinity(&curve));
    }

    #[test]
    fn test_secp256k1() {
        let secp256k1 = Secp256k1::new();
        assert_eq!(secp256k1.curve.a().num(), &BigUint::from(A));
        assert_eq!(secp256k1.curve.b().num(), &BigUint::from(B));
        assert_eq!(
            secp256k1.curve.a().field().order(),
            &BigUint::from_str_radix(P, 16).unwrap()
        );
    }
}
