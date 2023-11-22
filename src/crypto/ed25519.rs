
use alloy_primitives::FixedBytes;
use stylus_sdk::abi::Bytes;
use ed25519_compact::{PublicKey, Signature};


/// Verifies an ed25519 signature.
pub fn ed25519_verify(k: FixedBytes<32>, sig: Bytes, msg: Bytes) -> bool {
    let pk: PublicKey = PublicKey::from_slice(k.as_slice()).unwrap();
    let signature = Signature::from_slice(sig.as_slice()).unwrap();
    pk.verify(msg, &signature).is_ok()
}