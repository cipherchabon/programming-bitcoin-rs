use num::BigUint;
use programming_bitcoin::{ecc::private_key::PrivateKey, utils::hash256::hash256};

fn main() {
    let passphrase = "cypherchabon secret";
    let bytes = passphrase.as_bytes();

    let hash256 = hash256(bytes);

    let secret = BigUint::from_bytes_le(&hash256);

    let pk = PrivateKey::new(&secret);

    let wif = pk.to_wif(true, true);

    println!("WIF: {}", wif);
}

// WIF: cTjSqQCzDC1A6xkmDCoACqgtamP5uw1yGZGcfz3wmxPJ3b8riQxb
// ADDR: mpq2it3Q9esxbrEGMrs7nWxnRRqpsfYN3L
// 0.00012091

// fn main() {
//     let passphrase = "cypherchabon secret";
//     let bytes = passphrase.as_bytes();

//     let hash256 = hash256(bytes);

//     let secret = BigUint::from_bytes_le(&hash256);

//     let pk = Secp256k1Params::g() * secret;

//     let addr = pk.get_address(true, true);

//     println!("Address: {}", addr);
// }
