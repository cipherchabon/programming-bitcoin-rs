use num::BigUint;
use num_bigint::RandBigInt;
use rand;

use super::{secp256k1_params::Secp256k1Params, signature::Signature};

/// PrivateKey is a wrapper around a secret number.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PrivateKey {
    secret: BigUint,
}

impl PrivateKey {
    /// Creates a new private key from a secret number.
    pub fn new(secret: &BigUint) -> Self {
        Self {
            secret: secret.clone(),
        }
    }
}

impl std::fmt::Display for PrivateKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.secret.to_str_radix(16))
    }
}

impl PrivateKey {
    /// Signs a message using the private key.
    pub fn sign(&self, message: &BigUint) -> Signature {
        let n = Secp256k1Params::n();
        let g = Secp256k1Params::g();

        // chooses a random integer from [0,n)
        let mut rng = rand::thread_rng();
        let k = rng.gen_biguint_below(&n);

        // r is the x coordinate of k*G
        let x = (g * k.clone()).x().unwrap();
        let r = x.num();

        // We use Fermat’s little theorem, and n, which is prime.
        // s = (z + re)/k
        let exp = n.clone() - 2u32;
        let module = n.clone();
        let k_inv = k.modpow(&exp, &module);
        let mut s = (message + r * &self.secret) * k_inv % &n;

        // It turns out that using the low-s value will get nodes to relay our transactions
        if s > n.clone() / 2u32 {
            s = n - s;
        }

        Signature::new(&r, &s)
    }
}

#[cfg(test)]
mod tests {
    use num::BigUint;
    use num_bigint::RandBigInt;
    use rand;

    use super::PrivateKey;

    #[test]
    fn test_sign() {
        let n = super::super::secp256k1_params::Secp256k1Params::n();
        let g = super::super::secp256k1_params::Secp256k1Params::g();

        let mut rng = rand::thread_rng();
        let secret = rng.gen_biguint_below(&n);
        let pk = PrivateKey::new(&secret);

        let z = rng.gen_biguint_below(&BigUint::from(2u32).pow(256u32));

        let sig = pk.sign(&z);

        let point = g * secret;

        assert!(point.verify(&z, &sig));
    }
}
