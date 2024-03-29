use num::{BigUint, Integer};

use crate::utils::{encode_base58::encode_base58_checksum, hash160::hash160};

use super::{
    curve::EllipticCurve, element::FFElement, secp256k1_params::Secp256k1Params,
    signature::Signature,
};

/// An elliptic curve point
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ECPoint {
    /// The x coordinate
    /// None if the point is at infinity
    x: Option<FFElement>,

    /// The y coordinate
    /// None if the point is at infinity
    y: Option<FFElement>,

    /// The curve the point is on
    curve: EllipticCurve,
}

/// Constructors
impl ECPoint {
    /// Creates a new point on the curve
    ///
    /// Arguments:
    ///     x: x coordinate
    ///     y: y coordinate
    ///     curve: the curve the point is on
    ///
    /// Note: This function will panic if the point is not on the curve
    pub fn new(x: &FFElement, y: &FFElement, curve: &EllipticCurve) -> Result<Self, String> {
        let a = curve.a().clone();
        let b = curve.b().clone();

        let y2 = y.pow(2);
        let x3 = x.pow(3);

        if y2 != x3 + a * x.clone() + b {
            return Err(format!("({}, {}) is not on the curve", *x, *y));
        }

        Ok(Self {
            x: Some(x.clone()),
            y: Some(y.clone()),
            curve: curve.clone(),
        })
    }

    /// Returns the point at infinity
    pub fn new_infinity(curve: &EllipticCurve) -> Self {
        Self {
            x: None,
            y: None,
            curve: curve.clone(),
        }
    }

    /// Creates a new point on the secp256k1 curve
    pub fn new_secp256k1(x: &FFElement, y: &FFElement) -> Result<Self, String> {
        Self::new(x, y, &EllipticCurve::new_secp256k1())
    }

    /// Get the point at infinity on the secp256k1 curve
    pub fn new_secp256k1_infinity() -> Self {
        Self::new_infinity(&EllipticCurve::new_secp256k1())
    }
}

/// Methods
impl ECPoint {
    /// Verifies if a digital signature is valid for a given message.
    ///
    /// # Arguments
    ///
    /// * `self` - The public key to verify the signature against.
    /// * `z` - The hash of the message that was signed.
    /// * `signature` - The digital signature to verify.
    ///
    pub fn verify(&self, z: &BigUint, signature: &Signature) -> bool {
        // By Fermat's Little Theorem, 1/s = pow(s, N-2, N)
        let n = &Secp256k1Params::n();
        let s = signature.s();
        let two = &BigUint::from(2u8);
        let s_inv = s.modpow(&(n - two), n);

        // u = z / s
        let u = z * &s_inv % n;

        // v = r / s
        let v = signature.r() * &s_inv % n;

        // u*G + v*P should have as the x coordinate, r
        let g = Secp256k1Params::g();
        let p = self.clone();
        let total = g * u + p * v;

        total.x.unwrap().num() == signature.r()
    }

    /// Returns the address of the public key
    pub fn get_address(&self, compressed: bool, testnet: bool) -> String {
        let sec = if compressed {
            self.to_compressed_sec()
        } else {
            self.to_uncompressed_sec()
        };

        let h160 = hash160(&sec);

        let prefix = if testnet { b"\x6f" } else { b"\x00" };

        let mut address = prefix.to_vec();
        address.extend(h160);

        encode_base58_checksum(&address)
    }

    /// Returns true if the point is at infinity (additive identity)
    pub fn is_infinity(&self) -> bool {
        // The x coordinate and y coordinate being None is how we signify the point at infinity.
        self.x.is_none() && self.y.is_none()
    }

    /// Returns the x coordinate of the point
    /// Note: This will panic if the point is at infinity
    pub(super) fn x(&self) -> Option<FFElement> {
        self.x.clone()
    }

    /// Returns the y coordinate of the point
    /// Note: This will panic if the point is at infinity
    pub(super) fn y(&self) -> Option<FFElement> {
        self.y.clone()
    }
}

