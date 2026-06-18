# Architecture

## Core Design Principle

**Encryption always happens client-side.**

The Soroban contract is a verifiable storage and integrity layer — it never sees raw player data. All sensitive information is encrypted by the player's own keypair before it ever touches the blockchain.

## Data Flow

```
┌─────────────────────────────────────────────────────────────┐
│                        CLIENT (off-chain)                    │
│                                                             │
│  Player State (plaintext)                                   │
│  { gold: 420, level: 12, inventory: [...] }                 │
│           │                                                 │
│           ▼                                                 │
│  encrypt(state, playerPrivateKey)  →  encrypted_blob        │
│  sha256(state)                     →  state_hash            │
│           │                                                 │
└───────────┼─────────────────────────────────────────────────┘
            │
            ▼  store_state(player, encrypted_blob, state_hash)
┌─────────────────────────────────────────────────────────────┐
│                   SOROBAN CONTRACT (on-chain)                │
│                                                             │
│  Stores: { encrypted_blob, state_hash, updated_at }         │
│                                                             │
│  verify_condition(player, condition) →                      │
│    recompute proof from state_hash + condition params       │
│    compare with client-provided proof                       │
│    return true/false (no decryption needed)                 │
└─────────────────────────────────────────────────────────────┘
```

## Condition Verification

The key innovation is verifying conditions *without decryption*:

1. Client computes `proof = SHA-256(state_hash || field || op || value)` off-chain
2. Client submits the condition + proof to the contract
3. Contract recomputes the same hash using the stored `state_hash`
4. If they match, the condition is proven valid

This works because the `state_hash` is a binding commitment to the full state — it can't be changed without the player's signature, and the proof ties a specific condition claim to that commitment.

## Known Limitations

- The scheme provides *computational* privacy, not full ZK privacy. A sophisticated adversary with access to the encrypted blob and sufficient compute could attempt decryption.
- `state_hash` leaks the *number* of state updates (via `updated_at`), not the state itself.
- This is not a substitute for a full ZK proof system. It is a pragmatic, Soroban-native approach to hidden game state.

See [security.md](./security.md) for the full threat model.
