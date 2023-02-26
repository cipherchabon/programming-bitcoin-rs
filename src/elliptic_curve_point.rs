use crate::elliptic_curve::EllipticCurve;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ECPoint {
    x: Option<i32>,
    y: Option<i32>,
    curve: EllipticCurve,
}

impl ECPoint {
    pub fn new(x: i32, y: i32, curve: EllipticCurve) -> Self {
        let a = curve.a();
        let b = curve.b();

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

    pub fn infinity(curve: EllipticCurve) -> Self {
        Self {
            x: None,
            y: None,
            curve,
        }
    }

    pub fn is_infinity(&self) -> bool {
        self.x.is_none() && self.y.is_none()
    }

    pub fn x(&self) -> Option<i32> {
        self.x
    }

    pub fn y(&self) -> Option<i32> {
        self.y
    }

    pub fn curve(&self) -> EllipticCurve {
        self.curve
    }
}

impl std::fmt::Display for ECPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_infinity() {
            write!(f, "Point(infinity)")
        } else {
            write!(f, "Point({}, {})", self.x.unwrap(), self.y.unwrap())
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
}
