use num::BigUint;
use num_bigint::RandBigInt;
use rand;
use rfc6979::consts::U32;
use sha2::{digest::generic_array::GenericArray, Digest, Sha256};

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

        let k = self.deterministic_k(message);

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

    // see https://docs.rs/rfc6979/0.4.0/rfc6979/
    fn deterministic_k(&self, z: &BigUint) -> BigUint {
        let p_bytes = Secp256k1Params::n().to_bytes_be();
        let mut p = GenericArray::<u8, U32>::default();
        p.copy_from_slice(p_bytes.as_slice());

        let k_bytes = self.secret.to_bytes_be();
        let mut k = GenericArray::<u8, U32>::default();
        k.copy_from_slice(&k_bytes);

        let z_bytes = z.to_bytes_be();
        let mut z = GenericArray::<u8, U32>::default();
        z.copy_from_slice(&z_bytes);

        let h = Sha256::digest(&z);

        let k = rfc6979::generate_k::<Sha256, U32>(&k.into(), &p.into(), &h, b"");

        BigUint::from_bytes_be(&k)
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
