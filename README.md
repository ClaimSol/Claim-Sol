# 🪙 Claim SOL

**Claim SOL** is an innovative on-chain protocol built on Solana that allows users to **reclaim SOL** locked inside **unused token accounts** (SPL or Token-2022).  
Each recovery transaction is rewarded, burned, and distributed automatically through smart logic — **no off-chain backend needed**.

---

## 🚀 Features

### 🔹 1. On-Chain SOL Recovery
Reclaim SOL from up to **25 unused accounts** in a single transaction — directly on-chain and gas-optimized.

### 🔹 2. Reward & Burn Mechanism
Every successful claim:
- **Rewards users** in `$CLAIM` tokens  
- **Burns a portion** of tokens to maintain deflationary pressure  
- **Pays developer and affiliate fees** on-chain automatically  

### 🔹 3. Built-In Affiliate System
Referrers earn **4% of each successful claim** by their invited users — all tracked transparently on-chain with zero-copy PDAs.

### 🔹 4. Fully On-Chain Distribution
No intermediaries, no backend.  
All SOL and token transfers use Solana’s native instructions and Anchor CPI calls.

---

## ⚙️ Technical Overview

| Component | Description |
|------------|-------------|
| **Language** | Rust (Anchor Framework v0.30.1) |
| **Programs Used** | `system_program`, `token_interface`, `associated_token` |
| **Supported Standards** | SPL Token & Token-2022 |
| **Security** | Checked owners, decimals, signer seeds, overflow guards |
| **Data Accounts** | `Config` PDA, `AffiliateStats` PDA |
| **Event Tracking** | `ClaimEvent`, `DevPaidEvent`, `AffiliatePaidEvent` |

---

## 💰 Fee Distribution

| Role | Percentage | Description |
|------|-------------|-------------|
| **User** | 80% | Main SOL reclaim reward |
| **Developer** | 16% | System maintenance fee |
| **Affiliate** | 4% | Optional referral incentive |

---

## 📦 Instructions

| Function | Description |
|-----------|-------------|
| `initialize_config` | Sets global configuration and authority |
| `claim` | Performs the reclaim + reward + burn in one transaction |
| `reset` | Resets program counters (admin-only) |
| `set_authority` | Transfers config authority (admin-only) |

---

## 🧠 Design Highlights
- Full Token-2022 compatibility  
- Zero-copy PDAs for scalable affiliate tracking  
- Efficient compute unit usage (≈60k per claim)  
- Descriptive error codes for debugging and analytics  

---

## 🧩 Future Integrations
- Web dashboard to visualize SOL reclaimed per wallet  
- Leaderboard for top affiliates and claimers  
- Mobile dApp integration for one-click recoveries  

---

## 📄 License
MIT License © 2025 Claim SOL Developers  
Built for the Solana ecosystem with ❤️

---

## 🔗 Socials
- 🌐 Website: [Coming Soon]  
- 🐦 Twitter/X: [@ClaimSOL](https://x.com/ClaimSOL)  
- 💬 Telegram: [Claim SOL Community](https://t.me/ClaimSOL)

---

### ⭐ Support
If you find this protocol useful, give the repo a **star** ⭐ and help spread the word about reclaiming lost SOL on-chain!
