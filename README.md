# 🔒 Soroban Game Vault

> Private, verifiable game state management for on-chain games — built on Stellar's Soroban smart contract platform.

---

## The Problem

On-chain games have a fundamental tension: **blockchain data is public, but games need hidden information.**

A player's inventory, stats, progress, and strategy should be *their* data — not readable by opponents, bots, or exploiters. Today, game developers either:

- Store state off-chain (sacrificing verifiability), or
- Store it on-chain in plaintext (sacrificing privacy)

There is no middle ground. Until now.

---

## What is Soroban Game Vault?

**Soroban Game Vault** is a Soroban smart contract library that lets game developers store *encrypted* player state on-chain — where:

- ✅ Only the player can decrypt their own data (inventory, stats, progress)
- ✅ The contract can still **verify conditions** (e.g. "does this player have ≥ 50 XLM?") without ever exposing raw values
- ✅ All state transitions are on-chain, auditable, and tamper-proof
- ✅ No trusted third party or oracle required

Think hidden-information games — poker, strategy, RPGs — where what you *can prove* matters as much as what you *know*.

---

## How It Works

Soroban Game Vault uses a **commit-reveal + encrypted storage** model:

```
Player Action
     │
     ▼
Encrypt state client-side (player's key)
     │
     ▼
Store encrypted blob + state hash on Soroban contract
     │
     ▼
Contract verifies hash integrity on every update
     │
     ▼
Condition checks run against committed hash proofs
     (e.g. "prove balance ≥ X" without revealing balance)
```

---

## Features

| Feature | Description |
|---|---|
| `encrypt_state` | Encrypt arbitrary player state before storing on-chain |
| `verify_condition` | Verify a condition against encrypted state without decrypting |
| `commit_state` | Commit a hash of state for tamper-proof change tracking |
| `reveal_state` | Controlled reveal mechanism for end-of-game resolution |
| `transfer_ownership` | Hand off encrypted state (e.g. item trades between players) |

---

## Project Structure

```
soroban-game-vault/
├── contracts/
│   ├── game_state/          # Core encrypted state storage contract
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs       # Main contract logic
│   │       ├── storage.rs   # On-chain storage helpers
│   │       ├── verify.rs    # Condition verification logic
│   │       └── test.rs      # Unit tests
│   └── crypto_utils/        # Shared cryptographic primitives
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── hash.rs      # Hashing utilities
│           └── commit.rs    # Commit-reveal scheme
├── sdk/                     # Client-side SDK (TypeScript)
│   ├── src/
│   │   ├── index.ts
│   │   ├── encrypt.ts       # Client-side encryption helpers
│   │   └── vault.ts         # Contract interaction wrapper
│   └── package.json
├── examples/
│   ├── basic_inventory/     # Example: private RPG inventory
│   └── hidden_stats/        # Example: hidden player stats in a strategy game
├── docs/
│   ├── architecture.md      # Deep-dive on design decisions
│   ├── security.md          # Threat model and known limitations
│   └── getting-started.md   # Quickstart guide
├── .github/
│   └── ISSUE_TEMPLATE/
│       ├── bug_report.md
│       └── feature_request.md
├── Cargo.toml               # Rust workspace
└── README.md
```

---

## Quickstart

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable)
- [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools/cli/stellar-cli)
- Node.js ≥ 18 (for the TypeScript SDK)

### 1. Clone the repo

```bash
git clone https://github.com/YOUR_ORG/soroban-game-vault.git
cd soroban-game-vault
```

### 2. Build the contracts

```bash
stellar contract build
```

### 3. Run tests

```bash
cargo test
```

### 4. Deploy to Testnet

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/game_state.wasm \
  --network testnet
```

### 5. Use the TypeScript SDK

```typescript
import { GameVault } from 'soroban-game-vault-sdk';

const vault = new GameVault({ contractId: 'YOUR_CONTRACT_ID', network: 'testnet' });

// Encrypt and store player inventory
await vault.encryptState(playerKeypair, {
  inventory: ['sword_of_dawn', 'health_potion_x3'],
  gold: 420,
  level: 12,
});

// Verify a condition without revealing state
const hasEnoughGold = await vault.verifyCondition(playerKeypair, {
  field: 'gold',
  operator: '>=',
  value: 100,
});
```

---

## Roadmap

- [ ] Core `game_state` contract (encrypted storage + commit)
- [ ] `verify_condition` — prove state conditions without decryption
- [ ] TypeScript SDK with client-side encryption
- [ ] `basic_inventory` example game
- [ ] `hidden_stats` example game
- [ ] Security audit
- [ ] Mainnet deployment guide

---

## Contributing

We welcome contributors of all levels. See [CONTRIBUTING.md](./CONTRIBUTING.md) to get started.

This project participates in the **[Stellar Wave Program](https://www.drips.network/wave/stellar)** — meaning open issues may carry Point rewards for contributors.

---

## License

MIT © Soroban Game Vault Contributors
