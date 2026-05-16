use anchor_lang::prelude::*;  
use anchor_lang::system_program;  
use anchor_spl::associated_token::AssociatedToken;  
use anchor_spl::token::{self, CloseAccount as SplCloseAccount};  
use anchor_spl::token_interface::{  
    burn, close_account as token_2022_close_account, transfer_checked, Burn, Mint,  
    TokenAccount as InterfaceTokenAccount, TokenInterface, TransferChecked,  
};  
use spl_token_2022::extension::StateWithExtensions;  
use spl_token_2022::state::Account as Token2022Account;  
  
#[cfg(not(feature = "no-entrypoint"))]
solana_security_txt::security_txt! {
    name: "Claim SOL",
    project_url: "https://claimsol.pro",
    contacts: "email:info@claimsol.pro,twitter:@ClaimSol_",
    policy: "https://claimsol.pro/license",
    preferred_languages: "en",
    source_code: "https://github.com/ClaimSol/Claim-Sol",
    source_release: "v1.0.0",
    auditors: "None"
}

declare_id!("CLaim2U9C1AYg1APkb16PdxFVgeqSwzT9HDVESHhAxTt");

pub const AIRDROP_AUTHORITY_SEED: &[u8] = b"airdrop_authority";

#[program]
pub mod claim_sol {
    use super::*;

    use crate::{AffiliatePaidEvent, AirdropClaimedEvent, ClaimEvent, DevPaidEvent};
    // ──────────────────────────────────────────────────────────────
    // ORIGINAL INITIALIZE_CONFIG + snapshot slots added
    // ──────────────────────────────────────────────────────────────
    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        reward_per_account: u64,
        burn_per_account: u64,
        dev_fee_bps: u16,
        affiliate_fee_bps: u16,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.authority = ctx.accounts.user.key();
        config.total_sol_claimed = 0;
        config.total_claimed_accounts = 0;
        config.reward_per_account = reward_per_account;
        config.burn_per_account = burn_per_account;
        config.dev_fee_bps = dev_fee_bps;
        config.affiliate_fee_bps = affiliate_fee_bps;
        config.genesis_snapshot_slot = 0;
        config.titan_snapshot_slot = 0;

