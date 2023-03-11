use num::{BigUint, Num};

use crate::elliptic_curve::{element::FFElement, point::ECPoint};

use super::curve::Secp256k1;

// G = (Gx, Gy)
const Gx: &str = "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
const Gy: &str = "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8";

const N: &str = "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct S256Point(ECPoint);

impl S256Point {
    pub fn new(x: &FFElement, y: &FFElement) -> Result<Self, String> {
        let curve = Secp256k1::new();
        let point = ECPoint::new(&x, &y, &curve.curve())?;
        Ok(Self(point))
    }

    pub fn infinity() -> Self {
        let curve = Secp256k1::new();
        Self(ECPoint::infinity(&curve.curve()))
    }

    pub fn new_g() -> Self {
        let curve = Secp256k1::new();
        let x = FFElement::new(
            &BigUint::from_str_radix(Gx, 16).unwrap(),
            &curve.curve().a().field(),
        );
        let y = FFElement::new(
            &BigUint::from_str_radix(Gy, 16).unwrap(),
            &curve.curve().a().field(),
        );
        Self(ECPoint::new(&x, &y, &curve.curve()).unwrap())
    }

    pub fn is_infinity(&self) -> bool {
        self.0.is_infinity()
    }

    pub fn x(&self) -> Option<&FFElement> {
        self.0.x()
    }

    pub fn y(&self) -> Option<&FFElement> {
        self.0.y()
    }
}

impl std::ops::Add for S256Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Mul<BigUint> for S256Point {
    type Output = Self;

    fn mul(self, coefficient: BigUint) -> Self::Output {
        Self(self.0 * coefficient)
    }
}

impl std::fmt::Display for S256Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

//test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secp256k1_parameters() {
        let n = BigUint::from_str_radix(N, 16).unwrap();

        assert_eq!(S256Point::new_g() * n, S256Point::infinity());
    }
}
