use soroban_sdk::{Bytes, BytesN, Env};
use crate::hash::sha256;

/// Create a commitment: SHA-256(value || nonce)
///
/// The nonce should be a random 32-byte value kept secret by the committer
/// until reveal time. This prevents an adversary from brute-forcing the value.
pub fn create_commitment(env: &Env, value: &Bytes, nonce: &BytesN<32>) -> BytesN<32> {
    let mut preimage = Bytes::new(env);
    preimage.append(value);
    preimage.extend_from_slice(&nonce.to_array());
    sha256(env, &preimage)
}

/// Verify a reveal: recompute commitment and check it matches.
///
/// Returns true if SHA-256(value || nonce) == commitment.
pub fn verify_reveal(
    env: &Env,
    commitment: &BytesN<32>,
    value: &Bytes,
    nonce: &BytesN<32>,
) -> bool {
    let recomputed = create_commitment(env, value, nonce);
    recomputed == *commitment
}
