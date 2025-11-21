
🪙 CLAIM SOL — THE ON-CHAIN SOLANA RECOVERY PROTOCOL

Claim SOL is a fully on-chain Solana protocol that unlocks trapped SOL inside abandoned token accounts (SPL + Token-2022).
All scanning, recovering, rewarding, burning, and affiliate payouts happen inside one atomic transaction, with zero backend, zero trust, and zero dependencies.

This is how SOL gets reclaimed — the right way: fast, optimized, unstoppable.

⸻

🚀 CORE FEATURES

🔥 1. High-Throughput SOL Recovery

Reclaim SOL from up to 25 abandoned accounts per transaction, with compute-optimized logic built for speed and scale.

💎 2. Autonomous Rewards + Burn

Every claim triggers:
	•	User reward in $CLAIM
	•	Token burn (deflationary pressure)
	•	Dev fee payment
	•	Affiliate fee distribution
	•	Structured events for indexing

All on-chain. No backend servers. No cron jobs.

🤝 3. Built-In Affiliate Engine

A fully permissionless referral system:
	•	4% affiliate share
	•	Deterministic zero-copy PDAs
	•	Automatic distribution on every claim
	•	No manual tracking, no admin work

🛡️ 4. SPL + Token-2022 Compatible

Works seamlessly with:
	•	SPL Token
	•	Token-2022
	•	Mixed wallets
	•	Large abandoned account lists

⸻

⚙️ TECHNICAL ARCHITECTURE

🧩 Program Stack

Component	Details
Language	Rust (Anchor v0.30.1)
Programs	system_program, token_interface, associated_token
Standards	SPL Token + Token-2022
Data PDAs	Config, AffiliateStats
Events	ClaimEvent, DevPaidEvent, AffiliatePaidEvent
Security	Owner validation, overflow guards, signer seeds
Compute Usage	~60k CU for 25-account batch


⸻

⚡ FEE DISTRIBUTION MODEL

Receiver	Allocation	Purpose
User	80%	Main SOL reward
Developer	16%	Protocol operation fee
Affiliate	4%	Referral rewards

Everything distributed on-chain, atomically.

⸻

🧰 PROGRAM INSTRUCTIONS

⚙️ initialize_config

Set the global configuration and authority.

⚡ claim

Execute recovery + reward + burn + fee distribution in a single atomic CPI.

🔄 reset

Reset internal counters (authority-only).

🔑 set_authority

Transfer program config authority safely.

⸻

🧠 DESIGN PRINCIPLES

✅ 100% On-Chain

No backend, no servers, no off-chain logic.

✅ Deterministic Execution

Every instruction is predictable, auditable, and fully transparent.

✅ Zero-Copy PDA Storage

Efficient affiliate tracking at scale.

✅ Token-2022 Native

Handles all modern Solana token standards.

✅ Optimized for Speed

Minimal compute, minimal accounts, maximal throughput.

⸻

🔮 ROADMAP

📊 Dashboard

Visualize SOL reclaimed, wallet history, affiliate statistics.

🏆 Leaderboard

Top claimers, highest earners, biggest affiliates.

📱 Mobile dApp

One-tap recovery for mass users.

🧩 CU-Extended Batch Claims

Support for >25 accounts using CU delegation.

⸻

📄 LICENSE

MIT License © 2025 — Claim SOL Protocol

⸻

🔗 OFFICIAL LINKS
	•	🌐 Website — Coming Soon
	•	🐦 X — https://x.com/ClaimSOL
	•	💬 Telegram — Coming Soon

⸻

⭐ SUPPORT THE PROJECT

If Claim SOL helps you reclaim value,
give the repo a star ⭐ and help expand the movement of recovering abandoned SOL on-chain.