        msg!("CONFIG INITIALIZED!");
        msg!("REWARD PER ACCOUNT: {} $CLAIM", reward_per_account);
        msg!("BURN PER ACCOUNT: {} $CLAIM", burn_per_account);
        msg!(
            "DEV FEE: {} bps ({}%)",
            dev_fee_bps,
            dev_fee_bps as f64 / 100.0
        );
        msg!(
            "AFFILIATE FEE: {} bps ({}%)",
            affiliate_fee_bps,
            affiliate_fee_bps as f64 / 100.0
        );
        Ok(())
    }

    pub fn update_config(
        ctx: Context<UpdateConfig>,
        reward_per_account: u64,
        burn_per_account: u64,
        dev_fee_bps: u16,
        affiliate_fee_bps: u16,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        require!(
            ctx.accounts.authority.key() == config.authority,
            ClaimError::Unauthorized
        );
        config.reward_per_account = reward_per_account;
        config.burn_per_account = burn_per_account;
        config.dev_fee_bps = dev_fee_bps;
        config.affiliate_fee_bps = affiliate_fee_bps;
        msg!("CONFIG UPDATED!");
        Ok(())
    }

    pub fn reset_config(ctx: Context<ResetConfig>) -> Result<()> {
        let config = &mut ctx.accounts.config;
        require!(
            ctx.accounts.user.key() == config.authority,
            ClaimError::Unauthorized
        );
        config.total_sol_claimed = 0;
        config.total_claimed_accounts = 0;
        msg!("CONFIG COUNTERS RESET!");
        Ok(())
    }

    pub fn initialize_affiliate_stats(ctx: Context<InitializeAffiliateStats>) -> Result<()> {
        let stats = &mut ctx.accounts.affiliate_stats.load_init()?;
        stats.wallet = ctx.accounts.affiliate_wallet.key();
        stats.total_sol_earned = 0;
        stats.total_referrals = 0;
        stats.total_claimed_accounts = 0;
        stats.last_update_timestamp = Clock::get()?.unix_timestamp;
        msg!(
            "AFFILIATE STATS INITIALIZED FOR {}!",
            ctx.accounts.affiliate_wallet.key()
        );
        Ok(())
    }

    // ──────────────────────────────────────────────────────────────
    // YOUR ORIGINAL CLAIM FUNCTION – 100% UNCHANGED
    // ──────────────────────────────────────────────────────────────
    pub fn claim<'info>(ctx: Context<'_, '_, '_, 'info, Claim<'info>>) -> Result<()> {
        let config = &mut ctx.accounts.config;
        let user = &ctx.accounts.user;
        let dev_wallet = &ctx.accounts.dev_wallet;
        let token_program = &ctx.accounts.token_program;
        let token_2022 = &ctx.accounts.token_2022_program;
        let system_program = &ctx.accounts.system_program;
        let mint = &ctx.accounts.mint;
        let user_ata = &ctx.accounts.user_ata;
        let rewards_vault = &ctx.accounts.rewards_vault;

        let has_affiliate =
            ctx.accounts.affiliate_wallet.is_some() && ctx.accounts.affiliate_stats.is_some();
        let affiliate_wallet =
            has_affiliate.then(|| ctx.accounts.affiliate_wallet.as_ref().unwrap());
        let affiliate_stats = has_affiliate.then(|| ctx.accounts.affiliate_stats.as_mut().unwrap());

        let token_accounts = &ctx.remaining_accounts;
        let closed_count: u64 = token_accounts.len() as u64;

        require_gt!(closed_count, 0, ClaimError::NoAccountsClosed);
        require!(closed_count <= 25, ClaimError::TooManyAccounts);

        msg!("STARTING CLAIM FOR {} ACCOUNTS!", closed_count);

        let mut total_rent_lamports: u64 = 0;
        for (i, acc) in token_accounts.iter().enumerate() {
            let is_spl = *acc.owner == token_program.key();
            let is_2022 = *acc.owner == token_2022.key();
            require!(is_spl || is_2022, ClaimError::InvalidTokenProgram);

            let (amount, owner) = if is_spl {
                let token_acc = InterfaceTokenAccount::try_deserialize(&mut &acc.data.borrow()[..])
                    .map_err(|_| {
                        msg!("FAILED TO DESERIALIZE SPL TOKEN ACCOUNT #{}", i + 1);
                        ClaimError::InvalidAccount
                    })?;
                (token_acc.amount, token_acc.owner)
            } else {
                let data = acc.data.borrow();
                let state =
                    StateWithExtensions::<Token2022Account>::unpack(&data).map_err(|_| {
                        msg!("FAILED TO DESERIALIZE TOKEN-2022 ACCOUNT #{}", i + 1);
                        ClaimError::InvalidAccount
                    })?;
                (state.base.amount, state.base.owner)
            };

            require_eq!(amount, 0, ClaimError::NonZeroBalance);
            require_keys_eq!(owner, *user.key, ClaimError::InvalidOwner);

            total_rent_lamports = total_rent_lamports
                .checked_add(acc.lamports())
                .ok_or(ClaimError::CalculationError)?;

            if is_spl {
                let cpi = SplCloseAccount {
                    account: acc.clone(),
                    destination: user.to_account_info(),
                    authority: user.to_account_info(),
                };
                token::close_account(CpiContext::new(token_program.to_account_info(), cpi))
                    .map_err(|_| ClaimError::CloseFailed)?;
            } else {
                let cpi = anchor_spl::token_interface::CloseAccount {
                    account: acc.clone(),
                    destination: user.to_account_info(),
                    authority: user.to_account_info(),
                };
                token_2022_close_account(CpiContext::new(token_2022.to_account_info(), cpi))
                    .map_err(|_| ClaimError::CloseFailed)?;
            }

            msg!(
                "Closed account #{} – reclaimed {} lamports",
                i + 1,
                acc.lamports()
            );
        }

        let dev_fee_bps = if has_affiliate {
            config.dev_fee_bps
        } else {
            config.dev_fee_bps + config.affiliate_fee_bps
        };
        let affiliate_fee_bps = if has_affiliate {
            config.affiliate_fee_bps
        } else {
            0
        };

        let dev_fee = total_rent_lamports
            .checked_mul(dev_fee_bps as u64)
            .ok_or(ClaimError::CalculationError)?
            / 10_000;
        let affiliate_fee = total_rent_lamports
            .checked_mul(affiliate_fee_bps as u64)
            .ok_or(ClaimError::CalculationError)?
            / 10_000;
        let user_net = total_rent_lamports
            .checked_sub(dev_fee)
            .ok_or(ClaimError::CalculationError)?
            .checked_sub(affiliate_fee)
            .ok_or(ClaimError::CalculationError)?;

        msg!(
            "FEE SPLIT → Dev: {} lamports, Affiliate: {} lamports, User net: {} lamports",
            dev_fee,
            affiliate_fee,
            user_net
        );

        if dev_fee > 0 {
            system_program::transfer(
                CpiContext::new(
                    system_program.to_account_info(),
                    system_program::Transfer {
                        from: user.to_account_info(),
                        to: dev_wallet.to_account_info(),
                    },
                ),
                dev_fee,
            ).map_err(|_| {
                msg!("FAILED TO PAY DEV {} lamports", dev_fee);
                ClaimError::TransferFailed
            })?;
            msg!("DEV PAID {} lamports", dev_fee);
            emit!(DevPaidEvent {
                amount: dev_fee,
                recipient: dev_wallet.key()
            });
        }

        if has_affiliate && affiliate_fee > 0 {
            system_program::transfer(
                CpiContext::new(
                    system_program.to_account_info(),
                    system_program::Transfer {
                        from: user.to_account_info(),
                        to: affiliate_wallet.unwrap().to_account_info(),
                    },
                ),
                affiliate_fee,
            ).map_err(|_| {
                msg!("FAILED TO PAY AFFILIATE {} lamports", affiliate_fee);
                ClaimError::TransferFailed
            })?;
            msg!("AFFILIATE PAID {} lamports", affiliate_fee);
            emit!(AffiliatePaidEvent {
                amount: affiliate_fee,
                recipient: affiliate_wallet.unwrap().key()
            });
        }

        let reward_amount = closed_count
            .checked_mul(config.reward_per_account)
            .ok_or(ClaimError::CalculationError)?;
        let burn_amount = closed_count
            .checked_mul(config.burn_per_account)
            .ok_or(ClaimError::CalculationError)?;

        let seeds = &[b"config_v2".as_ref(), &[ctx.bumps.config]];
        let signer = &[&seeds[..]];

        transfer_checked(
            CpiContext::new_with_signer(
                token_2022.to_account_info(),
                TransferChecked {
                    from: rewards_vault.to_account_info(),
                    mint: mint.to_account_info(),
                    to: user_ata.to_account_info(),
                    authority: config.to_account_info(),
                },
                signer,
            ),
            reward_amount,
            mint.decimals,
        ).map_err(|_| {
            msg!("FAILED TO SEND REWARD {} $CLAIM", reward_amount);
            ClaimError::TokenTransferFailed
        })?;
        msg!("REWARD SENT: {} $CLAIM", reward_amount);

        burn(
            CpiContext::new_with_signer(
                token_2022.to_account_info(),
                Burn {
                    mint: mint.to_account_info(),
                    from: rewards_vault.to_account_info(),
                    authority: config.to_account_info(),
                },
                signer,
            ),
            burn_amount,
        ).map_err(|_| {
            msg!("FAILED TO BURN {} $CLAIM", burn_amount);
            ClaimError::TokenBurnFailed
        })?;
        msg!("BURNED: {} $CLAIM", burn_amount);

        if let Some(stats) = affiliate_stats {
            let mut s = stats.load_mut().map_err(|_| {
                msg!("FAILED TO LOAD AFFILIATE STATS PDA");
                ClaimError::PdaLoadFailed
            })?;
            s.total_sol_earned = s
                .total_sol_earned
                .checked_add(affiliate_fee)
                .ok_or(ClaimError::CalculationError)?;
            s.total_referrals = s
                .total_referrals
                .checked_add(1)
                .ok_or(ClaimError::CalculationError)?;
            s.total_claimed_accounts = s
                .total_claimed_accounts
                .checked_add(closed_count)
                .ok_or(ClaimError::CalculationError)?;
            s.last_update_timestamp = Clock::get()?.unix_timestamp;
        }

        config.total_sol_claimed = config
            .total_sol_claimed
            .checked_add(total_rent_lamports)
            .ok_or(ClaimError::CalculationError)?;
        config.total_claimed_accounts = config
            .total_claimed_accounts
            .checked_add(closed_count)
            .ok_or(ClaimError::CalculationError)?;

        emit!(ClaimEvent {
            user: user.key(),
            total_claimed: total_rent_lamports,
            accounts_closed: closed_count,
            dev_fee,
            affiliate_fee,
        });

        msg!(
            "CLAIM COMPLETED! Total reclaimed: {} lamports, User net: {} lamports + {} $CLAIM",
            total_rent_lamports,
            user_net,
            reward_amount
        );
        Ok(())
    }

    // ──────────────────────────────────────────────────────────────
    // NEW: AIRDROP FEATURES (secure & minimal)
    // ──────────────────────────────────────────────────────────────
    pub fn snapshot_genesis(ctx: Context<Snapshot>) -> Result<()> {
        ctx.accounts.config.genesis_snapshot_slot = Clock::get()?.slot;
        Ok(())
    }

    pub fn snapshot_titan(ctx: Context<Snapshot>) -> Result<()> {
        ctx.accounts.config.titan_snapshot_slot = Clock::get()?.slot;
        Ok(())
    }

    pub fn initialize_genesis_rank(ctx: Context<InitializeGenesisRank>, rank: u64) -> Result<()> {
        let p = &mut ctx.accounts.rank_proof;
        p.rank = rank;
        p.snapshot_slot = Clock::get()?.slot;
        p.claimed = false;
        Ok(())
    }

    pub fn initialize_titan_rank(ctx: Context<InitializeTitanRank>, rank: u64) -> Result<()> {
        let p = &mut ctx.accounts.rank_proof;
        p.rank = rank;
        p.snapshot_slot = Clock::get()?.slot;
        p.claimed = false;
        Ok(())
    }

    pub fn claim_genesis_drop(ctx: Context<ClaimGenesis>) -> Result<()> {
        let config = &ctx.accounts.config;
        require!(config.genesis_snapshot_slot > 0, ClaimError::NotUnlocked);
        let proof = &mut ctx.accounts.rank_proof;
        require_eq!(
            proof.snapshot_slot,
            config.genesis_snapshot_slot,
            ClaimError::InvalidSnapshot
        );
        require!(!proof.claimed, ClaimError::AlreadyClaimed);
        require!(proof.rank <= 100, ClaimError::NotEligible);

        let amount = match proof.rank {
            1 => 3_500_000,
            2 => 3_000_000,
            3 => 2_500_000,
            4 => 1_800_000,
            5 => 1_200_000,
            6..=10 => 800_000,
            11..=20 => 400_000,
            21..=50 => 150_000,
            _ => 75_000,
        } * 1_000_000_000;

        let seeds = &[AIRDROP_AUTHORITY_SEED, &[ctx.bumps.airdrop_authority]];
        transfer_checked(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.airdrop_treasury_ata.to_account_info(),
                    to: ctx.accounts.user_ata.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    authority: ctx.accounts.airdrop_authority.to_account_info(),
                },
                &[&seeds[..]],
            ),
            amount,
            9,
        )?;
        proof.claimed = true;
        emit!(AirdropClaimedEvent {
            user: ctx.accounts.user.key(),
            kind: 1,
            rank: proof.rank,
            amount
        });
        Ok(())
    }

    pub fn claim_titan_drop(ctx: Context<ClaimTitan>) -> Result<()> {
        let config = &ctx.accounts.config;
        require!(config.titan_snapshot_slot > 0, ClaimError::NotUnlocked);
        let proof = &mut ctx.accounts.rank_proof;
        require_eq!(
            proof.snapshot_slot,
            config.titan_snapshot_slot,
            ClaimError::InvalidSnapshot
        );
        require!(!proof.claimed, ClaimError::AlreadyClaimed);
        require!(proof.rank <= 1000, ClaimError::NotEligible);

        let amount = match proof.rank {
            1..=5 => 1_500_000,
            6..=10 => 1_000_000,
            11..=20 => 500_000,
            21..=50 => 300_000,
            51..=100 => 200_000,
            101..=250 => 100_000,
            251..=500 => 50_000,
            501..=750 => 25_000,
            _ => 20_000,
        } * 1_000_000_000;

        let seeds = &[AIRDROP_AUTHORITY_SEED, &[ctx.bumps.airdrop_authority]];
        transfer_checked(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.airdrop_treasury_ata.to_account_info(),
                    to: ctx.accounts.user_ata.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    authority: ctx.accounts.airdrop_authority.to_account_info(),
                },
                &[&seeds[..]],
            ),
            amount,
            9,
        )?;
        proof.claimed = true;
        emit!(AirdropClaimedEvent {
            user: ctx.accounts.user.key(),
            kind: 2,
            rank: proof.rank,
            amount
        });
        Ok(())
    }
}

