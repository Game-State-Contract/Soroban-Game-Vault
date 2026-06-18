# Contributing to Soroban Game Vault

Thank you for your interest in contributing! This project is part of the **Stellar Wave Program** — issues tagged with `Stellar Wave` may carry Point rewards distributed through [Drips Wave](https://www.drips.network/wave/stellar).

---

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How to Contribute](#how-to-contribute)
- [Development Setup](#development-setup)
- [Project Architecture](#project-architecture)
- [Issue Labels](#issue-labels)
- [Pull Request Guidelines](#pull-request-guidelines)
- [Commit Style](#commit-style)
- [Getting Help](#getting-help)

---

## Code of Conduct

This project follows the [Contributor Covenant](https://www.contributor-covenant.org/). Be respectful, constructive, and collaborative. We're here to build something useful together.

---

## How to Contribute

### 1. Find an Issue

- Browse [open issues](../../issues) — look for `good first issue` if you're new
- Issues tagged `Stellar Wave` are part of the active Wave sprint and carry rewards
- Do **not** start working on an issue until you've been assigned to it (Wave rules)

### 2. Apply via Drips Wave (for Wave issues)

If the issue is tagged `Stellar Wave`:
1. Go to [drips.network/wave](https://www.drips.network/wave) and log in with GitHub
2. Find the issue and submit an application
3. Wait to be assigned by a maintainer before opening any code

### 3. Fork & Branch

```bash
git clone https://github.com/YOUR_ORG/soroban-game-vault.git
cd soroban-game-vault
git checkout -b feat/your-feature-name
```

Branch naming conventions:
- `feat/` — new feature
- `fix/` — bug fix
- `docs/` — documentation only
- `test/` — adding or improving tests
- `refactor/` — code cleanup with no behavior change

### 4. Make Your Changes

- Keep changes focused on the issue — one concern per PR
- Write or update tests for any logic you add or change
- Update relevant docs if your change affects behavior

### 5. Open a Pull Request

- Reference the issue in your PR description: `Closes #42`
- Fill out the PR template
- A maintainer will review within the Wave window

---

## Development Setup

### Requirements

| Tool | Version |
|---|---|
| Rust | stable (latest) |
| Stellar CLI | latest |
| Node.js | ≥ 18 |
| cargo | comes with Rust |

### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown
```

### Install Stellar CLI

```bash
cargo install --locked stellar-cli --features opt
```

### Install dependencies & build

```bash
# Contracts
cargo build

# TypeScript SDK
cd sdk
npm install
```

### Run all tests

```bash
# Rust contract tests
cargo test

# SDK tests
cd sdk && npm test
```

### Deploy to local sandbox

```bash
stellar contract build
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/game_state.wasm \
  --network local
```

---

## Project Architecture

Understanding the codebase before contributing:

```
contracts/game_state/    ← Core contract. Start here.
  src/lib.rs             ← Entry point, public contract functions
  src/storage.rs         ← How encrypted blobs are stored on-chain
  src/verify.rs          ← Condition verification against committed hashes

contracts/crypto_utils/  ← Shared primitives used by game_state
  src/hash.rs            ← Hashing utilities (SHA-256 wrappers)
  src/commit.rs          ← Commit-reveal scheme implementation

sdk/src/                 ← TypeScript client SDK
  encrypt.ts             ← Client-side encryption (runs off-chain)
  vault.ts               ← Wraps contract interactions

examples/                ← Reference implementations for game developers
```

Key design principle: **encryption always happens client-side**. The Soroban contract never sees raw player data — only encrypted blobs and hashes. If you're adding a feature, keep this boundary clean.

---

## Issue Labels

| Label | Meaning |
|---|---|
| `good first issue` | Well-scoped, beginner-friendly |
| `Stellar Wave` | Active in current Wave sprint, carries Point reward |
| `contracts` | Touches Soroban Rust contracts |
| `sdk` | Touches the TypeScript SDK |
| `docs` | Documentation only |
| `security` | Security-sensitive — requires extra review |
| `examples` | Adds or improves example games |
| `bug` | Something is broken |
| `enhancement` | New feature or improvement |

---

## Pull Request Guidelines

- **One issue per PR** — don't bundle unrelated changes
- **Tests are required** for any contract logic change
- **Docs must be updated** if you change a public API or behavior
- PRs that touch `security`-labeled issues require two maintainer approvals
- For Wave issues: your PR must be merged **before the Wave ends** for points to be awarded

### PR Description Template

```
## What does this PR do?
<!-- Brief description of the change -->

## Related Issue
Closes #

## Type of change
- [ ] Bug fix
- [ ] New feature
- [ ] Documentation
- [ ] Refactor

## How was this tested?
<!-- Describe tests added or run -->

## Notes for reviewer
<!-- Anything specific to look at -->
```

---

## Commit Style

We use [Conventional Commits](https://www.conventionalcommits.org/):

```
feat(contracts): add verify_condition function
fix(sdk): handle missing keypair in encryptState
docs: update architecture diagram
test(crypto_utils): add edge cases for commit-reveal
```

---

## Getting Help

- Open a [GitHub Discussion](../../discussions) for questions
- Comment directly on the issue you're working on
- For Wave-related questions, visit [docs.drips.network/wave](https://docs.drips.network/wave)

We're glad you're here. Let's build something the Stellar ecosystem has never seen before.
