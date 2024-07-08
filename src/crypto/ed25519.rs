
use alloy_primitives::FixedBytes;
use ed25519_compact::{PublicKey, Signature};


/// Verifies an ed25519 signature.
pub fn ed25519_verify(k: FixedBytes<32>, sig: Vec<u8>, msg: Vec<u8>) -> bool {
    let pk: PublicKey = PublicKey::from_slice(k.as_slice()).unwrap();
    let signature = Signature::from_slice(sig.as_slice()).unwrap();
    pk.verify(msg, &signature).is_ok()
}