// ──────────────────────────────────────────────────────────────
// ACCOUNT STRUCTS — FINAL FIXED VERSION (mint is now mutable!)
// ──────────────────────────────────────────────────────────────
#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(init, payer = user, space = 8 + 32 + 8 + 8 + 8 + 8 + 2 + 2 + 8 + 8, seeds = [b"config_v2"], bump)]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut, seeds = [b"config_v2"], bump, has_one = authority)]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ResetConfig<'info> {
    #[account(mut, seeds = [b"config_v2"], bump)]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct InitializeAffiliateStats<'info> {
    #[account(init, payer = user, space = 8 + 32 + 8 + 8 + 8 + 8, seeds = [b"affiliate", affiliate_wallet.key().as_ref()], bump)]
    pub affiliate_stats: AccountLoader<'info, AffiliateStats>,
    pub affiliate_wallet: AccountInfo<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// ──────────────────────────────────────────────────────────────
// ──────────────────────────────────────────────────────────────
// CLAIM — FIXED: REMOVE AUTHORITY CONSTRAINT (THIS WAS THE BUG)
// ──────────────────────────────────────────────────────────────
#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub dev_wallet: AccountInfo<'info>,
    #[account(mut)]
    pub affiliate_stats: Option<AccountLoader<'info, AffiliateStats>>,
    #[account(mut, seeds = [b"config_v2"], bump)]
    pub config: Account<'info, Config>,
    pub token_program: Program<'info, token::Token>,
    pub token_2022_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub affiliate_wallet: Option<AccountInfo<'info>>,
    /// CHECK: sysvar
    pub instructions: UncheckedAccount<'info>,

    // FIXED: NO mint::authority — ONLY mut
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub user_ata: InterfaceAccount<'info, InterfaceTokenAccount>,
    #[account(mut)]
    pub rewards_vault: InterfaceAccount<'info, InterfaceTokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

