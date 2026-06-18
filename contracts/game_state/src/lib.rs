#![no_std]

//! # Game State Contract
//!
//! Stores encrypted player state on-chain. Encryption always happens
//! client-side — this contract never sees raw player data, only
//! encrypted blobs and their hashes.
//!
//! ## Usage
//!
//! 1. Client encrypts player state using their keypair (off-chain)
//! 2. Client calls `store_state` with the encrypted blob + hash
//! 3. Contract stores the blob and records the hash for integrity checks
//! 4. Client can call `verify_condition` to prove a condition holds
//!    without revealing the underlying data

use soroban_sdk::{contract, contractimpl, contracttype, Address, Bytes, BytesN, Env};

mod storage;
mod verify;

pub use verify::ConditionOp;

/// Encrypted state entry stored per player
#[contracttype]
pub struct StateEntry {
    /// Encrypted blob (client-side encrypted, opaque to contract)
    pub encrypted_blob: Bytes,
    /// SHA-256 hash of the plaintext state (for integrity verification)
    pub state_hash: BytesN<32>,
    /// Ledger sequence at last update
    pub updated_at: u32,
}

/// A condition to verify against committed state
#[contracttype]
pub struct Condition {
    /// Field name in the state (e.g. "gold", "level")
    pub field: Bytes,
    /// Comparison operator
    pub op: ConditionOp,
    /// Value to compare against (as bytes, interpreted by client)
    pub value: Bytes,
    /// Hash proof provided by client proving the condition holds
    pub proof: BytesN<32>,
}

#[contract]
pub struct GameStateContract;

#[contractimpl]
impl GameStateContract {
    /// Store encrypted player state on-chain.
    ///
    /// # Arguments
    /// * `player`         - The player's Stellar address (must sign the tx)
    /// * `encrypted_blob` - Client-side encrypted state (opaque bytes)
    /// * `state_hash`     - SHA-256 hash of the plaintext state
    pub fn store_state(
        env: Env,
        player: Address,
        encrypted_blob: Bytes,
        state_hash: BytesN<32>,
    ) {
        player.require_auth();

        let entry = StateEntry {
            encrypted_blob,
            state_hash,
            updated_at: env.ledger().sequence(),
        };

        storage::save_state(&env, &player, &entry);
    }

    /// Retrieve the encrypted state blob for a player.
    /// Returns None if no state has been stored yet.
    pub fn get_state(env: Env, player: Address) -> Option<StateEntry> {
        storage::load_state(&env, &player)
    }

    /// Verify a condition against a player's committed state hash,
    /// without decrypting or revealing the underlying data.
    ///
    /// The client provides a `proof` — a hash of (state_hash || field || op || value)
    /// that the contract verifies matches the stored state hash.
    ///
    /// Returns true if the condition proof is valid.
    pub fn verify_condition(env: Env, player: Address, condition: Condition) -> bool {
        let entry = match storage::load_state(&env, &player) {
            Some(e) => e,
            None => return false,
        };

        verify::check_condition(&env, &entry.state_hash, &condition)
    }

    /// Commit a new state hash without updating the encrypted blob.
    /// Useful for lightweight state transitions that don't change the full blob.
    pub fn commit_hash(env: Env, player: Address, new_hash: BytesN<32>) {
        player.require_auth();

        let mut entry = storage::load_state(&env, &player)
            .unwrap_or_else(|| panic!("no state found for player"));

        entry.state_hash = new_hash;
        entry.updated_at = env.ledger().sequence();

        storage::save_state(&env, &player, &entry);
    }

    /// Delete a player's state (e.g. on game reset or account deletion).
    pub fn clear_state(env: Env, player: Address) {
        player.require_auth();
        storage::delete_state(&env, &player);
    }
}

#[cfg(test)]
mod test;
