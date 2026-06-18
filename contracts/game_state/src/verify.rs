use soroban_sdk::{contracttype, BytesN, Env};
use crate::Condition;

/// Supported comparison operators for condition verification
#[contracttype]
pub enum ConditionOp {
    Gte,  // >=
    Lte,  // <=
    Eq,   // ==
    Neq,  // !=
}

/// Verify that a condition proof is valid against a stored state hash.
///
/// The proof must equal: SHA-256(state_hash || field || op_byte || value)
/// This is computed client-side and verified here without revealing raw state.
pub fn check_condition(env: &Env, state_hash: &BytesN<32>, condition: &Condition) -> bool {
    // Reconstruct the expected proof from the stored hash + condition params
    let mut preimage = soroban_sdk::Bytes::new(env);
    preimage.extend_from_slice(&state_hash.to_array());
    preimage.append(&condition.field);
    preimage.push_back(op_to_byte(&condition.op));
    preimage.append(&condition.value);

    let expected = env.crypto().sha256(&preimage);
    expected == condition.proof
}

fn op_to_byte(op: &ConditionOp) -> u8 {
    match op {
        ConditionOp::Gte => 0,
        ConditionOp::Lte => 1,
        ConditionOp::Eq  => 2,
        ConditionOp::Neq => 3,
    }
}
