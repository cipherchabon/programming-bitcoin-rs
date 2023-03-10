use num::{BigUint, FromPrimitive, Integer, One};
use num_bigint::RandBigInt;
use rand;

/// Check if a BigUint number is prime using the Miller-Rabin primality test.
pub(crate) fn biguint_primality_checker(n: &BigUint) -> bool {
    // If n is 2 or 3, then it is prime
    let two = BigUint::from_u32(2).unwrap();
    let three = BigUint::from_u32(3).unwrap();
    if n == &two || n == &three {
        return true;
    }

    // If n is 1 or even, then it is not prime
    let one = BigUint::one();
    if n == &one || n.is_even() {
        return false;
    }

    // Decompose n-1 into d * 2^s
    let mut d = n - &one;
    let mut s = 0;
    while d.is_even() {
        d >>= 1;
        s += 1;
    }

    // Determine the number of Miller-Rabin iterations required
    let bits = n.bits();
    let k = match bits {
        0..=64 => 7,
        65..=100 => 6,
        101..=128 => 5,
        129..=156 => 4,
        157..=191 => 3,
        192..=256 => 2,
        _ => 1,
    };

    // Perform Miller-Rabin iterations
    let mut rng = rand::thread_rng();
    for _ in 0..k {
        // Generate random number a in range [2, n-2]
        let a = loop {
            let rand = rng.gen_biguint_range(&two, &(n - &two));
            if &rand >= &two {
                break rand;
            }
        };

        // Calculate x = a^d mod n
        let mut x = a.modpow(&d, n);

        // If x is 1 or n-1, continue with next iteration
        if x == one || x == n - &one {
            continue;
        }

        // Perform additional Miller-Rabin iterations
        let mut j = 1;
        while j < s && x != n - &one {
            x = x.modpow(&two, n);
            if x == one {
                return false;
            }
            j += 1;
        }

        // If x is not n-1, then n is not prime
        if x != n - &one {
            return false;
        }
    }

    // If all Miller-Rabin iterations passed, then n is probably prime
    true
}