impl ECPoint {
    /// Uncompressed SEC format
    pub fn to_uncompressed_sec(&self) -> Vec<u8> {
        let mut sec = vec![4u8];
        sec.extend(self.x().unwrap().num().to_bytes_be());
        sec.extend(self.y().unwrap().num().to_bytes_be());
        sec
    }

    /// Compressed SEC format
    pub fn to_compressed_sec(&self) -> Vec<u8> {
        let y_is_even = self.y().unwrap().num().is_even();
        let mut sec = vec![if y_is_even { 2u8 } else { 3u8 }];
        sec.extend(self.x().unwrap().num().to_bytes_be());
        sec
    }

    /// Deserialize a point from SEC format
    pub fn parse(sec_bin: &[u8]) -> Result<Self, String> {
        // The uncompressed SEC format is pretty straightforward.
        if sec_bin.len() == 65 {
            if sec_bin[0] != 4 {
                return Err("Invalid SEC format".to_string());
            }

            let x = BigUint::from_bytes_be(&sec_bin[1..33]);
            let y = BigUint::from_bytes_be(&sec_bin[33..65]);
            return Self::new_secp256k1(
                &FFElement::new_secp256k1(&x),
                &FFElement::new_secp256k1(&y),
            );
        }

        if sec_bin.len() == 33 {
            // The evenness of the y coordinate is given in the first byte.
            let is_even = sec_bin[0] == 2;
            let x = BigUint::from_bytes_be(&sec_bin[1..]);
            let x = FFElement::new_secp256k1(&x);

            // right side of the equation y^2 = x^3 + 7
            let alpha = x.pow(3) + FFElement::new_secp256k1(&Secp256k1Params::b());

            // solve for left side
            let beta = alpha.sqrt();

            let even_beta = if beta.num().is_even() {
                beta.clone()
            } else {
                FFElement::new_secp256k1(&(Secp256k1Params::p() - beta.num()))
            };

            let odd_beta = if beta.num().is_even() {
                FFElement::new_secp256k1(&(Secp256k1Params::p() - beta.num()))
            } else {
                beta
            };

            if is_even {
                return Self::new_secp256k1(&x, &even_beta);
            } else {
                return Self::new_secp256k1(&x, &odd_beta);
            }
        }

        Err("Invalid SEC format".to_string())
    }
}

impl std::ops::Add for ECPoint {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.curve != other.curve {
            panic!("Points {}, {} are not on the same curve", self, other);
        }

        // If either point is the point at infinity, we return the other point.
        if self.is_infinity() {
            return other;
        } else if other.is_infinity() {
            return self;
        }

        // We need to unwrap the x and y coordinates because we know they are not None.
        let x1 = &self.x.unwrap();
        let y1 = &self.y.unwrap();
        let x2 = &other.x.unwrap();
        let y2 = &other.y.unwrap();

        // When the two points are additive inverses
        // (that is, they have the same x but a different y, causing a vertical line).
        // This should return the point at infinity.
        if x1 == x2 && y1 != y2 {
            return Self::new_infinity(&self.curve);
        }

        // When x1 != x2, we need to calculate the slope of the line between the two points.
        // The slope is (y2 - y1) / (x2 - x1).
        // Then we can calculate the x coordinate of the third point by squaring the slope and
        // subtracting x1 and x2.
        // The y coordinate of the third point is calculated by multiplying the slope by the
        // difference between x1 and the new x coordinate, and then subtracting y1.
        if x1 != x2 {
            let slope = (y2.clone() - y1.clone()) / (x2.clone() - x1.clone());
            let x3 = slope.pow(2) - x1.clone() - x2.clone();
            let y3 = slope * (x1.clone() - x3.clone()) - y1.clone();

            return Self::new(&x3, &y3, &self.curve).unwrap();
        }

