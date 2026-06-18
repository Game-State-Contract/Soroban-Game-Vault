use soroban_sdk::{Bytes, BytesN, Env};

/// Hash arbitrary bytes using SHA-256, returning a 32-byte digest.
pub fn sha256(env: &Env, data: &Bytes) -> BytesN<32> {
    env.crypto().sha256(data)
}

/// Hash two byte arrays concatenated — useful for building Merkle-style proofs.
pub fn hash_concat(env: &Env, a: &Bytes, b: &Bytes) -> BytesN<32> {
    let mut combined = Bytes::new(env);
    combined.append(a);
    combined.append(b);
    env.crypto().sha256(&combined)
}
