use num::BigUint;

use crate::utils::biguint_primality_checker::biguint_primality_checker;

use super::secp256k1_params::Secp256k1Params;

/// A finite field.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FiniteField {
    order: BigUint,
}

impl FiniteField {
    /// Create a new finite field.  
    /// Arguments:
    /// * `order`: the order of the field must be a prime number.
    pub fn new(order: &BigUint) -> Self {
        if !biguint_primality_checker(order) {
            panic!("The order of the field must be a prime number");
        }
        Self {
            order: order.clone(),
        }
    }

    pub fn new_secp256k1() -> Self {
        let order = Secp256k1Params::p();
        Self::new(&order)
    }

    /// Get the order of the field.
    pub fn order(&self) -> &BigUint {
        &self.order
    }
}

impl std::fmt::Display for FiniteField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Finite field of order {}", self.order)
    }
}

#[cfg(test)]
mod tests {
    use num::FromPrimitive;

    use super::*;

    #[test]
    fn test_new() {
        let two = BigUint::from_u32(2).unwrap();
        let field = FiniteField::new(&two);
        assert_eq!(field.order, two);
    }

    #[test]
    #[should_panic]
    fn test_new_panic() {
        let four = BigUint::from_u32(4).unwrap();
        let _field = FiniteField::new(&four);
    }

    #[test]
    fn test_new_panic_message() {
        let four = BigUint::from_u32(4).unwrap();
        let result = std::panic::catch_unwind(|| FiniteField::new(&four));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().downcast_ref::<&str>(),
            Some(&"The order of the field must be a prime number")
        );
    }

    #[test]
    fn test_eq() {
        let two = BigUint::from_u32(2).unwrap();
        let field1 = FiniteField::new(&two);
        let field2 = FiniteField::new(&two);
        assert_eq!(field1, field2);
    }

    #[test]
    fn test_ne() {
        let two = BigUint::from_u32(2).unwrap();
        let three = BigUint::from_u32(3).unwrap();
        let field1 = FiniteField::new(&two);
        let field2 = FiniteField::new(&three);
        assert_ne!(field1, field2);
    }

    #[test]
    fn test_display() {
        let two = BigUint::from_u32(2).unwrap();
        let field = FiniteField::new(&two);
        assert_eq!(format!("{}", field), "Finite field of order 2");
    }
}