        // When x1 == x2 and y1 == y2, we need to calculate the slope of the tangent line.
        // The slope is (3 * x1^2 + a) / (2 * y1).
        // Then we can calculate the x coordinate of the third point by squaring the slope and
        // subtracting 2 * x1.
        // The y coordinate of the third point is calculated by multiplying the slope by the
        // difference between x1 and the new x coordinate, and then subtracting y1.
        if x1 == x2 && y1 == y2 {
            // TODO: Fix this
            // if y1 == 0 {
            //     // If y1 == 0, then the tangent line is vertical, and the third point is the point
            //     return Self::infinity(self.curve);
            // }

            let term1 = x1.pow(2) * 3;
            let term2 = (*self.curve.a()).clone();
            let term3 = y1.clone() * 2;

            let slope = (term1 + term2) / term3;

            let x3 = slope.pow(2) - x1.clone() * 2;
            let y3 = slope * (x1.clone() - x3.clone()) - y1.clone();

            return Self::new(&x3, &y3, &self.curve).unwrap();
        }

        unreachable!();
    }
}

impl std::ops::Mul<u32> for ECPoint {
    type Output = Self;

    fn mul(self, coefficient: u32) -> Self {
        let mut coef = coefficient;
        // current represents the point that’s at the current bit. The first
        // time through the loop it represents 1 × self; the second time it will
        // be 2 × self, the third time 4 × self, then 8 × self, and so on. We
        // double the point each time. In binary the coefficients are 1, 10,
        // 100, 1000, 10000, etc.
        let mut current = self.clone();
        // We start the result at 0, or the point at infinity.
        let mut result = Self::new_infinity(&self.curve);

        while coef > 0 {
            // We are looking at whether the rightmost bit is a 1. If it is,
            // then we add the value of the current bit.
            if coef & 1 == 1 {
                result = result + current.clone();
            }
            // We need to double the point until we’re past how big the
            // coefficient can be.
            current = current.clone() + current.clone();
            // We bit-shift the coefficient to the right.
            coef >>= 1;
        }

        result
    }
}

impl std::ops::Mul<BigUint> for ECPoint {
    type Output = Self;

    fn mul(self, coefficient: BigUint) -> Self {
        let mut coef = coefficient;
        // current represents the point that’s at the current bit. The first
        // time through the loop it represents 1 × self; the second time it will
        // be 2 × self, the third time 4 × self, then 8 × self, and so on. We
        // double the point each time. In binary the coefficients are 1, 10,
        // 100, 1000, 10000, etc.
        let mut current = self.clone();
        // We start the result at 0, or the point at infinity.
        let mut result = Self::new_infinity(&self.curve);

        while coef > BigUint::from(0u32) {
            // We are looking at whether the rightmost bit is a 1. If it is,
            // then we add the value of the current bit.
            if coef.clone() & BigUint::from(1u32) == BigUint::from(1u32) {
                result = result + current.clone();
            }
            // We need to double the point until we’re past how big the
            // coefficient can be.
            current = current.clone() + current.clone();
            // We bit-shift the coefficient to the right.
            coef >>= 1;
        }

        result
    }
}

