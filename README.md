
# CLAIM SOL — Reclaim Locked SOL, Fully On-Chain 🚀

**Claim SOL** is a high-performance on-chain protocol on **Solana** that helps users *recover SOL* trapped in unused SPL or Token-2022 accounts — all **without any off-chain backend**.  
Designed for security, scale, and gas efficiency.

---

## ✨ Hero Features (Big & Bold)

- **On-Chain Bulk Recovery** — Reclaim SOL from up to **25 accounts** in a single transaction (gas-optimized).
- **Automatic Reward + Burn** — Each successful claim issues `$CLAIM` rewards, burns supply for deflation, and distributes developer/affiliate fees — all on-chain.
- **Built-in Affiliate System** — Transparent 4% affiliate payouts via zero-copy PDAs.
- **Token-2022 Compatible** — Full support for SPL and Token-2022 standards.
- **No Backend Required** — All logic runs inside the Anchor programs using native Solana instructions & CPI.

---

## 🚨 Why Claim SOL Matters

Many wallets accumulate **unused token accounts** with dust-like SOL locked for rent-exemption. Claim SOL consolidates and returns that value — giving power back to users and aligning incentives for reclaimers and referrers.

---

## 🛠️ Technical Overview (At a Glance)

| Component | Details |
|---|---|
| **Language** | Rust (Anchor v0.30.1) |
| **Programs** | `system_program`, `token_interface`, `associated_token` |
| **Standards** | SPL Token, Token-2022 |
| **Key PDAs** | `Config` PDA, `AffiliateStats` PDA (zero-copy) |
| **Security Checks** | Owner validation, decimals check, signer seeds, overflow guards |
| **Events** | `ClaimEvent`, `DevPaidEvent`, `AffiliatePaidEvent` |
| **Compute** | ~60k CU per claim (optimized) |

---

## 💸 Fee Breakdown (Transparent)

| Recipient | Share |
|---:|:---:|
| **User (Claimer)** | **80%** |
| **Developer** | **16%** |
| **Affiliate (referrer)** | **4%** |

---

## 📦 Program API — Functions

- `initialize_config(authority, params)`  
  Create global config and program authority (admin only).

- `claim(accounts: [token_accounts...], affiliate_opt: Option<Pubkey>)`  
  Core operation — reclaims SOL from provided accounts, mints/credits `$CLAIM`, burns portion, distributes fees.

- `reset()`  
  Reset program counters (admin only).

- `set_authority(new_authority)`  
  Transfer administrative control (admin only).

---

## 🧠 Design Highlights

- **Zero-copy PDAs** for scalable affiliate tracking (minimizes compute + memory costs).  
- **Anchor CPI** usage for secure cross-program calls.  
- **Descriptive error codes** for developer ergonomics & analytics.  
- **Gas-conscious** account iteration and batching (25 accounts per tx).

---

## 🧪 Example: Claim Flow (Simplified)

1. User calls `claim()` with up to 25 candidate token accounts.  
2. Program verifies each account owner, token decimals, and rent-exempt status.  
3. Program transfers SOL back to claimer, mints/credits `$CLAIM` reward, burns configured portion, and pays dev/affiliate PDAs.  
4. Program emits `ClaimEvent` with totals & affiliate data.

---

## 🔒 Security Notes

- All transfers validated against account owner and token metadata.  
- Overflow/underflow guards on arithmetic.  
- Admin functions protected by authority PDA and explicit checks.  
- Recommended: on-chain audits and fuzz tests prior to mainnet deployment.

---
