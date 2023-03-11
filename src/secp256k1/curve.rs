use num::{BigUint, Num};

use crate::elliptic_curve::{curve::EllipticCurve, element::FFElement, finite_field::FiniteField};

// Recommended 256-bit Elliptic Curve Domain Parameters
const A: u32 = 0;
const B: u32 = 7;

// Finite field prime order
const P: &str = "fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f";

/// Standard secp256k1 elliptic curve
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Secp256k1(EllipticCurve);

impl Secp256k1 {
    /// Create a new secp256k1 elliptic curve.
    pub fn new() -> Self {
        let order = BigUint::from_str_radix(P, 16).unwrap();
        let field = FiniteField::new(&order);
        let a = FFElement::new(&BigUint::from(A), &field);
        let b = FFElement::new(&BigUint::from(B), &field);
        let curve = EllipticCurve::new(a, b);

        Self(curve)
    }

    pub fn curve(&self) -> &EllipticCurve {
        &self.0
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_secp256k1() {
        let secp256k1 = Secp256k1::new();
        assert_eq!(secp256k1.curve().a().num(), &BigUint::from(A));
        assert_eq!(secp256k1.curve().b().num(), &BigUint::from(B));
        assert_eq!(
            secp256k1.curve().a().field().order(),
            &BigUint::from_str_radix(P, 16).unwrap()
        );
    }
}