impl std::fmt::Display for ECPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_infinity() {
            write!(f, "Point(infinity)")
        } else {
            let p = self.clone();
            write!(
                f,
                "Point({}, {})_{}_{} FieldElement({})",
                p.x.unwrap(),
                p.y.unwrap(),
                p.curve.a(),
                p.curve.b(),
                p.curve.a().field().order(),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use num::{BigUint, Num};

    use crate::ecc::finite_field::FiniteField;

    use super::*;

    #[test]
    fn test_point_on_curve() {
        let field = FiniteField::new(&BigUint::from(223u32));
        let a = FFElement::new(&BigUint::from(0u32), &field);
        let b = FFElement::new(&BigUint::from(7u32), &field);
        let curve = EllipticCurve::new(a, b);

        let valid_points = vec![(192_u32, 105_u32), (17, 56), (1, 193)];
        let invalid_points = vec![(200_u32, 119_u32), (42, 99)];

        for (x, y) in valid_points {
            let x = FFElement::new(&BigUint::from(x), &field);
            let y = FFElement::new(&BigUint::from(y), &field);
            ECPoint::new(&x, &y, &curve).unwrap();
        }

        for (x, y) in invalid_points {
            let x = FFElement::new(&BigUint::from(x), &field);
            let y = FFElement::new(&BigUint::from(y), &field);
            assert!(ECPoint::new(&x, &y, &curve).is_err());
        }
    }

    #[test]
    fn test_add() {
        let field = FiniteField::new(&BigUint::from(223_u32));
        let a = FFElement::new(&BigUint::from(0u32), &field);
        let b = FFElement::new(&BigUint::from(7u32), &field);
        let curve = EllipticCurve::new(a, b);

        let additions: Vec<(u32, u32, u32, u32, u32, u32)> = vec![
            // (x1, y1, x2, y2, x3, y3)
            (192, 105, 17, 56, 170, 142),
            (47, 71, 117, 141, 60, 139),
            (143, 98, 76, 66, 47, 71),
        ];

        for (x1_raw, y1_raw, x2_raw, y2_raw, x3_raw, y3_raw) in additions {
            let x1 = FFElement::new(&BigUint::from(x1_raw), &field);
            let y1 = FFElement::new(&BigUint::from(y1_raw), &field);
            let p1 = ECPoint::new(&x1, &y1, &curve).unwrap();

            let x2 = FFElement::new(&BigUint::from(x2_raw), &field);
            let y2 = FFElement::new(&BigUint::from(y2_raw), &field);
            let p2 = ECPoint::new(&x2, &y2, &curve).unwrap();

            let x3 = FFElement::new(&BigUint::from(x3_raw), &field);
            let y3 = FFElement::new(&BigUint::from(y3_raw), &field);
            let p3 = ECPoint::new(&x3, &y3, &curve).unwrap();

            assert_eq!(p1 + p2, p3);
        }
    }

    #[test]
    fn test_rmul() {
        let field = FiniteField::new(&BigUint::from(223_u32));
        let a = FFElement::new(&BigUint::from(0u32), &field);
        let b = FFElement::new(&BigUint::from(7u32), &field);
        let curve = EllipticCurve::new(a, b);

        let multiplications: Vec<(u32, u32, u32, u32, u32)> = vec![
            // (coefficient, x1, y1, x2, y2)
            (2, 192, 105, 49, 71),
            (2, 143, 98, 64, 168),
            (2, 47, 71, 36, 111),
            (4, 47, 71, 194, 51),
            (8, 47, 71, 116, 55),
            (21, 47, 71, 0, 0),
        ];

        for (s, x1_raw, y1_raw, x2_raw, y2_raw) in multiplications {
            let x1 = FFElement::new(&BigUint::from(x1_raw), &field);
            let y1 = FFElement::new(&BigUint::from(y1_raw), &field);
            let p1 = ECPoint::new(&x1, &y1, &curve).unwrap();

            let p2 = if x2_raw == 0 && y2_raw == 0 {
                ECPoint::new_infinity(&curve)
            } else {
                let x2 = FFElement::new(&BigUint::from(x2_raw), &field);
                let y2 = FFElement::new(&BigUint::from(y2_raw), &field);
                ECPoint::new(&x2, &y2, &curve).unwrap()
            };

            assert_eq!(p1 * s, p2);
        }
    }

    #[test]
    fn test_rmul_biguint() {
        let field = FiniteField::new(&BigUint::from(223_u32));
        let a = FFElement::new(&BigUint::from(0u32), &field);
        let b = FFElement::new(&BigUint::from(7u32), &field);
        let curve = EllipticCurve::new(a, b);

        let multiplications: Vec<(BigUint, u32, u32, u32, u32)> = vec![
            // (coefficient, x1, y1, x2, y2)
            (BigUint::from(2u32), 192, 105, 49, 71),
            (BigUint::from(2u32), 143, 98, 64, 168),
            (BigUint::from(2u32), 47, 71, 36, 111),
            (BigUint::from(4u32), 47, 71, 194, 51),
            (BigUint::from(8u32), 47, 71, 116, 55),
            (BigUint::from(21u32), 47, 71, 0, 0),
        ];

        for (s, x1_raw, y1_raw, x2_raw, y2_raw) in multiplications {
            let x1 = FFElement::new(&BigUint::from(x1_raw), &field);
            let y1 = FFElement::new(&BigUint::from(y1_raw), &field);
            let p1 = ECPoint::new(&x1, &y1, &curve).unwrap();

            let p2 = if x2_raw == 0 && y2_raw == 0 {
                ECPoint::new_infinity(&curve)
            } else {
                let x2 = FFElement::new(&BigUint::from(x2_raw), &field);
                let y2 = FFElement::new(&BigUint::from(y2_raw), &field);
                ECPoint::new(&x2, &y2, &curve).unwrap()
            };

            assert_eq!(p1 * s, p2);
        }
    }

    #[test]
    fn test_secp256k1_parameters() {
        assert_eq!(
            Secp256k1Params::g() * Secp256k1Params::n(),
            ECPoint::new_secp256k1_infinity()
        );
    }

    #[test]
    fn test_verify() {
        let x = FFElement::new_secp256k1(
            &BigUint::from_str_radix(
                "887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c",
                16,
            )
            .unwrap(),
        );
        let y = FFElement::new_secp256k1(
            &BigUint::from_str_radix(
                "61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34",
                16,
            )
            .unwrap(),
        );

        let point = ECPoint::new_secp256k1(&x, &y).unwrap();

        let z = BigUint::from_str_radix(
            "ec208baa0fc1c19f708a9ca96fdeff3ac3f230bb4a7ba4aede4942ad003c0f60",
            16,
        )
        .unwrap();

        let r = BigUint::from_str_radix(
            "ac8d1c87e51d0d441be8b3dd5b05c8795b48875dffe00b7ffcfac23010d3a395",
            16,
        )
        .unwrap();

        let s = BigUint::from_str_radix(
            "68342ceff8935ededd102dd876ffd6ba72d6a427a3edb13d26eb0781cb423c4",
            16,
        )
        .unwrap();

        assert!(point.verify(&z, &Signature::new(&r, &s)));

        let z = BigUint::from_str_radix(
            "7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d",
            16,
        )
        .unwrap();

        let r = BigUint::from_str_radix(
            "eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c",
            16,
        )
        .unwrap();

        let s = BigUint::from_str_radix(
            "c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6",
            16,
        )
        .unwrap();

        assert!(point.verify(&z, &Signature::new(&r, &s)));
    }

    #[test]
    fn test_uncompressed_sec() {
        let point = Secp256k1Params::g() * BigUint::from(5000u32);
        assert_eq!(
            point.to_uncompressed_sec(),
            hex::decode(
                "04\
                ffe558e388852f0120e46af2d1b370f85854a8eb0841811ece0e3e03d282d57c315dc72890a4\
                f10a1481c031b03b351b0dc79901ca18a00cf009dbdb157a1d10"
            )
            .unwrap()
        );
        let point = Secp256k1Params::g() * BigUint::from(2018_u32).pow(5);
        assert_eq!(
            point.to_uncompressed_sec(),
            hex::decode(
                "04\
                027f3da1918455e03c46f659266a1bb5204e959db7364d2f473bdf8f0a13cc9dff87647fd023\
                c13b4a4994f17691895806e1b40b57f4fd22581a4f46851f3b06"
            )
            .unwrap()
        );
        let point = Secp256k1Params::g() * BigUint::from_str_radix("deadbeef12345", 16).unwrap();
        assert_eq!(
            point.to_uncompressed_sec(),
            hex::decode(
                "04\
                d90cd625ee87dd38656dd95cf79f65f60f7273b67d3096e68bd81e4f5342691f842efa762fd5\
                9961d0e99803c61edba8b3e3f7dc3a341836f97733aebf987121"
            )
            .unwrap()
        );
    }

    #[test]
    fn test_compressed_sec() {
        let point = Secp256k1Params::g() * BigUint::from(5001u32);
        assert_eq!(
            point.to_compressed_sec(),
            hex::decode("0357a4f368868a8a6d572991e484e664810ff14c05c0fa023275251151fe0e53d1")
                .unwrap()
        );
        let point = Secp256k1Params::g() * BigUint::from(2019_u32).pow(5);
        assert_eq!(
            point.to_compressed_sec(),
            hex::decode("02933ec2d2b111b92737ec12f1c5d20f3233a0ad21cd8b36d0bca7a0cfa5cb8701")
                .unwrap()
        );
        let point = Secp256k1Params::g() * BigUint::from_str_radix("deadbeef54321", 16).unwrap();
        assert_eq!(
            point.to_compressed_sec(),
            hex::decode("0296be5b1292f6c856b3c5654e886fc13511462059089cdf9c479623bfcbe77690")
                .unwrap()
        );
    }

    #[test]
    fn test_parse_uncompressed_sec() {
        let point = Secp256k1Params::g() * BigUint::from(5000u32);
        assert_eq!(ECPoint::parse(&point.to_uncompressed_sec()).unwrap(), point);
        let point = Secp256k1Params::g() * BigUint::from(2018_u32).pow(5);
        assert_eq!(ECPoint::parse(&point.to_uncompressed_sec()).unwrap(), point);
        let point = Secp256k1Params::g() * BigUint::from_str_radix("deadbeef12345", 16).unwrap();
        assert_eq!(ECPoint::parse(&point.to_uncompressed_sec()).unwrap(), point);
    }

    #[test]
    fn test_parse_compressed_sec() {
        let point = Secp256k1Params::g() * BigUint::from(5001u32);
        assert_eq!(ECPoint::parse(&point.to_compressed_sec()).unwrap(), point);
        let point = Secp256k1Params::g() * BigUint::from(2019_u32).pow(5);
        assert_eq!(ECPoint::parse(&point.to_compressed_sec()).unwrap(), point);
        let point = Secp256k1Params::g() * BigUint::from_str_radix("deadbeef54321", 16).unwrap();
        assert_eq!(ECPoint::parse(&point.to_compressed_sec()).unwrap(), point);
    }

    #[test]
    fn test_address_exercise_5() {
        let point = Secp256k1Params::g() * BigUint::from(5002u32);

        assert_eq!(
            point.get_address(false, true),
            "mmTPbXQFxboEtNRkwfh6K51jvdtHLxGeMA"
        );

        let point = Secp256k1Params::g() * BigUint::from(2020_u32).pow(5);
        assert_eq!(
            point.get_address(true, true),
            "mopVkxp8UhXqRYbCYJsbeE1h1fiF64jcoH"
        );
        let point = Secp256k1Params::g() * BigUint::from_str_radix("12345deadbeef", 16).unwrap();
        assert_eq!(
            point.get_address(true, false),
            "1F1Pn2y6pDb68E5nYJJeba4TLg2U7B6KF1"
        );
    }

    #[test]
    fn test_address() {
        let secret1 = 888_u32.pow(3);
        let secret2 = 321_u32;
        let secret3 = 4242424242_u32;

        let values = vec![
            // secret, compressed, testnet, address
            (secret1, true, false, "148dY81A9BmdpMhvYEVznrM45kWN32vSCN"),
            (secret1, true, true, "mieaqB68xDCtbUBYFoUNcmZNwk74xcBfTP"),
            (secret2, false, false, "1S6g2xBJSED7Qr9CYZib5f4PYVhHZiVfj"),
            (secret2, false, true, "mfx3y63A7TfTtXKkv7Y6QzsPFY6QCBCXiP"),
            (secret3, false, false, "1226JSptcStqn4Yq9aAmNXdwdc2ixuH9nb"),
            (secret3, false, true, "mgY3bVusRUL6ZB2Ss999CSrGVbdRwVpM8s"),
        ];

        for (secret, compressed, testnet, address) in values {
            let point = Secp256k1Params::g() * BigUint::from(secret);
            assert_eq!(point.get_address(compressed, testnet), address);
        }
    }
}
