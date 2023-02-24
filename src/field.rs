#[derive(Debug, PartialEq, Eq)]
struct FiniteField {
    order: u32,
}

impl FiniteField {
    // create a new finite field
    // the order of the field must be a prime number
    fn new(order: u32) -> Self {
        if !is_prime(order) {
            panic!("The order of the field must be a prime number");
        }
        Self { order }
    }
}

// check if the value is prime
fn is_prime(num: u32) -> bool {
    if num == 2 {
        return true;
    }
    if num < 2 || num % 2 == 0 {
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
}
