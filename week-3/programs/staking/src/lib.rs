#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

pub mod error;
pub mod helpers;
pub mod instructions;
pub mod math;
pub mod state;

use instructions::{claim::*, init::*, stake::*};

declare_id!("8Y1PPAsKbeKiT361EbKeCrU9yE1bNLXWNnM7va2PMQ67");

#[program]
pub mod staking {
    use super::*;

    pub fn init(ctx: Context<Init>, rewards_rate: u8, max_stake: u64) -> Result<()> {
        ctx.accounts.init(
            ctx.bumps.config,
            ctx.bumps.rewards_mint,
            rewards_rate,
            max_stake,
        )
    }

    pub fn stake(ctx: Context<Stake>, tokens: Vec<u16>) -> Result<()> {
        ctx.accounts.stake(tokens)
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        ctx.accounts.claim()
    }
}

// TODO: unstake
