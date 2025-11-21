<!-- ===========================
     Claim SOL — README.md
     Impressive, high-impact GitHub description with inline SVG logo
     ============================ -->

<!-- Large inline SVG hero/logo (renders on GitHub) -->
<div align="center">
  <!-- 512x128 viewBox for a wide hero -->
  <svg width="760" height="220" viewBox="0 0 760 220" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="Claim SOL logo">
    <defs>
      <linearGradient id="g1" x1="0" x2="1" y1="0" y2="1">
        <stop offset="0%" stop-color="#00d1ff"/>
        <stop offset="100%" stop-color="#6f4cff"/>
      </linearGradient>
      <filter id="drop" x="-50%" y="-50%" width="200%" height="200%">
        <feDropShadow dx="0" dy="8" stdDeviation="18" flood-color="#000000" flood-opacity="0.18"/>
      </filter>
    </defs>

    <!-- Background rounded rectangle -->
    <rect x="8" y="8" rx="28" ry="28" width="744" height="204" fill="white" opacity="0.04"/>

    <!-- Stylized coin / SOL + rocket emblem -->
    <g transform="translate(36,40)" filter="url(#drop)">
      <!-- coin -->
      <circle cx="96" cy="56" r="56" fill="url(#g1)" />
      <!-- SOL glyph (stylized S) -->
      <path d="M72 46 L120 34 L88 86 L136 74" stroke="white" stroke-width="6" stroke-linecap="round" stroke-linejoin="round" fill="none" opacity="0.96"/>
      <!-- small sparkle -->
      <polygon points="140,10 146,22 158,24 148,34 150,46 140,38 130,46 132,34 122,24 134,22" fill="white" opacity="0.18"/>
    </g>

    <!-- Title text -->
    <g transform="translate(220,48)">
      <text x="0" y="56" font-family="Inter, Roboto, Arial" font-size="44" font-weight="800" fill="#ffffff">CLAIM&nbsp;SOL</text>
      <text x="0" y="96" font-family="Inter, Roboto, Arial" font-size="18" font-weight="600" fill="#d3d7ff" opacity="0.95">
        Reclaim locked SOL from unused token accounts — 100% on-chain, gas optimized.
      </text>
    </g>

    <!-- Tagline ribbon -->
    <g transform="translate(220,128)">
      <rect x="0" y="-6" rx="10" ry="10" width="460" height="36" fill="#0f1724" opacity="0.28"/>
      <text x="18" y="18" font-family="Inter, Roboto, Arial" font-size="14" fill="#bcd7ff">
        🔗 On-chain recovery · ⚙️ Anchor (Rust) · 🔒 Secure · ♻️ Reward & Burn
      </text>
    </g>
  </svg>
</div>

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

## 🔧 Quickstart (Developer)

```bash
# Build (example)
anchor build

# Run tests
anchor test

# Deploy (example)
anchor deploy --provider.cluster mainnet

# Interact via Anchor client (JS / TS)
# - Use Anchor IDL
# - Call `claim()` with prepared account list & optional affiliate
