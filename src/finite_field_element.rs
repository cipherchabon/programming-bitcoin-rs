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
        if num >= field.order() {
            panic!("num must be between 0 and order-1 inclusive");
        }
        Self { num, field }
    }
}

impl std::fmt::Display for FFElement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "FieldElement_{}({})", self.field.order(), self.num)
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
}
