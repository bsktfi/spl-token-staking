use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::state::StakePool;

#[derive(Accounts)]
#[instruction(nonce: u8, digit_shift: i8)]
pub struct InitializeStakePool<'info> {
    /// Payer and authority of the StakePool
    #[account(mut)]
    pub authority: Signer<'info>,

    /// SPL Token Mint of the underlying token to be deposited for staking
    pub mint: Account<'info, Mint>,

    #[account(
      init,
      seeds = [
        &nonce.to_le_bytes(),
        authority.key().as_ref(),
        b"stakePool",
      ],
      bump,
      payer = authority,
      space = 8 + StakePool::LEN,
    )]
    pub stake_pool: Account<'info, StakePool>,

    /// An SPL token Mint for the effective stake weight token
    #[account(
      init,
      seeds = [&stake_pool.key().to_bytes()[..], b"stakeMint"],
      bump,
      payer = authority,
      mint::decimals = mint.decimals, // TODO account for potential precision errors with digit shift
      mint::authority = stake_pool,
    )]
    pub stake_mint: Account<'info, Mint>,

    /// An SPL token Account for staging A tokens
    #[account(
      init,
      seeds = [&stake_pool.key().to_bytes()[..], b"vault"],
      bump,
      payer = authority,
      token::mint = mint,
      token::authority = stake_pool,
    )]
    pub vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeStakePool>, nonce: u8, digit_shift: i8) -> Result<()> {
    let stake_pool = &mut ctx.accounts.stake_pool;
    stake_pool.authority = ctx.accounts.authority.key();
    stake_pool.stake_mint = ctx.accounts.stake_mint.key();
    stake_pool.vault = ctx.accounts.vault.key();
    stake_pool.digit_shift = digit_shift;
    stake_pool.nonce = nonce;
    stake_pool.bump_seed = *ctx.bumps.get("stake_pool").unwrap();
    Ok(())
}
