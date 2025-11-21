
🪙 Claim SOL — On-Chain SOL Recovery Protocol

Claim SOL is a fully on-chain Solana protocol that enables users to reclaim SOL locked inside unused token accounts (SPL & Token-2022).
All logic — scanning, reclaiming, rewarding, burning, and affiliate payouts — is executed entirely on-chain with no backend, no cron jobs, and zero trust assumptions.

⸻

✨ Overview

Billions of lamports are trapped across abandoned token accounts on Solana.
Claim SOL unlocks this value by:
	•	Aggregating unused token accounts
	•	Reclaiming the rent lamports
	•	Automatically distributing rewards
	•	Burning the protocol token
	•	Paying affiliates
	•	All in one atomic instruction

The result? A fast, gas-optimized, censorship-resistant tool for reclaiming lost SOL.

⸻

🚀 Core Features

🔹 1. Multi-Account SOL Recovery

Reclaim SOL from up to 25 abandoned token accounts per transaction, with optimized compute usage and zero off-chain scanning.

🔹 2. Reward, Burn & Fee Logic (All On-Chain)

Each successful reclaim automatically:
	•	Rewards the user in $CLAIM
	•	Burns a protocol-defined percentage
	•	Pays developer and affiliate fees
	•	Emits structured events for indexing
	•	Runs with no backend — purely Solana instructions + Anchor CPI

🔹 3. Integrated Affiliate System

A built-in, permissionless affiliate model:
	•	Affiliates earn 4% of every claim made by referred users
	•	All tracked via deterministic PDAs
	•	No manual management, no off-chain ledger

🔹 4. Token-2022 Compatible

Claim SOL supports:
	•	SPL Tokens
	•	Token-2022 accounts
	•	Mixed wallets holding both standards

⸻

⚙️ Technical Architecture

Component	Details
Language	Rust (Anchor v0.30.1)
Programs Used	system_program, token_interface, associated_token
Supported Token Standards	SPL + Token-2022
Data PDAs	Config, AffiliateStats
Events	ClaimEvent, DevPaidEvent, AffiliatePaidEvent
Safety	Owner checks, account validation, overflow guards, signer seeds
Compute Usage	~60k CU per claim (25 accounts)

Program Workflow
	1.	Validate + filter abandoned token accounts
	2.	Reclaim rent lamports → pool
	3.	Distribute SOL & tokens according to config
	4.	Burn $CLAIM using CPI
	5.	Emit events for analytics
	6.	Update affiliate stats via zero-copy PDAs

⸻

💰 Fee Structure

Receiver	Allocation	Purpose
User	80%	Net SOL reclaim reward
Developer	16%	Protocol maintenance
Affiliate	4%	Referral incentives

All percentages are configurable via admin authority.

⸻

📦 Instructions

Instruction	Functionality
initialize_config	Creates program config + sets global authority
claim	Reclaim SOL, reward, burn, distribute fees (single atomic call)
reset	Reset claim counters — authority-only
set_authority	Transfer config authority securely


⸻

🧠 Design Principles
	•	No backend — purely on-chain
	•	Deterministic, auditable reward distribution
	•	Minimal compute & account usage
	•	Zero-copy PDAs for affiliate performance
	•	Token-2022 native support
	•	Explicit error codes for better DX

⸻

🧩 Roadmap
	•	Web dashboard for recovered SOL analytics
	•	Global leaderboard (top claimers & affiliates)
	•	Mobile app & QR-scan claiming
	•	Support for batching >25 accounts with CU delegation
	•	Auto-tracking abandonment patterns using lightweight indexing

⸻

📄 License

MIT License © 2025 — Claim SOL Protocol

⸻

🔗 Official Links
	•	🌐 Website: Coming Soon
	•	🐦 Twitter/X: @ClaimSOL￼
	•	💬 Telegram: https://t.me/ClaimSOL

⸻

⭐ Like the project?

Give the repo a star, contribute improvements, or build tools on top — the protocol is open and permissionless.

⸻
