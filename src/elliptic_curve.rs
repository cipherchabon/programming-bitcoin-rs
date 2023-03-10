use crate::finite_field_element::FFElement;

/// Elliptic curve
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EllipticCurve {
    /// The coefficient of the x term
    a: FFElement,
    /// The constant term
    b: FFElement,
}

impl EllipticCurve {
    /// Create a new elliptic curve.
    /// Arguments:
    /// * `a`: the coefficient of the x term
    /// * `b`: the constant term
    pub const fn new(a: FFElement, b: FFElement) -> Self {
        Self { a, b }
    }

    pub fn a(&self) -> &FFElement {
        &self.a
    }

    pub fn b(&self) -> &FFElement {
        &self.b
    }
}

impl std::fmt::Display for EllipticCurve {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "y^2 = x^3 + {}x + {}", self.a, self.b)
    }
}

#[cfg(test)]
mod tests {

    use num::BigUint;

    use super::*;

    use crate::finite_field::FiniteField;

    #[test]
    fn test_new() {
        let field = FiniteField::new(&BigUint::from(17u32));
        let curve = EllipticCurve::new(
            FFElement::new(&BigUint::from(1u32), &field),
            FFElement::new(&BigUint::from(2u32), &field),
        );
        assert_eq!(curve.a, FFElement::new(&BigUint::from(1u32), &field));
        assert_eq!(curve.b, FFElement::new(&BigUint::from(2u32), &field));
    }

    #[test]
    fn test_eq() {
        let field = FiniteField::new(&BigUint::from(17u32));
        let curve1 = EllipticCurve::new(
            FFElement::new(&BigUint::from(1u32), &field),
            FFElement::new(&BigUint::from(2u32), &field),
        );
        let curve2 = EllipticCurve::new(
            FFElement::new(&BigUint::from(1u32), &field),
            FFElement::new(&BigUint::from(2u32), &field),
        );
        assert_eq!(curve1, curve2);
    }

    #[test]
    fn test_ne() {
        let field = FiniteField::new(&BigUint::from(17u32));
        let curve1 = EllipticCurve::new(
            FFElement::new(&BigUint::from(1u32), &field),
            FFElement::new(&BigUint::from(2u32), &field),
        );
        let curve2 = EllipticCurve::new(
            FFElement::new(&BigUint::from(2u32), &field),
            FFElement::new(&BigUint::from(1u32), &field),
        );
        assert_ne!(curve1, curve2);
    }
}
