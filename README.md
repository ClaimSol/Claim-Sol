# Claim SOL — On-Chain Program

Source code for the **Claim SOL** Anchor program deployed to Solana mainnet.

- **Program ID:** [`CLaim2U9C1AYg1APkb16PdxFVgeqSwzT9HDVESHhAxTt`](https://solscan.io/account/CLaim2U9C1AYg1APkb16PdxFVgeqSwzT9HDVESHhAxTt)
- **Website:** https://claimsol.pro
- **dApp Store:** Listed on the Solana dApp Store (Seeker)
- **X / Twitter:** [@ClaimSol_](https://x.com/ClaimSol_)
- **Telegram:** [t.me/ClaimSolana](https://t.me/ClaimSolana)

This repository contains **only the on-chain program source**. The Claim SOL frontend, backend, Telegram bot, and operational scripts are proprietary and not included here.

## What the program does

Claim SOL closes zero-balance SPL and Token-2022 token accounts on Solana, returning the locked SOL rent to the user. Per closed account, the program:

- Returns **80%** of the reclaimed rent to the user
- Routes **16%** to the protocol dev wallet
- Routes **4%** to an optional referrer (or to the protocol if no/invalid referrer is provided)
- Mints **25 $CLAIM** (Token-2022) to the user from the rewards vault
- **Burns 25 $CLAIM** from the rewards vault (1:1 reward-to-burn deflation)

All state — protocol config, affiliate stats, leaderboard, and airdrop ranks — lives in on-chain PDAs (`Config`, `AffiliateStats`, `RankProof`). There is no off-chain database.

Batch size cap: **≤ 25 accounts per claim transaction**.

### Token

- **$CLAIM mint** (Token-2022): [`CLaimZUmwA5jQgh24jNPqNfD7YxR8ZKnhRUzs4xsizK9`](https://solscan.io/account/CLaimZUmwA5jQgh24jNPqNfD7YxR8ZKnhRUzs4xsizK9)
- Fixed supply: 1,000,000,000 $CLAIM
- Mint authority disabled

## Instructions

| Instruction | Purpose |
|---|---|
| `initialize_config` | One-time setup of the program config PDA |
| `update_config`, `reset_config` | Admin maintenance of fee splits and counters |
| `initialize_affiliate_stats` | Create per-wallet affiliate stats PDA |
| `claim` | Core: closes batched zero-balance accounts, splits rent, mints + burns $CLAIM, updates stats |
| `snapshot_genesis`, `snapshot_titan` | Lock snapshot slots for airdrops |
| `initialize_genesis_rank`, `initialize_titan_rank` | Create rank-proof PDAs for eligible wallets |
| `claim_genesis_drop`, `claim_titan_drop` | Tiered Genesis / Titan airdrop claims |

## Toolchain

- **Anchor**: 0.29.0
- **Solana**: 1.18.26
- **Rust**: edition 2021

These versions must match exactly to reproduce the deployed bytecode.

## Reproducible build & verification

The program is built and verified via [`solana-verify`](https://github.com/Ellipsis-Labs/solana-verifiable-build), which uses a pinned Docker image to produce a byte-for-byte reproducible build.

```bash
# One-time install
cargo install solana-verify

# Build inside the pinned Docker image
solana-verify build --library-name claim_sol

# Compare hashes — these must match
solana-verify get-executable-hash target/deploy/claim_sol.so
solana-verify get-program-hash CLaim2U9C1AYg1APkb16PdxFVgeqSwzT9HDVESHhAxTt
```

To submit the verification to OtterSec / the public Solana verified-builds registry:

```bash
solana-verify verify-from-repo https://github.com/ClaimSol/Claim-Sol \
  --program-id CLaim2U9C1AYg1APkb16PdxFVgeqSwzT9HDVESHhAxTt \
  --library-name claim_sol \
  --remote
```

## Upgrade authority

The program is **upgradeable**. The on-chain upgrade authority is publicly visible via `solana program show CLaim2U9C1AYg1APkb16PdxFVgeqSwzT9HDVESHhAxTt`. Upgrades are governed by the Claim SOL team for security fixes and improvements.

## Security

- Security contact: **info@claimsol.pro**
- Responsible-disclosure policy: https://claimsol.pro/license
- The deployed binary contains an embedded `security_txt!` block with structured contact info (visible via [`query-security-txt`](https://github.com/neodyme-labs/solana-security-txt#querying-a-program-for-security-txt-information) or by inspecting the program's `.security_txt` ELF section).
- No audits at this time.

## License

The on-chain program source is released under the **PolyForm Noncommercial License 1.0.0** — source-available, no commercial use without permission. See [LICENSE](./LICENSE).

The Claim SOL frontend (React/Vite), backend (Node.js/Express), Telegram bot (Telegraf), operational scripts, and brand assets are **proprietary** and not included in this repository.

## Contact

| Channel | |
|---|---|
| General | info@claimsol.pro |
| Security disclosures | info@claimsol.pro |
| X / Twitter | [@ClaimSol_](https://x.com/ClaimSol_) |
| Telegram community | [t.me/ClaimSolana](https://t.me/ClaimSolana) |
| Telegram wallet-check bot | [t.me/ClaimSol_Check_Bot](https://t.me/ClaimSol_Check_Bot) |

---

**Required notice (per LICENSE):** Copyright Claim SOL (https://claimsol.pro)
