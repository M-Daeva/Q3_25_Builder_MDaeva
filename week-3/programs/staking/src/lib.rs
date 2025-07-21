#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod math;
pub mod state;

use instructions::{claim::*, init::*, stake::*, unstake::*};

declare_id!("FS3fX9yzYMkurJPXsPeFY1mt8pCQtbTQSDQqpR61wkhb");

#[program]
pub mod staking {
    use super::*;

    pub fn init(
        ctx: Context<Init>,
        rewards_rate: u8,
        max_stake: u64,
        collection: Pubkey,
    ) -> Result<()> {
        ctx.accounts.init(
            ctx.bumps.config,
            ctx.bumps.rewards_mint,
            rewards_rate,
            max_stake,
            collection,
        )
    }

    pub fn stake(ctx: Context<Stake>, token_id: u16) -> Result<()> {
        ctx.accounts.stake(token_id)
    }

    pub fn unstake(ctx: Context<Unstake>, token_id: u16) -> Result<()> {
        ctx.accounts.unstake(token_id)
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        ctx.accounts.claim()
    }
}