// ──────────────────────────────────────────────────────────────
// AIRDROP ACCOUNTS — ALSO FIXED (optional but recommended)
// ──────────────────────────────────────────────────────────────
#[derive(Accounts)]
pub struct Snapshot<'info> {
    #[account(mut, seeds = [b"config_v2"], bump, has_one = authority)]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct InitializeGenesisRank<'info> {
    #[account(init, payer = authority, space = 8 + 8 + 8 + 1, seeds = [b"genesis_rank", user_wallet.key().as_ref()], bump)]
    pub rank_proof: Account<'info, RankProof>,
    /// CHECK
    pub user_wallet: UncheckedAccount<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeTitanRank<'info> {
    #[account(init, payer = authority, space = 8 + 8 + 8 + 1, seeds = [b"titan_rank", user_wallet.key().as_ref()], bump)]
    pub rank_proof: Account<'info, RankProof>,
    /// CHECK
    pub user_wallet: UncheckedAccount<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimGenesis<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(seeds = [b"config_v2"], bump)]
    pub config: Account<'info, Config>,
    #[account(seeds = [AIRDROP_AUTHORITY_SEED], bump)]
    pub airdrop_authority: SystemAccount<'info>,
    #[account(mut)]
    pub airdrop_treasury_ata: InterfaceAccount<'info, InterfaceTokenAccount>,
    #[account(mut)]
    pub user_ata: InterfaceAccount<'info, InterfaceTokenAccount>,
    #[account(mut)] // ← Also safe to add here if you ever mint more
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    #[account(mut, seeds = [b"genesis_rank", user.key().as_ref()], bump)]
    pub rank_proof: Account<'info, RankProof>,
}

