# Serialization

There are two forms of SEC format that we need to be concerned with: uncompressed and compressed. 

## Uncompressed SEC Format
Here is how the uncompressed SEC format for a given point `P = (x,y)` is generated:
1. Start with the prefix byte, which is `0x04`.
2. Next, append the `x` coordinate in 32 bytes as a big-endian integer.
3. Next, append the `y` coordinate in 32 bytes as a big-endian integer.

## Compressed SEC Format
Here is the serialization of the compressed SEC format for a given point `P = (x,y)`:
1. Start with the prefix byte. If `y` is even, it’s `0x02`; otherwise, it’s `0x03`.
2. Next, append the `x` coordinate in 32 bytes as a big-endian integer.

Calculate `y` given the `x` coordinate requires us to calculate a square root in a finite field: 

if `w2 = v` and `p % 4 = 3`, `w = v(p+1)/4`

## DER Signatures

Distinguished Encoding Rules (DER)

DER format is defined like this:
1. Start with the `0x30` byte.
2. Encode the length of the rest of the signature (usually `0x44` or `0x45`) and append.
3. Append the marker byte, `0x02`.
4. Encode `r` as a big-endian integer, but prepend it with the 0x00 byte if `r`’s first byte ≥ `0x80`. Prepend the resulting length to `r`. Add this to the result.
5. Append the marker byte, `0x02`.
6. Encode `s` as a big-endian integer, but prepend with the `0x00` byte if s’s first byte ≥ `0x80`. Prepend the resulting length to `s`. Add this to the result.

The rules for #4 and #6 with the first byte starting with something greater than or equal to `0x80` are because DER is a general encoding and allows for negative numbers to be encoded. The first bit being 1 means that the number is negative. All numbers in an ECDSA signature are positive, so we have to prepend with `0x00` if the first bit is zero, which is equivalent to first byte ≥ `0x80`.

## Base58

`BASE58_ALPHABET = '123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz'`

## Address format
The 264 bits from compressed SEC format are still a bit too long, not to mention a bit less secure. To both shorten the address and increase security, we can use the ripemd160 hash.

By not using the SEC format directly, we can go from 33 bytes to 20 bytes, shortening the address significantly. Here is how a Bitcoin address is created:

1. For mainnet addresses, start with the prefix 0x00, for testnet `0x6f`.
2. Take the SEC format (compressed or uncompressed) and do a sha256 operation followed by the ripemd160 hash operation, the combination of which is called a hash160 operation.
3. Combine the prefix from #1 and resulting hash from #2.
4. Do a hash256 of the result from #3 and get the first 4 bytes. (checksum)
5. Take the combination of #3 and #4 and encode it in Base58.

## WIF Format
Wallet Import Format is a serialization of the private key that’s meant to be human-readable. WIF uses the same Base58 encoding that addresses use.

Here is how the WIF format is created:
1. For mainnet private keys, start with the prefix `0x80`, for testnet `0xef`. 
2. Encode the secret in 32-byte big-endian.
3. If the SEC format used for the public key address was compressed, add a suffix of `0x01`.
4. Combine the prefix from #1, serialized secret from #2, and suffix from #3.
5. Do a hash256 of the result from #4 and get the first 4 bytes.
6. Take the combination of #4 and #5 and encode it in Base58.