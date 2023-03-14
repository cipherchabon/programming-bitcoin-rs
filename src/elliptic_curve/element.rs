use num::BigUint;

use super::finite_field::FiniteField;

/// A finite field element.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FFElement {
    num: BigUint,
    field: FiniteField,
}

impl FFElement {
    pub fn new(num: &BigUint, field: &FiniteField) -> Self {
        // check that num is between 0 and order-1 inclusive
        if num >= field.order() || num < &BigUint::from(0u32) {
            panic!("num must be between 0 and order-1 inclusive");
        }
        Self {
            num: num.clone(),
            field: field.clone(),
        }
    }

    pub fn new_secp256k1(num: &BigUint) -> Self {
        Self {
            num: num.clone(),
            field: FiniteField::new_secp256k1(),
        }
    }

    pub fn pow(&self, exponent: u32) -> Self {
        let p = self.field.order();
        let exp = BigUint::from(exponent);
        let num = self.num.modpow(&exp, &p);
        Self::new(&num, &self.field)
    }

    pub fn sqrt(&self) -> Self {
        let p = self.field.order();
        let exp = (p + BigUint::from(1u32)) / BigUint::from(4u32);
        let num = self.num.modpow(&exp, &p);
        Self::new(&num, &self.field)
    }

    pub fn num(&self) -> &BigUint {
        &self.num
    }

    pub fn field(&self) -> &FiniteField {
        &self.field
    }
}

impl std::fmt::Display for FFElement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "FieldElement_{}({})", self.field.order(), self.num)
    }
}

impl std::ops::Add for FFElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.field != other.field {
            panic!("Cannot add two numbers in different Fields");
        }

        let s = self.num + other.num;
        let mod_sum = s % self.field.order();

        Self::new(&mod_sum, &self.field)
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
        let p = self.field.order();
        let a = self.num % p;
        let b = other.num % p;

        Self::new(&((a + p - b) % p), &self.field)
    }
}

impl std::ops::Mul for FFElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.field != other.field {
            panic!("Cannot multiply two numbers in different Fields");
        }

        let p = self.field.order();
        let m = (self.num * other.num) % p;

        Self::new(&m, &self.field)
    }
}

impl std::ops::Mul<u32> for FFElement {
    type Output = Self;

    fn mul(self, other: u32) -> Self {
        let p = self.field.order();
        let m = (self.num * other) % p;

        Self::new(&m, &self.field)
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
        let p = self.field.order();
        let a = self.num % p;
        let b = other.num % p;

        // b^-1
        let two = BigUint::from(2u32);
        let b_inv = b.modpow(&(p - two), &p);

        Self::new(&((a * b_inv) % p), &self.field)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq() {
        let field = FiniteField::new(&BigUint::from(13u32));
        let a = FFElement::new(&BigUint::from(7u32), &field);
        let b = FFElement::new(&BigUint::from(6u32), &field);
        assert_eq!(a == b, false);
        assert_eq!(a == a, true);
    }

    #[test]
    fn test_ne() {
        let field = FiniteField::new(&BigUint::from(13u32));
        let a = FFElement::new(&BigUint::from(7u32), &field);
        let b = FFElement::new(&BigUint::from(6u32), &field);
        assert_eq!(a != b, true);
        assert_eq!(a != a, false);
    }

    #[test]
    fn test_display() {
        let field = FiniteField::new(&BigUint::from(13u32));
        let a = FFElement::new(&BigUint::from(7u32), &field);
        assert_eq!(format!("{}", a), "FieldElement_13(7)");
    }

    #[test]
    fn test_add() {
        let field = FiniteField::new(&BigUint::from(31u32));
        let a = FFElement::new(&BigUint::from(2u32), &field);
        let b = FFElement::new(&BigUint::from(15u32), &field);
        let c = FFElement::new(&BigUint::from(17u32), &field);
        assert_eq!(a + b == c, true);
        let a = FFElement::new(&BigUint::from(17u32), &field);
        let b = FFElement::new(&BigUint::from(21u32), &field);
        let c = FFElement::new(&BigUint::from(7u32), &field);
        assert_eq!(a + b == c, true);
    }

    #[test]
    fn test_sub() {
        let field = FiniteField::new(&BigUint::from(31u32));
        let a = FFElement::new(&BigUint::from(29u32), &field);
        let b = FFElement::new(&BigUint::from(4u32), &field);
        let c = FFElement::new(&BigUint::from(25u32), &field);
        assert_eq!(a - b == c, true);
        let a = FFElement::new(&BigUint::from(15u32), &field);
        let b = FFElement::new(&BigUint::from(30u32), &field);
        let c = FFElement::new(&BigUint::from(16u32), &field);
        assert_eq!(a - b == c, true);
    }

    #[test]
    fn test_mul() {
        let field = FiniteField::new(&BigUint::from(31u32));
        let a = FFElement::new(&BigUint::from(24u32), &field);
        let b = FFElement::new(&BigUint::from(19u32), &field);
        let c = FFElement::new(&BigUint::from(22u32), &field);
        assert_eq!(a * b == c, true);
    }

    #[test]
    fn test_rmul() {
        let field = FiniteField::new(&BigUint::from(31u32));
        let a = FFElement::new(&BigUint::from(24u32), &field);
        let b = FFElement::new(&BigUint::from(19u32), &field);
        let c = FFElement::new(&BigUint::from(22u32), &field);
        assert_eq!(b * a == c, true);
    }

    #[test]
    fn test_pow() {
        let field = FiniteField::new(&BigUint::from(31u32));
        let a = FFElement::new(&BigUint::from(17u32), &field);
        let b = FFElement::new(&BigUint::from(5u32), &field);
        let c = FFElement::new(&BigUint::from(18u32), &field);
        assert_eq!(
            a.pow(3) == FFElement::new(&BigUint::from(15u32), &field),
            true
        );
        assert_eq!(
            b.pow(5) * c == FFElement::new(&BigUint::from(16u32), &field),
            true
        );
    }

    #[test]
    fn test_div() {
        let field = FiniteField::new(&BigUint::from(31u32));
        let a = FFElement::new(&BigUint::from(3u32), &field);
        let b = FFElement::new(&BigUint::from(24u32), &field);
        assert_eq!(a / b == FFElement::new(&BigUint::from(4u32), &field), true);
    }
}