#[derive(Accounts)]
pub struct ClaimTitan<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(seeds = [b"config_v2"], bump)]
    pub config: Account<'info, Config>,
    #[account(seeds = [AIRDROP_AUTHORITY_SEED], bump)]
    pub airdrop_authority: SystemAccount<'info>,
    #[account(mut)]
    pub airdrop_treasury_ata: InterfaceAccount<'info, InterfaceTokenAccount>,
    #[account(mut)]
    pub user_ata: InterfaceAccount<'info, InterfaceTokenAccount>,
    #[account(mut)] // ← Same here
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    #[account(mut, seeds = [b"titan_rank", user.key().as_ref()], bump)]
    pub rank_proof: Account<'info, RankProof>,
}
// ──────────────────────────────────────────────────────────────
// STATE + EVENTS + ERRORS
// ──────────────────────────────────────────────────────────────
#[account]
#[derive(Default)]
pub struct Config {
    pub authority: Pubkey,
    pub total_sol_claimed: u64,
    pub total_claimed_accounts: u64,
    pub reward_per_account: u64,
    pub burn_per_account: u64,
    pub dev_fee_bps: u16,
    pub affiliate_fee_bps: u16,
    pub genesis_snapshot_slot: u64,
    pub titan_snapshot_slot: u64,
}

