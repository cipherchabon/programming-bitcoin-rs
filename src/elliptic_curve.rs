/// Elliptic curve
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct EllipticCurve {
    /// The coefficient of the x term
    pub a: i32,
    /// The constant term
    pub b: i32,
}

impl EllipticCurve {
    /// Create a new elliptic curve.
    /// Arguments:
    /// * `a`: the coefficient of the x term
    /// * `b`: the constant term
    pub const fn new(a: i32, b: i32) -> Self {
        Self { a, b }
    }
}

impl std::fmt::Display for EllipticCurve {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "y^2 = x^3 + {}x + {}", self.a, self.b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let curve = EllipticCurve::new(1, 2);
        assert_eq!(curve.a, 1);
        assert_eq!(curve.b, 2);
    }

    #[test]
    fn test_fmt() {
        let curve = EllipticCurve::new(1, 2);
        assert_eq!(format!("{}", curve), "y^2 = x^3 + 1x + 2");
    }

    #[test]
    fn test_eq() {
        let curve1 = EllipticCurve::new(1, 2);
        let curve2 = EllipticCurve::new(1, 2);
        assert_eq!(curve1, curve2);
    }

    #[test]
    fn test_ne() {
        let curve1 = EllipticCurve::new(1, 2);
        let curve2 = EllipticCurve::new(2, 1);
        assert_ne!(curve1, curve2);
    }
}
