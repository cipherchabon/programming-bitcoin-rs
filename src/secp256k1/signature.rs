use core::fmt;

use num::BigUint;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Signature {
    r: BigUint,
    s: BigUint,
}

impl Signature {
    pub fn new(r: BigUint, s: BigUint) -> Self {
        Self { r, s }
    }

    pub fn r(&self) -> &BigUint {
        &self.r
    }

    pub fn s(&self) -> &BigUint {
        &self.s
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Signature({:x},{:x})", self.r, self.s)
    }
}