#[account(zero_copy)]
pub struct AffiliateStats {
    pub wallet: Pubkey,
    pub total_sol_earned: u64,
    pub total_referrals: u64,
    pub total_claimed_accounts: u64,
    pub last_update_timestamp: i64,
}

#[account]
pub struct RankProof {
    pub rank: u64,
    pub snapshot_slot: u64,
    pub claimed: bool,
}

#[event]
pub struct ClaimEvent {
    pub user: Pubkey,
    pub total_claimed: u64,
    pub accounts_closed: u64,
    pub dev_fee: u64,
    pub affiliate_fee: u64,
}

#[event]
pub struct DevPaidEvent {
    pub amount: u64,
    pub recipient: Pubkey,
}

#[event]
pub struct AffiliatePaidEvent {
    pub amount: u64,
    pub recipient: Pubkey,
}

#[event]
pub struct AirdropClaimedEvent {
    pub user: Pubkey,
    pub kind: u8,
    pub rank: u64,
    pub amount: u64,
}

#[error_code]
pub enum ClaimError {
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Too many accounts provided")]
    TooManyAccounts,
    #[msg("No accounts were closed")]
    NoAccountsClosed,
    #[msg("Account has non-zero balance")]
    NonZeroBalance,
    #[msg("Invalid owner for token account")]
    InvalidOwner,
    #[msg("Invalid token program")]
    InvalidTokenProgram,
    #[msg("Calculation error")]
    CalculationError,
    #[msg("Failed to close token account")]
    CloseFailed,
    #[msg("Token transfer failed")]
    TokenTransferFailed,
    #[msg("Token burn failed")]
    TokenBurnFailed,
    #[msg("Failed to load PDA")]
    PdaLoadFailed,
    #[msg("Transfer failed")]
    TransferFailed,
    #[msg("Invalid account")]
    InvalidAccount,
    #[msg("Airdrop not unlocked")]
    NotUnlocked,
    #[msg("Already claimed")]
    AlreadyClaimed,
    #[msg("Not eligible")]
    NotEligible,
    #[msg("Invalid snapshot")]
    InvalidSnapshot,
}
