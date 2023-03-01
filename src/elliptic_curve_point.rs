use crate::elliptic_curve::EllipticCurve;

/// An elliptic curve point
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ECPoint {
    /// The x coordinate
    /// None if the point is at infinity
    x: Option<i32>,

    /// The y coordinate
    /// None if the point is at infinity
    y: Option<i32>,

    /// The curve the point is on
    curve: EllipticCurve,
}

impl ECPoint {
    /// Creates a new point on the curve
    ///
    /// Arguments:
    ///     x: x coordinate
    ///     y: y coordinate
    ///     curve: the curve the point is on
    ///
    /// Note: This function will panic if the point is not on the curve
    pub fn new(x: i32, y: i32, curve: EllipticCurve) -> Self {
        let a = curve.a;
        let b = curve.b;

        let y2 = y.pow(2);
        let x3 = x.pow(3);

        if y2 != x3 + a * x + b {
            panic!("({}, {}) is not on the curve", x, y);
        }

        Self {
            x: Some(x),
            y: Some(y),
            curve,
        }
    }

    /// Returns the point at infinity
    pub fn infinity(curve: EllipticCurve) -> Self {
        Self {
            x: None,
            y: None,
            curve,
        }
    }

    /// Returns true if the point is at infinity (additive identity)
    pub fn is_infinity(&self) -> bool {
        // The x coordinate and y coordinate being None is how we signify the point at infinity.
        self.x.is_none() && self.y.is_none()
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
        let x1 = self.x.unwrap();
        let y1 = self.y.unwrap();
        let x2 = other.x.unwrap();
        let y2 = other.y.unwrap();

        // When the two points are additive inverses
        // (that is, they have the same x but a different y, causing a vertical line).
        // This should return the point at infinity.
        if x1 == x2 && y1 != y2 {
            return Self::infinity(self.curve);
        }

        // When x1 != x2, we need to calculate the slope of the line between the two points.
        // The slope is (y2 - y1) / (x2 - x1).
        // Then we can calculate the x coordinate of the third point by squaring the slope and
        // subtracting x1 and x2.
        // The y coordinate of the third point is calculated by multiplying the slope by the
        // difference between x1 and the new x coordinate, and then subtracting y1.
        if x1 != x2 {
            let slope = (y2 - y1) / (x2 - x1);
            let x3 = slope.pow(2) - x1 - x2;
            let y3 = slope * (x1 - x3) - y1;

            return Self::new(x3, y3, self.curve);
        }

        // When x1 == x2 and y1 == y2, we need to calculate the slope of the tangent line.
        // The slope is (3 * x1^2 + a) / (2 * y1).
        // Then we can calculate the x coordinate of the third point by squaring the slope and
        // subtracting 2 * x1.
        // The y coordinate of the third point is calculated by multiplying the slope by the
        // difference between x1 and the new x coordinate, and then subtracting y1.
        if x1 == x2 && y1 == y2 {
            if y1 == 0 {
                // If y1 == 0, then the tangent line is vertical, and the third point is the point
                return Self::infinity(self.curve);
            }

            let slope = (3 * x1.pow(2) + self.curve.a) / (2 * y1);
            let x3 = slope.pow(2) - 2 * x1;
            let y3 = slope * (x1 - x3) - y1;

            return Self::new(x3, y3, self.curve);
        }

        unreachable!();
    }
}

impl std::fmt::Display for ECPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_infinity() {
            write!(f, "Point(infinity)")
        } else {
            write!(
                f,
                "Point({}, {})_{}_{} FieldElement({})",
                self.x.unwrap().num,
                self.y.unwrap().num,
                self.curve.a.num,
                self.curve.b.num,
                self.curve.a.field.order
            )
        }
    }
}

// test module
#[cfg(test)]
mod tests {
    use super::*;

    const CURVE: EllipticCurve = EllipticCurve::new(5, 7);

    #[cfg(test)]
    mod curve_point_tests {
        use super::*;
        // Determine which of these points are on the curve y2 = x3 + 5x + 7:
        // Cases:
        //      - true: (–1,–1), (18,77)
        //      - false: (2,4), (5,7)

        #[test]
        fn test_point_on_curve_1() {
            ECPoint::new(-1, -1, CURVE);
        }

        #[test]
        fn test_point_on_curve_2() {
            ECPoint::new(18, 77, CURVE);
        }

        #[test]
        #[should_panic]
        fn test_point_not_on_curve_1() {
            ECPoint::new(2, 4, CURVE);
        }

        #[test]
        #[should_panic]
        fn test_point_not_on_curve_2() {
            ECPoint::new(5, 7, CURVE);
        }
    }

    // Determine which of these points are equal:
    #[cfg(test)]
    mod point_equality_tests {
        use super::*;

        #[test]
        fn test_point_equality() {
            let a = ECPoint::new(3, -7, CURVE);
            let b = ECPoint::new(18, 77, CURVE);
            assert_eq!(a == b, false);
            assert_eq!(a == a, true);
        }

        #[test]
        fn test_point_inequality() {
            let a = ECPoint::new(3, -7, CURVE);
            let b = ECPoint::new(18, 77, CURVE);
            assert_eq!(a != b, true);
            assert_eq!(a != a, false);
        }
    }

    mod point_addition_tests {
        use super::*;

        #[test]
        #[should_panic]
        fn test_add_different_curves() {
            let a = ECPoint::new(3, -7, CURVE);
            let b = ECPoint::new(18, 77, EllipticCurve::new(1, 2));
            let _ = a + b;
        }

        #[test]
        fn test_add() {
            let inf = ECPoint::infinity(CURVE);
            let p1 = ECPoint::new(-1, -1, CURVE);
            let p2 = ECPoint::new(-1, 1, CURVE);
            assert_eq!(inf + p1, p1);
            assert_eq!(p1 + inf, p1);
            assert_eq!(p1 + p2, inf);
        }

        #[test]
        fn test_add0() {
            let a = ECPoint::infinity(CURVE);
            let b = ECPoint::new(2, 5, CURVE);
            let c = ECPoint::new(2, -5, CURVE);
            assert_eq!(a + b, b);
            assert_eq!(b + a, b);
            assert_eq!(b + c, a);
        }

        #[test]
        fn test_add1() {
            let a = ECPoint::new(3, 7, CURVE);
            let b = ECPoint::new(-1, -1, CURVE);
            assert_eq!(a + b, ECPoint::new(2, -5, CURVE));
        }

        #[test]
        fn test_add2() {
            let a = ECPoint::new(-1, -1, CURVE);
            assert_eq!(a + a, ECPoint::new(18, 77, CURVE));
        }

        #[test]
        fn test_add3() {
            let a = ECPoint::new(2, 5, CURVE);
            let b = ECPoint::new(-1, -1, CURVE);
            assert_eq!(a + b, ECPoint::new(3, -7, CURVE));
        }
    }
}
