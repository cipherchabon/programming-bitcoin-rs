/// A finite field.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FiniteField {
    pub order: u32,
}

impl FiniteField {
    /// Create a new finite field.  
    /// Arguments:
    /// * `order`: the order of the field must be a prime number.
    pub fn new(order: u32) -> Self {
        if !is_prime(order) {
            panic!("The order of the field must be a prime number");
        }
        Self { order }
    }
}

impl std::fmt::Display for FiniteField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Finite field of order {}", self.order)
    }
}

fn is_prime(num: u32) -> bool {
    if num < 2 || (num > 2 && num % 2 == 0) {
        return false;
    }
    let mut i = 3;
    while i * i <= num {
        if num % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        let values = vec![2, 11, 127, 10_453, 15_485_863];
        for value in values {
            assert_eq!(is_prime(value), true);
        }
    }

    #[test]
    fn test_is_not_prime() {
        let values = vec![0, 1, 10, 378, 34_521, 98_765_432];
        for value in values {
            assert_eq!(is_prime(value), false);
        }
    }

    #[test]
    fn test_new() {
        let field = FiniteField::new(2);
        assert_eq!(field.order, 2);
    }

    #[test]
    #[should_panic]
    fn test_new_panic() {
        let _field = FiniteField::new(4);
    }

    #[test]
    fn test_new_panic_message() {
        let result = std::panic::catch_unwind(|| FiniteField::new(4));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().downcast_ref::<&str>(),
            Some(&"The order of the field must be a prime number")
        );
    }

    #[test]
    fn test_eq() {
        let field1 = FiniteField::new(2);
        let field2 = FiniteField::new(2);
        assert_eq!(field1, field2);
    }

    #[test]
    fn test_ne() {
        let field1 = FiniteField::new(2);
        let field2 = FiniteField::new(3);
        assert_ne!(field1, field2);
    }

    #[test]
    fn test_display() {
        let field = FiniteField::new(2);
        assert_eq!(format!("{}", field), "Finite field of order 2");
    }
}
