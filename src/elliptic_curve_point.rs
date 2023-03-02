use crate::{elliptic_curve::EllipticCurve, finite_field_element::FFElement};

/// An elliptic curve point
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

impl ECPoint {
    /// Creates a new point on the curve
    ///
    /// Arguments:
    ///     x: x coordinate
    ///     y: y coordinate
    ///     curve: the curve the point is on
    ///
    /// Note: This function will panic if the point is not on the curve
    pub fn new(x: FFElement, y: FFElement, curve: EllipticCurve) -> Result<Self, String> {
        let a = curve.a;
        let b = curve.b;

        let y2 = y.pow(2);
        let x3 = x.pow(3);

        if y2 != x3 + a * x + b {
            return Err(format!("({}, {}) is not on the curve", x, y));
        }

        Ok(Self {
            x: Some(x),
            y: Some(y),
            curve,
        })
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

            return Self::new(x3, y3, self.curve).unwrap();
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

            let slope = (x1.pow(2) * 3 + self.curve.a) / (y1 * 2);
            let x3 = slope.pow(2) - x1 * 2;
            let y3 = slope * (x1 - x3) - y1;

            return Self::new(x3, y3, self.curve).unwrap();
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

#[cfg(test)]
mod tests {
    use crate::finite_field::FiniteField;

    use super::*;

    #[test]
    fn test_point_on_curve() {
        let order = 223;
        let field = FiniteField::new(order);
        let a = FFElement::new(0, field);
        let b = FFElement::new(7, field);
        let curve = EllipticCurve::new(a, b);

        let valid_points = vec![(192, 105), (17, 56), (1, 193)];
        let invalid_points = vec![(200, 119), (42, 99)];

        for (x, y) in valid_points {
            let x = FFElement::new(x, field);
            let y = FFElement::new(y, field);
            ECPoint::new(x, y, curve).unwrap();
        }

        for (x, y) in invalid_points {
            let x = FFElement::new(x, field);
            let y = FFElement::new(y, field);
            assert!(ECPoint::new(x, y, curve).is_err());
        }
    }

    #[test]
    fn test_add() {
        let order = 223;
        let field = FiniteField::new(order);
        let a = FFElement::new(0, field);
        let b = FFElement::new(7, field);
        let curve = EllipticCurve::new(a, b);

        let additions = vec![
            // (x1, y1, x2, y2, x3, y3)
            (192, 105, 17, 56, 170, 142),
            (47, 71, 117, 141, 60, 139),
            (143, 98, 76, 66, 47, 71),
        ];

        for (x1_raw, y1_raw, x2_raw, y2_raw, x3_raw, y3_raw) in additions {
            let x1 = FFElement::new(x1_raw, field);
            let y1 = FFElement::new(y1_raw, field);
            let p1 = ECPoint::new(x1, y1, curve).unwrap();

            let x2 = FFElement::new(x2_raw, field);
            let y2 = FFElement::new(y2_raw, field);
            let p2 = ECPoint::new(x2, y2, curve).unwrap();

            let x3 = FFElement::new(x3_raw, field);
            let y3 = FFElement::new(y3_raw, field);
            let p3 = ECPoint::new(x3, y3, curve).unwrap();

            assert_eq!(p1 + p2, p3);
        }
    }
}
