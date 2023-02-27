use crate::finite_field::FiniteField;

/// A finite field element.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FFElement {
    num: u32,
    field: FiniteField,
}

impl FFElement {
    pub fn new(num: u32, field: FiniteField) -> Self {
        // check that num is between 0 and order-1 inclusive
        if num >= field.order {
            panic!("num must be between 0 and order-1 inclusive");
        }
        Self { num, field }
    }

    pub fn pow(&self, exponent: u32) -> Self {
        let p = self.field.order;
        let exp = exponent.rem_euclid(p - 1);
        let num = self.num.pow(exp).rem_euclid(p);
        Self::new(num, self.field)
    }
}

impl std::fmt::Display for FFElement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "FieldElement_{}({})", self.field.order, self.num)
    }
}

impl std::ops::Add for FFElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.field != other.field {
            panic!("Cannot add two numbers in different Fields");
        }

        match self.num.checked_add(other.num) {
            Some(num) => {
                let mod_sum = num.rem_euclid(self.field.order);
                Self::new(mod_sum, self.field)
            }
            None => panic!("Overflow error"),
        }
    }
}

impl std::ops::Sub for FFElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.field != other.field {
            panic!("Cannot subtract two numbers in different Fields");
        }

        // property of sums and differences in modular arithmetic
        // (a - b) mod p = [(a mod p) - (b mod p)] mod p
        let p = self.field.order;
        let a = self.num.rem_euclid(p);
        let b = other.num.rem_euclid(p);

        Self::new((a + p - b).rem_euclid(p), self.field)
    }
}

impl std::ops::Mul for FFElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.field != other.field {
            panic!("Cannot multiply two numbers in different Fields");
        }

        match self.num.checked_mul(other.num) {
            Some(num) => {
                let p = self.field.order;
                let mod_prod = num.rem_euclid(p);
                Self::new(mod_prod, self.field)
            }
            None => panic!("Overflow error"),
        }
    }
}

impl std::ops::Div for FFElement {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        if self.field != other.field {
            panic!("Cannot divide two numbers in different Fields");
        }

        // property of products and quotients in modular arithmetic
        // (a / b) mod p = [(a mod p) * (b^-1 mod p)] mod p
        let p = self.field.order;
        let a = self.num.rem_euclid(p);
        let b = other.num.rem_euclid(p);

        // find the multiplicative inverse of b
        let b_inv = match b {
            0 => panic!("Cannot divide by zero"),
            1 => 1,
            _ => {
                let mut x = 1;
                while (b * x).rem_euclid(p) != 1 {
                    x += 1;
                }
                x
            }
        };

        let num = (a * b_inv).rem_euclid(p);

        Self::new(num, self.field)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq() {
        let field = FiniteField::new(13);
        let a = FFElement::new(7, field);
        let b = FFElement::new(6, field);
        assert_eq!(a == b, false);
        assert_eq!(a == a, true);
    }

    #[test]
    fn test_ne() {
        let field = FiniteField::new(13);
        let a = FFElement::new(7, field);
        let b = FFElement::new(6, field);
        assert_eq!(a != b, true);
        assert_eq!(a != a, false);
    }

    #[test]
    fn test_display() {
        let field = FiniteField::new(13);
        let a = FFElement::new(7, field);
        assert_eq!(format!("{}", a), "FieldElement_13(7)");
    }

    #[test]
    fn test_add() {
        let field = FiniteField::new(31);
        let a = FFElement::new(2, field);
        let b = FFElement::new(15, field);
        let c = FFElement::new(17, field);
        assert_eq!(a + b == c, true);
        let a = FFElement::new(17, field);
        let b = FFElement::new(21, field);
        let c = FFElement::new(7, field);
        assert_eq!(a + b == c, true);
    }

    #[test]
    fn test_sub() {
        let field = FiniteField::new(31);
        let a = FFElement::new(29, field);
        let b = FFElement::new(4, field);
        let c = FFElement::new(25, field);
        assert_eq!(a - b == c, true);
        let a = FFElement::new(15, field);
        let b = FFElement::new(30, field);
        let c = FFElement::new(16, field);
        assert_eq!(a - b == c, true);
    }

    #[test]
    fn test_mul() {
        let field = FiniteField::new(31);
        let a = FFElement::new(24, field);
        let b = FFElement::new(19, field);
        let c = FFElement::new(22, field);
        assert_eq!(a * b == c, true);
    }

    #[test]
    fn test_pow() {
        let field = FiniteField::new(31);
        let a = FFElement::new(17, field);
        let b = FFElement::new(5, field);
        let c = FFElement::new(18, field);
        assert_eq!(a.pow(3) == FFElement::new(15, field), true);
        assert_eq!(b.pow(5) * c == FFElement::new(16, field), true);
    }

    #[test]
    fn test_div() {
        let field = FiniteField::new(31);
        let a = FFElement::new(3, field);
        let b = FFElement::new(24, field);
        assert_eq!(a / b == FFElement::new(4, field), true);
    }
}
