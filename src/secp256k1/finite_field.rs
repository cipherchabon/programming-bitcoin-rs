use num::{BigUint, Num};

use crate::elliptic_curve::finite_field::FiniteField;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct S256FiniteField(FiniteField);

// Finite field prime order
const P: &str = "fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f";

impl S256FiniteField {
    pub fn new() -> Self {
        let order = BigUint::from_str_radix(P, 16).unwrap();
        let field = FiniteField::new(&order);
        Self(field)
    }
}

impl std::fmt::Display for S256FiniteField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "S256FiniteField")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = S256FiniteField::new();
    }

    #[test]
    fn test_display() {
        let field = S256FiniteField::new();
        assert_eq!(format!("{}", field), "S256FiniteField");
    }

    #[test]
    fn test_order() {
        let field = S256FiniteField::new();
        assert_eq!(field.0.order(), &BigUint::from_str_radix(P, 16).unwrap());
    }
}
